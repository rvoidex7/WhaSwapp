import LLMSettings from './LLMSettings';
import { useAuthStore } from '../stores/authStore';

export default function SettingsPanel() {
    const { user } = useAuthStore();

    return (
        <div className="h-full bg-gray-50 p-6">
            <h1 className="text-2xl font-bold mb-6">Settings</h1>

            <div className="mb-8">
                <h2 className="text-lg font-semibold mb-4">Account</h2>
                <div className="bg-white p-4 rounded shadow">
                    <p>Status: {user ? 'Connected' : 'Disconnected'}</p>
                    <p>User: {user?.name || 'Unknown'}</p>
                </div>
            </div>

            <div className="mb-8">
                <h2 className="text-lg font-semibold mb-4">AI & Intelligence</h2>
                <LLMSettings />
            </div>
        </div>
    );
}
