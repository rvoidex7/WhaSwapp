import { useState } from 'react';
import { updateSettings } from '../services/ipc';

export default function LLMSettings() {
    const [model, setModel] = useState('gemini-nano');

    const handleSave = () => {
        updateSettings({ llmModel: model });
    };

    return (
        <div className="p-4 bg-white rounded shadow">
            <h3 className="font-bold mb-2">AI Model Settings</h3>
            <select
                value={model}
                onChange={(e) => setModel(e.target.value)}
                className="w-full p-2 border rounded mb-4"
            >
                <option value="gemini-nano">Gemini Nano (Local)</option>
                <option value="gpt-4">GPT-4 (Cloud)</option>
            </select>
            <button onClick={handleSave} className="bg-blue-500 text-white px-4 py-2 rounded">
                Save
            </button>
        </div>
    );
}
