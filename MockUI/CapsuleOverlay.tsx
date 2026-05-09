import { useState, useEffect, useRef, ReactNode } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import {
  Mic, MicOff, Volume2, VolumeX, Settings, ChevronDown, ChevronUp,
  Sparkles, Bird, History, Send, X, Key, Download,
  Eye, EyeOff, Trash2, Copy, Check, FileText, Sliders, Power,
  Database, Layers,
} from 'lucide-react';

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────
type PanelType = 'none' | 'chat' | 'history' | 'expanded' | 'settings';
type SettingsTab = 'general' | 'apikeys' | 'history' | 'export';
type ChatMessage = { id: number; role: 'user' | 'ai'; text: string; time: string };
type ApiKeyEntry = { id: number; service: string; key: string; visible: boolean; saved: boolean; color: string };

// ─────────────────────────────────────────────────────────────────────────────
// Mock data
// ─────────────────────────────────────────────────────────────────────────────
const SESSIONS = [
  {
    id: 1, date: 'May 8, 2026', time: '14:30', duration: '45 min',
    title: 'Team standup — Sprint planning',
    lines: [
      { speaker: 'User', text: "Let's discuss the project timeline and Q2 deliverables." },
      { speaker: 'Speaker', text: 'Prioritize core features and allocate resources accordingly.' },
      { speaker: 'User', text: 'What about the testing phase?' },
      { speaker: 'Speaker', text: 'Two weeks for comprehensive testing and QA.' },
    ],
  },
  {
    id: 2, date: 'May 8, 2026', time: '10:15', duration: '30 min',
    title: 'Client call — Requirements review',
    lines: [
      { speaker: 'User', text: 'Walk me through the new feature requests.' },
      { speaker: 'Speaker', text: 'Client wants real-time collaboration and offline mode.' },
      { speaker: 'User', text: 'That adds complexity. We need to adjust the timeline.' },
      { speaker: 'Speaker', text: "Agreed. I'll update the project board." },
    ],
  },
  {
    id: 3, date: 'May 7, 2026', time: '16:00', duration: '60 min',
    title: 'Design review — New feature mockups',
    lines: [
      { speaker: 'User', text: 'The new dashboard looks clean. Love the dark mode.' },
      { speaker: 'Speaker', text: 'We iterated on it three times based on user feedback.' },
      { speaker: 'User', text: 'Can we add a quick-access toolbar?' },
      { speaker: 'Speaker', text: "Absolutely. I'll add that to the next sprint." },
    ],
  },
];

const AI_RESPONSES: Record<string, string> = {
  summary:
    '**Session Summary**\nThe team discussed Q2 deliverables with focus on core feature prioritization. Testing phase is allocated 2 weeks. Milestones need to be documented and tickets created in the project board.',
  actions:
    '**Suggested Action Items**\n1. Document agreed Q2 milestones\n2. Create tracking tickets for testing phase\n3. Allocate dedicated resources for core features\n4. Schedule follow-up review in 2 weeks',
  help:
    "I can help you with:\n• Summarizing the current conversation\n• Extracting action items & decisions\n• Answering questions about what was discussed\n• Suggesting next steps based on context",
  default:
    'Based on the current session, the discussion is progressing well around Q2 deliverables and resource allocation. Would you like a summary or a list of action items?',
};

function getAiResponse(msg: string): string {
  const l = msg.toLowerCase();
  if (l.includes('summary') || l.includes('summarize')) return AI_RESPONSES.summary;
  if (l.includes('action') || l.includes('todo') || l.includes('item')) return AI_RESPONSES.actions;
  if (l.includes('help') || l.includes('what can')) return AI_RESPONSES.help;
  return AI_RESPONSES.default;
}

// ─────────────────────────────────────────────────────────────────────────────
// Logo
// ─────────────────────────────────────────────────────────────────────────────
function CapsuleLogo() {
  return (
    <div className="flex items-center gap-2 shrink-0 pr-3 border-r border-white/10">
      <div className="relative shrink-0">
        <div className="size-7 rounded-full bg-gradient-to-br from-violet-500 to-cyan-500 flex items-center justify-center shadow-lg shadow-violet-500/30">
          <Bird className="size-4 text-white" strokeWidth={2.5} />
        </div>
        <div className="absolute -bottom-0.5 -right-0.5 size-2.5 rounded-full bg-emerald-400 border-2 border-[#0c0c16]" />
      </div>
      <span className="text-white text-sm font-semibold tracking-tight whitespace-nowrap">
        ParakeetAI
      </span>
    </div>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// Ticker
// ─────────────────────────────────────────────────────────────────────────────
function TranscriptTicker({ text }: { text: string }) {
  return (
    <div
      className="flex-1 overflow-hidden relative"
      style={{
        height: 20,
        maskImage: 'linear-gradient(to right, transparent 0%, black 6%, black 92%, transparent 100%)',
        WebkitMaskImage: 'linear-gradient(to right, transparent 0%, black 6%, black 92%, transparent 100%)',
      }}
    >
      <style>{`
        @keyframes tickerScroll {
          from { transform: translateX(820px); }
          to   { transform: translateX(-100%); }
        }
        .tk { animation: tickerScroll 20s linear; position: absolute; white-space: nowrap; top: 0; left: 0; }
      `}</style>
      <div key={text} className="tk flex items-center gap-2 h-5">
        <span className="size-1.5 rounded-full bg-red-500 shrink-0 inline-block" />
        <span className="text-white/80 text-sm">{text || 'Waiting for audio…'}</span>
        <span className="text-white/25 mx-3 text-sm">◆</span>
        <span className="text-white/35 text-sm">Live Transcript</span>
      </div>
    </div>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// AI Chat Panel
// ─────────────────────────────────────────────────────────────────────────────
function AIChatPanel({ onClose }: { onClose: () => void }) {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 1, role: 'ai',
      text: "Hi! I'm analyzing your current session. Ask me anything about the conversation — summaries, action items, key decisions, and more.",
      time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    },
  ]);
  const [input, setInput] = useState('');
  const [thinking, setThinking] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  const quickQ = ['Summarize this session', 'List action items', 'What was discussed?', 'Suggest next steps'];

  const send = (text: string) => {
    if (!text.trim() || thinking) return;
    const t = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    setMessages(p => [...p, { id: Date.now(), role: 'user', text, time: t }]);
    setInput('');
    setThinking(true);
    setTimeout(() => {
      setMessages(p => [...p, { id: Date.now() + 1, role: 'ai', text: getAiResponse(text), time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) }]);
      setThinking(false);
    }, 1200 + Math.random() * 800);
  };

  useEffect(() => { bottomRef.current?.scrollIntoView({ behavior: 'smooth' }); }, [messages, thinking]);

  return (
    <PanelShell onClose={onClose} icon={<Sparkles className="size-4 text-violet-400" />} title="AI Assistant" subtitle="Powered by ParakeetAI">
      {/* Quick questions */}
      <div className="px-4 pt-3 pb-2 flex flex-wrap gap-1.5">
        {quickQ.map(q => (
          <button key={q} onClick={() => send(q)}
            className="px-3 py-1 bg-white/6 hover:bg-violet-600/20 border border-white/8 hover:border-violet-500/30 rounded-full text-white/55 hover:text-violet-300 text-xs transition-all"
          >{q}</button>
        ))}
      </div>

      {/* Messages */}
      <div className="px-4 py-2 space-y-3 max-h-[280px] overflow-y-auto">
        <AnimatePresence initial={false}>
          {messages.map(m => (
            <motion.div key={m.id} initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.18 }}
              className={`flex ${m.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              {m.role === 'ai' && (
                <div className="size-6 rounded-full bg-gradient-to-br from-violet-500 to-indigo-600 flex items-center justify-center shrink-0 mt-0.5 mr-2">
                  <Sparkles className="size-3 text-white" />
                </div>
              )}
              <div className={`max-w-[80%] rounded-2xl px-3.5 py-2.5 text-sm leading-relaxed ${
                m.role === 'user'
                  ? 'bg-violet-600/80 text-white rounded-tr-sm'
                  : 'bg-white/8 text-white/85 rounded-tl-sm border border-white/8'
              }`}>
                <p className="whitespace-pre-line">{m.text}</p>
                <p className={`text-[10px] mt-1 ${m.role === 'user' ? 'text-white/45' : 'text-white/25'}`}>{m.time}</p>
              </div>
            </motion.div>
          ))}

          {thinking && (
            <motion.div key="thinking" initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} className="flex justify-start">
              <div className="size-6 rounded-full bg-gradient-to-br from-violet-500 to-indigo-600 flex items-center justify-center shrink-0 mt-0.5 mr-2">
                <Sparkles className="size-3 text-white" />
              </div>
              <div className="bg-white/8 border border-white/8 rounded-2xl rounded-tl-sm px-4 py-3 flex items-center gap-1.5">
                {[0, 1, 2].map(i => (
                  <motion.div key={i} animate={{ y: [-3, 0, -3] }} transition={{ duration: 0.6, repeat: Infinity, delay: i * 0.15 }}
                    className="size-1.5 rounded-full bg-violet-400"
                  />
                ))}
              </div>
            </motion.div>
          )}
        </AnimatePresence>
        <div ref={bottomRef} />
      </div>

      {/* Input */}
      <div className="px-4 py-3 border-t border-white/8">
        <div className="flex items-center gap-2 bg-white/6 border border-white/10 focus-within:border-white/20 rounded-xl px-4 py-2.5 transition-colors">
          <input
            type="text" value={input} onChange={e => setInput(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && send(input)}
            placeholder="Ask about this session…"
            className="flex-1 bg-transparent text-white/80 text-sm placeholder:text-white/25 outline-none"
          />
          <button onClick={() => send(input)} disabled={!input.trim() || thinking}
            className="p-1.5 bg-violet-600 hover:bg-violet-500 disabled:opacity-30 rounded-lg transition-all"
          >
            <Send className="size-3.5 text-white" />
          </button>
        </div>
      </div>
    </PanelShell>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// History Panel
// ─────────────────────────────────────────────────────────────────────────────
function HistoryPanel({ onClose, onAskAI }: { onClose: () => void; onAskAI: () => void }) {
  return (
    <PanelShell onClose={onClose} icon={<History className="size-4 text-white/55" />} title="Session History"
      badge={`${SESSIONS.length} sessions`}
    >
      <div className="p-4 space-y-3 max-h-[400px] overflow-y-auto">
        {SESSIONS.map(session => (
          <div key={session.id} className="bg-white/4 border border-white/6 hover:border-white/12 rounded-xl p-4 transition-all">
            {/* Header */}
            <div className="flex items-start justify-between mb-3">
              <div>
                <p className="text-white/90 text-sm font-medium">{session.title}</p>
                <div className="flex items-center gap-2 mt-0.5">
                  <span className="text-white/35 text-[11px]">{session.date} · {session.time}</span>
                  <span className="size-1 rounded-full bg-white/20" />
                  <span className="text-white/35 text-[11px]">{session.duration}</span>
                </div>
              </div>
              {/* Mini Help Me */}
              <button onClick={onAskAI}
                className="flex items-center gap-1 px-2.5 py-1.5 bg-violet-600/20 hover:bg-violet-600/35 border border-violet-500/20 hover:border-violet-500/40 rounded-full text-violet-300 transition-all shrink-0"
                title="Ask AI about this session"
              >
                <Sparkles className="size-3" />
                <span className="text-[10px] font-semibold">Ask AI</span>
              </button>
            </div>

            {/* Transcript preview */}
            <div className="space-y-1.5">
              {session.lines.slice(0, 3).map((line, i) => (
                <p key={i} className="text-sm">
                  <span className={`font-semibold ${line.speaker === 'User' ? 'text-violet-400' : 'text-cyan-400'}`}>
                    {line.speaker}:&nbsp;
                  </span>
                  <span className="text-white/50">{line.text}</span>
                </p>
              ))}
              {session.lines.length > 3 && (
                <p className="text-white/25 text-[11px]">+{session.lines.length - 3} more lines</p>
              )}
            </div>
          </div>
        ))}
      </div>
    </PanelShell>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// Expanded Transcript Panel (no AI insights, no previous sessions)
// ─────────────────────────────────────────────────────────────────────────────
function ExpandedPanel({ selectedMic, selectedSpeaker, onClose }: {
  selectedMic: string; selectedSpeaker: string; onClose: () => void;
}) {
  const lines = [
    { speaker: 'User', color: 'text-violet-400', text: "Let's discuss the project timeline and deliverables for Q2." },
    { speaker: 'Speaker', color: 'text-cyan-400', text: 'I think we should prioritize core features and allocate resources accordingly.' },
    { speaker: 'User', color: 'text-violet-400', text: 'That makes sense. What about the testing phase?' },
    { speaker: 'Speaker', color: 'text-cyan-400', text: 'Two weeks for comprehensive testing and QA should be enough.' },
    { speaker: 'User', color: 'text-violet-400', text: "Sounds good. Let's finalize the milestones today." },
  ];

  return (
    <PanelShell onClose={onClose}
      icon={<span className="size-2 rounded-full bg-emerald-400 animate-pulse inline-block" />}
      title="Live Session"
    >
      <div className="p-5 space-y-5">
        {/* Stats */}
        <div className="grid grid-cols-3 gap-2">
          {[
            { label: 'Duration', value: '12:34' },
            { label: 'Microphone', value: selectedMic.split(' ')[0] },
            { label: 'Speaker', value: selectedSpeaker.split(' ')[0] },
          ].map(({ label, value }) => (
            <div key={label} className="bg-white/5 border border-white/6 rounded-xl p-3 text-center">
              <p className="text-white/35 text-[10px] uppercase tracking-wide mb-1">{label}</p>
              <p className="text-white text-sm font-semibold truncate">{value}</p>
            </div>
          ))}
        </div>

        {/* Transcript */}
        <div>
          <p className="text-white/35 text-[10px] font-semibold uppercase tracking-widest mb-3">Transcript</p>
          <div className="space-y-2 max-h-[260px] overflow-y-auto">
            {lines.map((e, i) => (
              <div key={i} className="bg-white/4 border border-white/6 rounded-xl p-3">
                <p className="text-sm leading-relaxed">
                  <span className={`font-semibold ${e.color}`}>{e.speaker}:&nbsp;</span>
                  <span className="text-white/70">{e.text}</span>
                </p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </PanelShell>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// Settings Control Panel — 4 tabs
// ─────────────────────────────────────────────────────────────────────────────
function SettingsControlPanel({ onClose, onEndSession }: { onClose: () => void; onEndSession: () => void }) {
  const [tab, setTab] = useState<SettingsTab>('general');
  const [opacity, setOpacity] = useState(90);
  const [blur, setBlur] = useState(80);
  const [autoStart, setAutoStart] = useState(true);
  const [saveTranscripts, setSaveTranscripts] = useState(false);
  const [noise, setNoise] = useState(true);
  const [apiKeys, setApiKeys] = useState<ApiKeyEntry[]>([
    { id: 1, service: 'OpenAI',        key: '', visible: false, saved: false, color: '#10a37f' },
    { id: 2, service: 'Anthropic',     key: '', visible: false, saved: false, color: '#d97706' },
    { id: 3, service: 'Google Gemini', key: '', visible: false, saved: false, color: '#4285f4' },
    { id: 4, service: 'AssemblyAI',    key: '', visible: false, saved: false, color: '#7c3aed' },
    { id: 5, service: 'Deepgram',      key: '', visible: false, saved: false, color: '#0ea5e9' },
  ]);
  const [copiedId, setCopiedId] = useState<number | null>(null);
  const [exportFmt, setExportFmt] = useState<'txt' | 'json' | 'md'>('txt');
  const [exportRange, setExportRange] = useState<'7d' | '30d' | 'all'>('all');
  const [confirmEnd, setConfirmEnd] = useState(false);

  const TABS: { id: SettingsTab; label: string; icon: ReactNode }[] = [
    { id: 'general',  label: 'General',   icon: <Sliders className="size-3.5" /> },
    { id: 'apikeys',  label: 'API Keys',  icon: <Key className="size-3.5" /> },
    { id: 'history',  label: 'History',   icon: <Database className="size-3.5" /> },
    { id: 'export',   label: 'Export',    icon: <Download className="size-3.5" /> },
  ];

  const placeholders: Record<string, string> = {
    'OpenAI': 'sk-…', 'Anthropic': 'sk-ant-…', 'Google Gemini': 'AIzaSy…',
    'AssemblyAI': 'Enter key…', 'Deepgram': 'Enter key…',
  };

  const updateKey  = (id: number, v: string) => setApiKeys(p => p.map(k => k.id === id ? { ...k, key: v, saved: false } : k));
  const toggleVis  = (id: number) =>           setApiKeys(p => p.map(k => k.id === id ? { ...k, visible: !k.visible } : k));
  const saveKey    = (id: number) =>           setApiKeys(p => p.map(k => k.id === id ? { ...k, saved: true } : k));
  const deleteKey  = (id: number) =>           setApiKeys(p => p.map(k => k.id === id ? { ...k, key: '', saved: false } : k));
  const copyKey    = (id: number, key: string) => {
    navigator.clipboard.writeText(key).catch(() => {});
    setCopiedId(id); setTimeout(() => setCopiedId(null), 1500);
  };

  return (
    <PanelShell onClose={onClose} icon={<Settings className="size-4 text-white/55" />} title="Settings">
      {/* Tabs */}
      <div className="flex items-center gap-1 px-4 pt-3 pb-1">
        {TABS.map(t => (
          <button key={t.id} onClick={() => setTab(t.id)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all ${
              tab === t.id
                ? 'bg-violet-600/25 text-violet-300 border border-violet-500/30'
                : 'text-white/40 hover:text-white/70 hover:bg-white/6'
            }`}
          >
            {t.icon}{t.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="p-4 max-h-[340px] overflow-y-auto">

        {/* ── General ── */}
        {tab === 'general' && (
          <div className="space-y-5">
            <div>
              <SectionLabel>Overlay Appearance</SectionLabel>
              <div className="space-y-4">
                <SliderRow label="Transparency" value={opacity} unit="%" onChange={setOpacity} min={30} max={100} />
                <SliderRow label="Blur Intensity" value={blur} unit="%" onChange={setBlur} min={0} max={100} />
              </div>
            </div>
            <div className="border-t border-white/6" />
            <div>
              <SectionLabel>Behavior</SectionLabel>
              <div className="space-y-2">
                {[
                  { label: 'Auto-start audio capture', value: autoStart, set: setAutoStart },
                  { label: 'Save transcripts locally',  value: saveTranscripts, set: setSaveTranscripts },
                  { label: 'Noise suppression',          value: noise, set: setNoise },
                ].map(({ label, value, set }) => (
                  <div key={label} className="flex items-center justify-between bg-white/4 border border-white/6 rounded-xl px-4 py-2.5">
                    <span className="text-white/70 text-sm">{label}</span>
                    <button onClick={() => set(!value)}
                      className={`relative w-9 h-5 rounded-full transition-colors ${value ? 'bg-violet-500' : 'bg-white/15'}`}
                    >
                      <div className={`absolute top-0.5 size-4 rounded-full bg-white shadow transition-transform ${value ? 'translate-x-4' : 'translate-x-0.5'}`} />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* ── API Keys ── */}
        {tab === 'apikeys' && (
          <div className="space-y-2">
            <SectionLabel>API Key Management</SectionLabel>
            {apiKeys.map(entry => (
              <div key={entry.id} className="group flex items-center gap-2 bg-white/4 hover:bg-white/6 border border-white/6 hover:border-white/12 rounded-xl px-3 py-2.5 transition-all">
                {/* Service */}
                <div className="flex items-center gap-2 w-[118px] shrink-0">
                  <div className="size-2 rounded-full shrink-0" style={{ backgroundColor: entry.color, boxShadow: `0 0 5px ${entry.color}80` }} />
                  <span className="text-white/70 text-xs font-medium truncate">{entry.service}</span>
                </div>
                {/* Status */}
                <div className="shrink-0">
                  {entry.saved && entry.key
                    ? <span className="text-[9px] font-bold px-1.5 py-0.5 rounded-full bg-emerald-500/15 text-emerald-400 border border-emerald-500/20">ACTIVE</span>
                    : <span className="text-[9px] font-bold px-1.5 py-0.5 rounded-full bg-white/5 text-white/25 border border-white/8">NOT SET</span>
                  }
                </div>
                {/* Input */}
                <div className="flex-1 min-w-0">
                  <input
                    type={entry.visible ? 'text' : 'password'}
                    value={entry.key}
                    onChange={e => updateKey(entry.id, e.target.value)}
                    placeholder={placeholders[entry.service] ?? 'Enter key…'}
                    className="w-full bg-black/30 border border-white/8 focus:border-white/25 rounded-lg px-2.5 py-1.5 text-white/80 text-xs placeholder:text-white/20 outline-none font-mono transition-colors"
                  />
                </div>
                {/* Actions */}
                <div className="flex items-center gap-0.5 shrink-0">
                  <IconBtn onClick={() => toggleVis(entry.id)} title={entry.visible ? 'Hide' : 'Show'}>
                    {entry.visible ? <EyeOff className="size-3.5 text-white/35" /> : <Eye className="size-3.5 text-white/35" />}
                  </IconBtn>
                  {entry.key && (
                    <IconBtn onClick={() => copyKey(entry.id, entry.key)} title="Copy">
                      {copiedId === entry.id ? <Check className="size-3.5 text-emerald-400" /> : <Copy className="size-3.5 text-white/35" />}
                    </IconBtn>
                  )}
                  {entry.key && !entry.saved && (
                    <IconBtn onClick={() => saveKey(entry.id)} title="Save" className="hover:bg-emerald-500/20">
                      <Check className="size-3.5 text-emerald-400" />
                    </IconBtn>
                  )}
                  {entry.key && (
                    <IconBtn onClick={() => deleteKey(entry.id)} title="Remove" className="opacity-0 group-hover:opacity-100 hover:bg-red-500/20">
                      <Trash2 className="size-3.5 text-red-400/60" />
                    </IconBtn>
                  )}
                </div>
              </div>
            ))}
            <p className="text-white/20 text-[10px] px-1 pt-1">Keys stored locally — never sent to ParakeetAI servers.</p>
          </div>
        )}

        {/* ── History viewer ── */}
        {tab === 'history' && (
          <div className="space-y-4">
            <SectionLabel>Stored Sessions</SectionLabel>
            <div className="space-y-2">
              {SESSIONS.map(s => (
                <div key={s.id} className="flex items-center justify-between bg-white/4 border border-white/6 rounded-xl px-4 py-3">
                  <div className="min-w-0">
                    <p className="text-white/75 text-sm font-medium truncate">{s.title}</p>
                    <p className="text-white/30 text-[11px] mt-0.5">{s.date} · {s.duration}</p>
                  </div>
                  <div className="flex items-center gap-1 shrink-0 ml-3">
                    <IconBtn title="Export" className="hover:bg-white/10">
                      <Download className="size-3.5 text-white/40" />
                    </IconBtn>
                    <IconBtn title="Delete" className="hover:bg-red-500/20">
                      <Trash2 className="size-3.5 text-white/30" />
                    </IconBtn>
                  </div>
                </div>
              ))}
            </div>
            <div className="flex gap-2">
              <button className="flex-1 flex items-center justify-center gap-2 py-2.5 bg-white/6 hover:bg-white/10 border border-white/8 rounded-xl text-white/55 text-xs font-medium transition-all">
                <Download className="size-3.5" />Export All
              </button>
              <button className="px-4 py-2.5 bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 rounded-xl text-red-400 text-xs font-medium transition-all">
                Clear All
              </button>
            </div>
          </div>
        )}

        {/* ── Export management ── */}
        {tab === 'export' && (
          <div className="space-y-5">
            <div>
              <SectionLabel>Format</SectionLabel>
              <div className="grid grid-cols-3 gap-2">
                {[
                  { id: 'txt',  label: 'Plain Text', icon: <FileText className="size-4" /> },
                  { id: 'json', label: 'JSON',       icon: <Layers className="size-4" /> },
                  { id: 'md',   label: 'Markdown',   icon: <FileText className="size-4" /> },
                ].map(f => (
                  <button key={f.id} onClick={() => setExportFmt(f.id as 'txt' | 'json' | 'md')}
                    className={`flex flex-col items-center gap-1.5 py-3 rounded-xl border text-xs font-medium transition-all ${
                      exportFmt === f.id
                        ? 'bg-violet-600/20 border-violet-500/30 text-violet-300'
                        : 'bg-white/4 border-white/8 text-white/40 hover:bg-white/8'
                    }`}
                  >{f.icon}{f.label}</button>
                ))}
              </div>
            </div>
            <div>
              <SectionLabel>Date Range</SectionLabel>
              <div className="flex gap-2">
                {[{ id: '7d', label: 'Last 7 Days' }, { id: '30d', label: 'Last 30 Days' }, { id: 'all', label: 'All Time' }].map(r => (
                  <button key={r.id} onClick={() => setExportRange(r.id as '7d' | '30d' | 'all')}
                    className={`flex-1 py-2 rounded-xl border text-xs font-medium transition-all ${
                      exportRange === r.id
                        ? 'bg-violet-600/20 border-violet-500/30 text-violet-300'
                        : 'bg-white/4 border-white/8 text-white/40 hover:bg-white/8'
                    }`}
                  >{r.label}</button>
                ))}
              </div>
            </div>
            <button className="w-full flex items-center justify-center gap-2 py-3 bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 rounded-xl text-white text-sm font-semibold transition-all shadow-lg shadow-violet-600/25">
              <Download className="size-4" />Export Now
            </button>
          </div>
        )}
      </div>

      {/* End Session */}
      <div className="px-4 py-3 border-t border-white/8">
        {confirmEnd ? (
          <div className="flex items-center gap-2">
            <p className="text-white/55 text-xs flex-1">End current session?</p>
            <button onClick={() => setConfirmEnd(false)}
              className="px-3 py-1.5 bg-white/8 hover:bg-white/14 rounded-lg text-white/55 text-xs transition-all">
              Cancel
            </button>
            <button onClick={onEndSession}
              className="px-3 py-1.5 bg-red-600 hover:bg-red-500 rounded-lg text-white text-xs font-semibold transition-all">
              End Session
            </button>
          </div>
        ) : (
          <button onClick={() => setConfirmEnd(true)}
            className="w-full flex items-center justify-center gap-2 py-2.5 bg-red-500/10 hover:bg-red-500/18 border border-red-500/22 hover:border-red-500/40 rounded-xl text-red-400 text-sm font-semibold transition-all"
          >
            <Power className="size-4" />End Session
          </button>
        )}
      </div>
    </PanelShell>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared primitives
// ─────────────────────────────────────────────────────────────────────────────
function PanelShell({ children, onClose, icon, title, subtitle, badge }: {
  children: ReactNode; onClose: () => void;
  icon?: ReactNode; title: string; subtitle?: string; badge?: string;
}) {
  return (
    <motion.div
      initial={{ opacity: 0, y: -8, scale: 0.97 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: -8, scale: 0.97 }}
      transition={{ duration: 0.22, ease: 'easeOut' }}
      className="absolute top-0 left-0 right-0 bg-[#0f0f18]/96 backdrop-blur-2xl border border-white/10 rounded-2xl shadow-2xl overflow-hidden"
      style={{ zIndex: 40 }}
    >
      <div className="flex items-center justify-between px-5 py-3.5 border-b border-white/8">
        <div className="flex items-center gap-2.5">
          <div className="size-6 flex items-center justify-center">{icon}</div>
          <div>
            <p className="text-white text-sm font-semibold leading-none">{title}</p>
            {subtitle && <p className="text-white/35 text-[10px] mt-0.5">{subtitle}</p>}
          </div>
          {badge && (
            <span className="text-[10px] px-1.5 py-0.5 bg-white/8 rounded-full text-white/35">{badge}</span>
          )}
        </div>
        <button onClick={onClose} className="p-1.5 hover:bg-white/10 rounded-lg transition-colors">
          <X className="size-4 text-white/40" />
        </button>
      </div>
      {children}
    </motion.div>
  );
}

function SectionLabel({ children }: { children: ReactNode }) {
  return <p className="text-white/40 text-[10px] font-semibold uppercase tracking-widest mb-3">{children}</p>;
}

function SliderRow({ label, value, unit, onChange, min = 0, max = 100 }: {
  label: string; value: number; unit: string; onChange: (v: number) => void; min?: number; max?: number;
}) {
  return (
    <div>
      <div className="flex justify-between items-center mb-2">
        <span className="text-white/65 text-sm">{label}</span>
        <span className="text-white/35 text-xs">{value}{unit}</span>
      </div>
      <input type="range" min={min} max={max} value={value} onChange={e => onChange(Number(e.target.value))}
        className="w-full h-1.5 rounded-full appearance-none cursor-pointer"
        style={{ accentColor: '#7c3aed' }}
      />
    </div>
  );
}

function IconBtn({ children, onClick, title, className = '' }: {
  children: ReactNode; onClick?: () => void; title?: string; className?: string;
}) {
  return (
    <button onClick={onClick} title={title}
      className={`p-1.5 rounded-lg transition-all ${className || 'hover:bg-white/10'}`}
    >{children}</button>
  );
}

// ─────────────────────────────────────────────────────────────────────────────
// Main Capsule
// ─────────────────────────────────────────────────────────────────────────────
export function CapsuleOverlay() {
  const [activePanel, setActivePanel] = useState<PanelType>('none');
  const [micEnabled, setMicEnabled] = useState(true);
  const [speakerEnabled, setSpeakerEnabled] = useState(true);
  const [transcript, setTranscript] = useState('');
  const containerRef = useRef<HTMLDivElement>(null);

  const phrases = [
    "User: Let's discuss the project timeline and Q2 deliverables.",
    'Speaker: We should prioritize core features and allocate resources accordingly.',
    'User: What about the testing phase? Do we have enough time?',
    'Speaker: Two weeks for comprehensive testing — that should be sufficient.',
    "User: Great. Let's finalize the milestones before end of day.",
  ];

  useEffect(() => {
    let i = 0;
    const iv = setInterval(() => { setTranscript(phrases[i % phrases.length]); i++; }, 6000);
    return () => clearInterval(iv);
  }, []);

  useEffect(() => {
    const h = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node))
        setActivePanel('none');
    };
    document.addEventListener('mousedown', h);
    return () => document.removeEventListener('mousedown', h);
  }, []);

  const toggle = (p: PanelType) => setActivePanel(prev => prev === p ? 'none' : p);

  return (
    <div className="fixed top-8 left-1/2 -translate-x-1/2 z-50" ref={containerRef}>
      <div style={{ width: 820 }}>

        {/* ── Compact bar ── */}
        <div className="bg-[#0c0c16]/92 backdrop-blur-2xl border border-white/10 rounded-full px-5 py-2.5 shadow-2xl shadow-black/70">
          <div className="flex items-center gap-3">

            <CapsuleLogo />
            <TranscriptTicker text={transcript} />

            <div className="h-5 w-px bg-white/10 shrink-0" />

            {/* Help Me */}
            <button onClick={() => toggle('chat')}
              className={`flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-white text-xs font-semibold transition-all shrink-0 ${
                activePanel === 'chat'
                  ? 'bg-violet-500 shadow-lg shadow-violet-500/40'
                  : 'bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 shadow-lg shadow-violet-600/30 hover:shadow-violet-500/50'
              }`}
            >
              <Sparkles className="size-3.5" />Help Me
            </button>

            <div className="h-5 w-px bg-white/10 shrink-0" />

            {/* Mic — enable/disable only */}
            <button onClick={() => setMicEnabled(!micEnabled)}
              className={`relative p-2 rounded-full transition-all ${micEnabled ? 'hover:bg-white/8' : 'hover:bg-red-500/15'}`}
              title={micEnabled ? 'Mic On — click to mute' : 'Mic Off — click to unmute'}
            >
              {micEnabled
                ? <Mic className="size-4 text-emerald-400" />
                : <MicOff className="size-4 text-red-400" />}
              <span className={`absolute -bottom-0.5 -right-0.5 size-2 rounded-full border-2 border-[#0c0c16] ${micEnabled ? 'bg-emerald-400' : 'bg-red-400'}`} />
            </button>

            {/* Speaker — enable/disable only */}
            <button onClick={() => setSpeakerEnabled(!speakerEnabled)}
              className={`relative p-2 rounded-full transition-all ${speakerEnabled ? 'hover:bg-white/8' : 'hover:bg-red-500/15'}`}
              title={speakerEnabled ? 'Speaker On — click to mute' : 'Speaker Off — click to unmute'}
            >
              {speakerEnabled
                ? <Volume2 className="size-4 text-emerald-400" />
                : <VolumeX className="size-4 text-red-400" />}
              <span className={`absolute -bottom-0.5 -right-0.5 size-2 rounded-full border-2 border-[#0c0c16] ${speakerEnabled ? 'bg-emerald-400' : 'bg-red-400'}`} />
            </button>

            <div className="h-5 w-px bg-white/10 shrink-0" />

            {/* History */}
            <button onClick={() => toggle('history')}
              className={`p-2 rounded-full transition-colors ${activePanel === 'history' ? 'bg-white/15 text-white' : 'hover:bg-white/8 text-white/60'}`}
              title="Session History"
            >
              <History className="size-4" />
            </button>

            {/* Expand */}
            <button onClick={() => toggle('expanded')}
              className={`p-2 rounded-full transition-colors ${activePanel === 'expanded' ? 'bg-white/15 text-white' : 'hover:bg-white/8 text-white/60'}`}
              title={activePanel === 'expanded' ? 'Collapse' : 'Expand transcript'}
            >
              {activePanel === 'expanded' ? <ChevronUp className="size-4" /> : <ChevronDown className="size-4" />}
            </button>

            {/* Settings */}
            <button onClick={() => toggle('settings')}
              className={`p-2 rounded-full transition-colors ${activePanel === 'settings' ? 'bg-white/15 text-white' : 'hover:bg-white/8 text-white/60'}`}
              title="Settings"
            >
              <Settings className="size-4" />
            </button>
          </div>
        </div>

        {/* ── Panels ── */}
        <div className="relative mt-3">
          <AnimatePresence>
            {activePanel === 'chat' && (
              <AIChatPanel key="chat" onClose={() => setActivePanel('none')} />
            )}
            {activePanel === 'history' && (
              <HistoryPanel key="history"
                onClose={() => setActivePanel('none')}
                onAskAI={() => setActivePanel('chat')}
              />
            )}
            {activePanel === 'expanded' && (
              <ExpandedPanel key="expanded"
                selectedMic="Default Microphone"
                selectedSpeaker="System Audio"
                onClose={() => setActivePanel('none')}
              />
            )}
            {activePanel === 'settings' && (
              <SettingsControlPanel key="settings"
                onClose={() => setActivePanel('none')}
                onEndSession={() => setActivePanel('none')}
              />
            )}
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
