import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

const App = () => {
  const [greeting, setGreeting] = useState('Hello from LibreChat Desktop');

  useEffect(() => {
    const fetchGreeting = async () => {
      try {
        const message = await invoke<string>('get_greeting');
        setGreeting(message);
      } catch (error) {
        console.warn('Failed to load greeting from backend', error);
      }
    };

    fetchGreeting();
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-6 bg-gradient-to-br from-slate-900 via-slate-950 to-black p-8 text-center">
      <div className="rounded-3xl border border-slate-800 bg-slate-900/60 p-10 shadow-2xl backdrop-blur">
        <h1 className="text-4xl font-semibold tracking-tight text-slate-100">LibreChat Desktop</h1>
        <p className="mt-4 text-lg text-slate-300">A Tauri-powered desktop experience for LibreChat.</p>
        <p className="mt-6 text-xl font-medium text-emerald-400">{greeting}</p>
      </div>
      <section className="max-w-xl text-left text-sm text-slate-400">
        <h2 className="text-base font-semibold uppercase tracking-widest text-slate-500">Getting Started</h2>
        <ul className="mt-4 space-y-2">
          <li>• Run <code className="rounded bg-slate-800 px-2 py-1">npm install</code> to install frontend dependencies.</li>
          <li>• Run <code className="rounded bg-slate-800 px-2 py-1">npm run tauri dev</code> to start the desktop app.</li>
          <li>• Edit <code className="rounded bg-slate-800 px-2 py-1">src/App.tsx</code> and save to test hot reload.</li>
        </ul>
      </section>
    </main>
  );
};

export default App;
