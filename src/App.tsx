import "./App.css";

function App() {
  return (
    <main className="flex flex-col items-center justify-center min-h-screen bg-[#0c0c16] text-white p-8">
      <div className="flex flex-col items-center gap-6 animate-in fade-in zoom-in duration-700">
        <span className="text-9xl mb-4">🦊</span>
        <h1 className="text-7xl font-bold tracking-tighter bg-gradient-to-br from-violet-400 to-indigo-600 bg-clip-text text-transparent">
          Akela
        </h1>
        <p className="text-violet-300/60 text-xl font-medium">
          Your Intelligent Meeting Companion
        </p>
      </div>
    </main>
  );
}

export default App;
