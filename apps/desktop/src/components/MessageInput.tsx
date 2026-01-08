import { useState } from 'react';
import { generateDraft, sendText } from '../services/ipc';

interface Props {
    jid: string;
}

export default function MessageInput({ jid }: Props) {
    const [text, setText] = useState('');

    const handleSend = () => {
        if (!text.trim()) return;
        sendText(jid, text);
        setText('');
    };

    const handleGenerate = async () => {
        const draft = await generateDraft(jid);
        if (draft) setText(draft);
    };

    return (
        <div className="flex gap-2 p-2 bg-gray-100">
            <button onClick={handleGenerate} className="text-purple-600 p-2" title="AI Draft">
                ✨
            </button>
            <input
                value={text}
                onChange={(e) => setText(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                className="flex-1 p-2 border rounded"
                placeholder="Type a message..."
            />
            <button onClick={handleSend} className="bg-green-500 text-white px-4 rounded">
                Send
            </button>
        </div>
    );
}
