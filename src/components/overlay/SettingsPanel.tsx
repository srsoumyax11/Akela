import React, { useEffect } from 'react';
import { useSettingsStore } from '../../store/useSettingsStore';
import './SettingsPanel.css';

export const SettingsPanel: React.FC = () => {
  const { activeProvider, apiKeys, setActiveProvider, setApiKey, saveApiKeyToDb, loadApiKeys } = useSettingsStore();

  useEffect(() => {
    loadApiKeys();
  }, [loadApiKeys]);

  const handleBlur = (provider: 'groq' | 'openai' | 'mistral', key: string) => {
    saveApiKeyToDb(provider, key);
  };

  return (
    <div className="settings-panel">
      <div className="settings-header">
        <h3>Speech to Text Providers</h3>
      </div>
      
      <div className="settings-content">
        <div className="setting-group">
          <label>Active Provider</label>
          <select 
            value={activeProvider} 
            onChange={(e) => setActiveProvider(e.target.value as any)}
            className="provider-select"
          >
            <option value="groq">Groq (Whisper)</option>
            <option value="openai">OpenAI (Whisper)</option>
            <option value="mistral">Mistral</option>
          </select>
        </div>

        <div className="api-keys-section">
          <h4>API Keys (Saved securely via Rust later)</h4>
          
          <div className="key-input-group">
            <label>Groq API Key</label>
            <input 
              type="password" 
              placeholder="gsk_..."
              value={apiKeys.groq}
              onChange={(e) => setApiKey('groq', e.target.value)}
              onBlur={(e) => handleBlur('groq', e.target.value)}
            />
          </div>

          <div className="key-input-group">
            <label>OpenAI API Key</label>
            <input 
              type="password" 
              placeholder="sk-..."
              value={apiKeys.openai}
              onChange={(e) => setApiKey('openai', e.target.value)}
              onBlur={(e) => handleBlur('openai', e.target.value)}
            />
          </div>

          <div className="key-input-group">
            <label>Mistral API Key</label>
            <input 
              type="password" 
              placeholder="..."
              value={apiKeys.mistral}
              onChange={(e) => setApiKey('mistral', e.target.value)}
              onBlur={(e) => handleBlur('mistral', e.target.value)}
            />
          </div>
        </div>
      </div>
    </div>
  );
};
