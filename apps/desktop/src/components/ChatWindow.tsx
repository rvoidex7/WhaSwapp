import { useState, useRef, useEffect } from 'react';
import { sendText } from '../services/ipc';
import { useChatStore } from '../stores/chatStore';

export default function ChatWindow() {
    const [inputValue, setInputValue] = useState("");
    const messages = useChatStore(state => state.messages);
    const activeChat = useChatStore(state => state.activeChat);
    const chats = useChatStore(state => state.chats);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const activeChatData = chats.find(c => c.id === activeChat);

    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    useEffect(() => {
        scrollToBottom();
    }, [messages, activeChat]);

    // Filter messages for active chat
    const chatMessages = messages.filter(m => {
        if (!activeChat) return false;
        // Check both remoteJid and participant (if group)
        const remote = m.key.remoteJid;
        return remote === activeChat;
    });

    const handleSend = async () => {
        if (!inputValue.trim() || !activeChat) return;

        try {
             await sendText(activeChat, inputValue);
             setInputValue("");
        } catch (e) {
            console.error("Failed to send", e);
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    };

    if (!activeChat) {
        return (
            <div className="h-full flex flex-col items-center justify-center bg-[#efeae2] border-b-[6px] border-b-green-500">
                <div className="text-center text-gray-500">
                    <h1 className="text-3xl font-light mb-4">LiteWA</h1>
                    <p>Select a chat to start messaging</p>
                </div>
            </div>
        );
    }

    return (
        <div className="flex flex-col h-full bg-[#efeae2]">
            {/* Header */}
            <div className="bg-gray-100 p-3 flex justify-between items-center border-b border-gray-200">
                 <div className="flex items-center gap-3">
                     <div className="w-10 h-10 rounded-full bg-gray-300"></div>
                     <span className="font-medium text-gray-800">{activeChatData?.name || activeChat}</span>
                 </div>
                 <div className="flex gap-4 text-gray-600">
                     <button><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M15.9 14.3H15l-.3-.3c1-1.1 1.6-2.7 1.6-4.3 0-3.7-3-6.7-6.7-6.7S3 6 3 9.7s3 6.7 6.7 6.7c1.6 0 3.2-.6 4.3-1.6l.3.3v.9l5.1 5.1 1.5-1.5-5-5.1zm-6.2 0c-2.6 0-4.7-2.1-4.7-4.7s2.1-4.7 4.7-4.7 4.7 2.1 4.7 4.7-2.1 4.7-4.7 4.7z"></path></svg></button>
                     <button><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M12 7a2 2 0 1 0-.001-4.001A2 2 0 0 0 12 7zm0 2a2 2 0 1 0-.001 3.999A2 2 0 0 0 12 9zm0 6a2 2 0 1 0-.001 3.999A2 2 0 0 0 12 15z"></path></svg></button>
                 </div>
            </div>

            {/* Messages Area */}
            <div className="flex-1 overflow-y-auto p-4 space-y-2">
                {chatMessages.map((msg: any, i: number) => (
                    <div key={i} className={`flex ${msg.fromMe ? 'justify-end' : 'justify-start'}`}>
                        <div className={`max-w-[70%] p-2 rounded-lg shadow-sm ${msg.fromMe ? 'bg-[#d9fdd3]' : 'bg-white'}`}>
                            <p className="text-gray-800 text-sm leading-relaxed">{msg.content?.conversation || msg.content?.text || 'Media'}</p>
                            <span className="text-[10px] text-gray-500 float-right ml-2 mt-1">
                                {new Date((msg.timestamp || Date.now() / 1000) * 1000).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                            </span>
                        </div>
                    </div>
                ))}
                <div ref={messagesEndRef} />
            </div>

            {/* Input Area */}
            <div className="bg-gray-100 p-3 flex items-center gap-2">
                <button className="text-gray-500 p-2">
                    <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M9.153 11.603c.795 0 1.439-.879 1.439-1.962s-.644-1.962-1.439-1.962-1.439.879-1.439 1.962.644 1.962 1.439 1.962zm-3.204 1.362c-.026-.307-.131 5.218 6.063 5.551 6.066-.25 6.066-5.551 6.066-5.551-6.078-1.416-12.129 0-12.129 0zm11.363 1.108s-.669 1.959-5.051 1.959c-3.505 0-5.388-1.164-5.607-1.959 0 0 5.912 1.055 10.658 0zM11.804 1.011C5.609 1.011.978 6.033.978 12.228s4.826 10.761 11.021 10.761S23.02 18.423 23.02 12.228c.001-6.195-5.021-11.217-11.216-11.217zM12 21.354c-5.273 0-9.381-3.886-9.381-9.159s3.942-9.548 9.215-9.548 9.38 4.272 9.38 9.548c0 5.272-4.109 9.159-9.214 9.159zm3.196-9.751c.795 0 1.439-.879 1.439-1.962s-.644-1.962-1.439-1.962-1.439.879-1.439 1.962.644 1.962 1.439 1.962z"></path></svg>
                </button>
                <input
                    type="text"
                    value={inputValue}
                    onChange={(e) => setInputValue(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Type a message"
                    className="flex-1 p-2 rounded-lg border border-gray-300 outline-none focus:border-green-500"
                />
                <button onClick={handleSend} className="text-gray-500 p-2">
                   <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M1.101 21.757 23.8 12.028 1.101 2.3l.011 7.912 13.623 1.816-13.623 1.817-.011 7.912z"></path></svg>
                </button>
            </div>
        </div>
    );
}
