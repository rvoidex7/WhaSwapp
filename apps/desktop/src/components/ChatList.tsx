import { useChatStore } from '../stores/chatStore';

export default function ChatList() {
    const chats = useChatStore(state => state.chats);
    const activeChat = useChatStore(state => state.activeChat);
    const setActiveChat = useChatStore(state => state.setActiveChat);

    return (
        <div className="flex flex-col h-full bg-white border-r border-gray-200">
            <div className="p-3 bg-gray-100 border-b border-gray-200 flex justify-between items-center">
                <div className="w-8 h-8 rounded-full bg-gray-300"></div>
                <div className="flex gap-2 text-gray-600">
                    <button className="p-1 hover:bg-gray-200 rounded-full">
                        <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M19.005 3.175H4.674C3.642 3.175 3 3.789 3 4.821V21.02l3.544-3.514h12.461c1.033 0 2.064-1.06 2.064-2.093V4.821c-.001-1.032-1.032-1.646-2.064-1.646zm-4.989 9.869H6.666V12.06h7.349v.984zm3.063-2.659H6.666V9.4h10.413v.984zm0-2.663H6.666V6.738h10.413v.984z"></path></svg>
                    </button>
                    <button className="p-1 hover:bg-gray-200 rounded-full">
                         <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 7a2 2 0 1 0-.001-4.001A2 2 0 0 0 12 7zm0 2a2 2 0 1 0-.001 3.999A2 2 0 0 0 12 9zm0 6a2 2 0 1 0-.001 3.999A2 2 0 0 0 12 15z"></path></svg>
                    </button>
                </div>
            </div>
            <div className="p-2 bg-white border-b border-gray-200">
                <div className="bg-gray-100 rounded-lg flex items-center px-2 py-1">
                     <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor" className="text-gray-500 mr-2"><path d="M15.009 13.805h-.636l-.22-.219a5.184 5.184 0 0 0 1.256-3.386 5.207 5.207 0 1 0-5.207 5.208 5.183 5.183 0 0 0 3.385-1.254l.221.22v.635l4.004 3.999 1.194-1.195-3.997-4.008zm-4.792 0a3.58 3.58 0 0 1-3.578-3.578 3.58 3.58 0 0 1 3.578-3.579 3.58 3.58 0 0 1 3.578 3.579 3.58 3.58 0 0 1-3.578 3.578z"></path></svg>
                    <input type="text" placeholder="Search or start new chat" className="bg-transparent w-full text-sm outline-none py-1" />
                </div>
            </div>

            <div className="overflow-y-auto flex-1">
                {chats.length === 0 && (
                     <div className="p-4 text-center text-gray-500 text-sm">
                         No active chats.<br/>
                         Scan QR to sync messages.
                     </div>
                )}
                {chats.map(chat => (
                    <div
                        key={chat.id}
                        onClick={() => setActiveChat(chat.id)}
                        className={`p-3 border-b border-gray-100 hover:bg-gray-50 cursor-pointer flex gap-3 ${activeChat === chat.id ? 'bg-gray-100' : ''}`}
                    >
                        <div className="w-12 h-12 rounded-full bg-gray-300 flex-shrink-0">
                            {/* Placeholder for avatar */}
                        </div>
                        <div className="flex-1 min-w-0">
                            <div className="flex justify-between items-baseline">
                                <h3 className="text-gray-900 font-medium truncate">{chat.name || chat.id}</h3>
                                <span className="text-xs text-gray-500">
                                    {/* Date formatter would go here */}
                                    12:00
                                </span>
                            </div>
                            <p className="text-sm text-gray-500 truncate">{chat.lastMessage || 'No messages'}</p>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
