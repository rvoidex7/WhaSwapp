import makeWASocket, {
    DisconnectReason,
    fetchLatestBaileysVersion,
    makeCacheableSignalKeyStore
} from '@whiskeysockets/baileys';
import { Boom } from '@hapi/boom';
import pino from 'pino';
import readline from 'readline';

// Simple Logger
const logger = pino({ level: 'silent' });

// State
let sock = null;

// IPC Helper
function sendEvent(type, payload) {
    console.log(JSON.stringify({ type, payload }));
}

// In-Memory Auth Store with IPC Sync
const createIPCAuthState = (initialCreds, keysData) => {
    // 1. Creds (Session info)
    const creds = initialCreds || makeWASocket.initAuthCreds();

    // 2. Keys (Signal Protocol keys)
    const keys = keysData || {};

    const saveCreds = () => {
        sendEvent('auth_update', { type: 'creds', data: creds });
    };

    return {
        state: {
            creds,
            keys: makeCacheableSignalKeyStore({
                get: async (type, ids) => {
                    const data = {};
                    for (const id of ids) {
                        if (keys[type] && keys[type][id]) {
                            data[id] = keys[type][id];
                        }
                    }
                    return data;
                },
                set: async (data) => {
                    for (const type in data) {
                        if (!keys[type]) keys[type] = {};
                        for (const id in data[type]) {
                            const value = data[type][id];
                            if (value) {
                                keys[type][id] = value;
                            } else {
                                delete keys[type][id];
                            }
                        }
                    }
                    sendEvent('auth_update', { type: 'keys', data });
                }
            }, logger),
        },
        saveCreds
    };
};


// Command Handlers
async function handleInit(payload) {
    const initialCreds = payload.auth_data?.creds;
    const initialKeys = payload.auth_data?.keys;

    await startSock(initialCreds, initialKeys);
}

async function handleSendMessage(payload) {
    if (!sock) return;
    const { jid, content } = payload;
    try {
        await sock.sendMessage(jid, { text: content });
        sendEvent('ack', { status: 'sent', jid });
    } catch (e) {
        sendEvent('error', { message: e.message });
    }
}

async function handleDisconnect() {
    if (sock) {
        sock.end(undefined);
        sock = null;
    }
    process.exit(0); // Exit to let Rust restart us
}

// Main Socket Logic
async function startSock(initialCreds, initialKeys) {
    const { state, saveCreds } = createIPCAuthState(initialCreds, initialKeys);
    const { version, isLatest } = await fetchLatestBaileysVersion();

    sock = makeWASocket.default({
        version,
        logger,
        printQRInTerminal: false,
        auth: state,
        generateHighQualityLinkPreview: true,
    });

    sock.ev.on('connection.update', (update) => {
        const { connection, lastDisconnect, qr } = update;

        if (qr) {
            sendEvent('qr_code', qr);
        }

        if (connection === 'close') {
            const shouldReconnect = (lastDisconnect?.error instanceof Boom)?.output?.statusCode !== DisconnectReason.loggedOut;
            if (shouldReconnect) {
                // Do NOT recurse startSock here. State (keys) would be stale/lost.
                // Instead, notify Rust to restart the process.
                sendEvent('connection_status', 'disconnected_reconnecting');
                process.exit(0); // Rust backend should detect exit and restart if desired, or we rely on 'init' again.
            } else {
                 sendEvent('connection_status', 'disconnected');
                 sendEvent('auth_failure', 'logged_out');
                 process.exit(0);
            }
        } else if (connection === 'open') {
             sendEvent('connection_status', 'connected');
        }
    });

    sock.ev.on('creds.update', saveCreds);

    sock.ev.on('messages.upsert', async (m) => {
        if (m.type === 'notify') {
            for (const msg of m.messages) {
                if (!msg.key.fromMe) {
                    const jid = msg.key.remoteJid;
                    const name = msg.pushName || jid;
                    const content = msg.message?.conversation || msg.message?.extendedTextMessage?.text || '[Media/Other]';

                    sendEvent('message', {
                        jid,
                        name,
                        content,
                        timestamp: msg.messageTimestamp,
                        raw: msg
                    });
                }
            }
        }
    });
}

// Stdin Reader (IPC)
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

rl.on('line', async (line) => {
    try {
        if (!line.trim()) return;
        const command = JSON.parse(line);
        switch (command.type) {
            case 'init':
                await handleInit(command.payload);
                break;
            case 'send_message':
                await handleSendMessage(command.payload);
                break;
            case 'disconnect':
                await handleDisconnect();
                break;
            default:
                break;
        }
    } catch (e) {
    }
});

// Keep process alive
process.stdin.resume();
