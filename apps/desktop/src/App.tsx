import { useEffect } from 'react';
import { useAuthStore } from './stores/authStore';
import { initIPC } from './services/ipc';
import { invoke } from '@tauri-apps/api/core';
import { QRCodeSVG } from 'qrcode.react';
import ChatList from './components/ChatList';
import ChatWindow from './components/ChatWindow';
import Terminal from './components/Terminal';

function App() {
  useEffect(() => {
    initIPC();
  }, []);

  const { status, qrCode } = useAuthStore();

  const handleResetSession = async () => {
    try {
      await invoke('reset_session');
      // Session temizlendi, yeniden bağlanmayı dene
      window.location.reload();
    } catch (error) {
      console.error('Failed to reset session:', error);
    }
  };

  if (status === 'disconnected') {
    return (
      <div className="h-screen flex overflow-hidden">
        <div className="flex-1 flex items-center justify-center bg-gray-100">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-green-500 mx-auto mb-4"></div>
            <p className="text-gray-700 mb-4">Connecting to service...</p>
            <button
              onClick={handleResetSession}
              className="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors"
            >
              🔄 Reset Session
            </button>
            <p className="text-xs text-gray-500 mt-2">
              Click if stuck or connection fails
            </p>
          </div>
        </div>
        <div className="w-1/3 bg-black">
          <Terminal />
        </div>
      </div>
    );
  }

  if (status === 'qr' && qrCode) {
    return (
      <div className="h-screen flex overflow-hidden">
        <div className="flex-1 flex flex-col items-center justify-center bg-gray-100">
          <div className="bg-white p-8 rounded-lg shadow-lg text-center">
            <h1 className="text-2xl mb-4 font-bold text-gray-800">Scan QR Code</h1>
            <div className="bg-white p-2 inline-block">
              <QRCodeSVG value={qrCode} size={256} />
            </div>
            <p className="mt-4 text-gray-600">Open WhatsApp on your phone and scan</p>
          </div>
        </div>
        <div className="w-1/3 bg-black">
          <Terminal />
        </div>
      </div>
    );
  }

  if (status === 'connected') {
    return (
      <div className="h-screen flex overflow-hidden">
        <div className="w-1/4 border-r bg-white">
           <ChatList />
        </div>
        <div className="flex-1 bg-gray-50">
           <ChatWindow />
        </div>
        <div className="w-1/3 bg-black">
           <Terminal />
        </div>
      </div>
    );
  }

  if (status === 'connecting') {
    return (
      <div className="h-screen flex overflow-hidden">
        <div className="flex-1 flex flex-col items-center justify-center bg-gray-100">
          <div className="bg-white p-8 rounded-lg shadow-lg text-center">
            <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-green-500 mx-auto mb-4"></div>
            <h2 className="text-xl font-semibold text-gray-800">Connecting to WhatsApp...</h2>
            <p className="text-gray-600 mt-2">Please wait</p>
          </div>
        </div>
        <div className="w-1/3 bg-black">
          <Terminal />
        </div>
      </div>
    );
  }

  // Default fallback (e.g. disconnected state)
  return <div className="h-screen flex items-center justify-center">Initializing...</div>;
}

export default App;
