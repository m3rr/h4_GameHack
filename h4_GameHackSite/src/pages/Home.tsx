import React from 'react';
import { motion } from 'framer-motion';
import { Link } from 'react-router-dom';

export const Home = () => {
  return (
    <div className="min-h-screen p-4 md:p-12 flex flex-col items-center justify-center relative overflow-hidden">
      {/* Background Decorative Assets - Moved to Public/Assets */}
      <div className="absolute inset-0 z-0 opacity-20 pointer-events-none overflow-hidden">
        <img
          src="/assets/netrunner_dashboard.png"
          alt="Dashboard HUD"
          className="absolute -top-20 -left-20 w-full h-full object-contain filter invert mix-blend-screen opacity-20"
        />
        <img
          src="/assets/holographic_hud.png"
          alt="Holographic HUD"
          className="absolute -bottom-20 -right-20 w-full h-full object-contain filter invert mix-blend-screen animate-pulse opacity-20"
        />
      </div>

      {/* Main Grid Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 w-full max-w-7xl z-10">

        {/* Hero Section */}
        <motion.div
          className="lg:col-span-8 space-y-8"
          initial={{ opacity: 0, x: -50 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.2 }}
        >
          <div className="glass p-8 md:p-12 cyber-border relative group">
            <div className="absolute top-0 right-0 p-4 font-mono text-[10px] opacity-30 group-hover:opacity-100 transition-opacity">
              BUILD: v1.0.0-BETA (EXPERIMENTAL)
            </div>

            <h1 className="text-5xl md:text-8xl font-black mb-4 tracking-tighter italic">
              H4_G<span className="glitch-text text-[var(--theme-primary)]">AME</span>HACK
            </h1>
            <p className="text-lg md:text-xl font-mono text-[var(--theme-primary)] opacity-80 max-w-2xl leading-relaxed">
              Precision memory engineering for the modern operator. Deploy kernel-level hooks, resolve complex pointers, and bypass anti-cheat systems with zero-trace architecture.
            </p>

            <div className="mt-12 flex flex-wrap gap-4">
              <a
                href="https://github.com/m3rr/h4_GameHack/raw/main/h4_GameHack.exe"
                className="px-8 py-3 bg-[var(--theme-primary)] text-[var(--theme-bg)] font-bold uppercase tracking-widest hover:skew-x-2 transition-transform cursor-pointer active:scale-95 no-underline"
              >
                Download Suite
              </a>
              <a
                href="https://github.com/m3rr/h4_GameHack"
                target="_blank"
                className="px-8 py-3 border border-[var(--theme-primary)] text-[var(--theme-primary)] font-bold uppercase tracking-widest hover:-skew-x-2 transition-transform cursor-pointer active:scale-95 no-underline"
              >
                Documentation
              </a>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Link to="/details" className="glass p-6 cyber-border group hover:bg-white/[0.02] transition-all cursor-pointer no-underline block overflow-hidden active:scale-[0.98]">
              <h3 className="text-xs font-mono uppercase tracking-[0.3em] mb-4 opacity-50 group-hover:opacity-100 transition-opacity flex justify-between items-center">
                <span>Discovery Performance</span>
                <span className="text-[var(--theme-primary)] text-[9px] animate-pulse">VIEW TELEMETRY {">"}</span>
              </h3>
              <div className="space-y-4">
                {[
                  { label: "Memory Skip (Fast Path)", value: "82% Logic Accuracy", width: "82%" },
                  { label: "AOB Heuristics", value: "60% Behavioral Score", width: "60%" },
                  { label: "Stability Interlock", value: "85% Kernel Guard", width: "85%" },
                  { label: "Match Cap Reliability", value: "80% Buffer Load", width: "80%" }
                ].map((stat, i) => (
                  <div key={i} className="space-y-1">
                    <div className="flex justify-between text-[10px] font-mono">
                      <span>{stat.label}</span>
                      <span>{stat.value}</span>
                    </div>
                    <div className="h-1 bg-[var(--theme-primary)]/10 w-full overflow-hidden">
                      <motion.div
                        className="h-full bg-[var(--theme-primary)] shadow-[0_0_8px_var(--theme-primary)]"
                        initial={{ width: 0 }}
                        animate={{ width: stat.width }}
                        transition={{ delay: 0.5 + i * 0.2, duration: 1 }}
                      />
                    </div>
                  </div>
                ))}
              </div>
            </Link>
            <Link to="/changelog" className="glass p-6 cyber-border relative group overflow-hidden hover:bg-white/[0.02] transition-all cursor-pointer no-underline block active:scale-[0.98]">
              <img
                src="/assets/system_corruption.png"
                alt="System Corruption"
                className="absolute inset-0 w-full h-full object-cover opacity-5 group-hover:opacity-10 transition-opacity pointer-events-none"
              />
              <h3 className="text-xs font-mono uppercase tracking-[0.3em] mb-4 opacity-50 relative z-10 group-hover:opacity-100 transition-opacity flex justify-between items-center text-[var(--theme-primary)]">
                <span>Software Status</span>
                <span className="text-[var(--theme-primary)] text-[9px] animate-pulse italic">LIVE_FEED {">"}</span>
              </h3>
              <ul className="text-xs font-mono space-y-2 text-[var(--theme-primary)]/80 relative z-10">
                <li className="flex gap-2"><span>[v1.0.0-Beta]</span> <span>Initial Public Release Candidate.</span></li>
                <li className="flex gap-2"><span>[v0.9.8]</span> <span>Core Rust Engine Modularization.</span></li>
                <li className="flex gap-2 text-[var(--theme-primary)]/30"><span>[...]</span> <span>FETCHING LATEST COMMITS...</span></li>
              </ul>
            </Link>
          </div>
        </motion.div>

        {/* Sidebar / Info */}
        <motion.div
          className="lg:col-span-4 space-y-6"
          initial={{ opacity: 0, x: 50 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.4 }}
        >
          <div className="glass p-6 cyber-border group/hud">
            <h3 className="text-xs font-mono uppercase tracking-[0.3em] mb-4 opacity-50 flex justify-between">
              <span>Autonomous Diagnostics</span>
              <span className="text-[var(--theme-primary)] text-[9px] animate-pulse">HUD_LINK_ACTIVE</span>
            </h3>
            <div className="relative h-64 overflow-hidden border border-white/5 bg-[var(--theme-bg)]/40 group-hover/hud:border-[var(--theme-primary)]/30 transition-colors">
              <img
                src="/assets/h4_app_interface.png"
                alt="H4 Engine Interface"
                className="w-full h-full object-cover object-top opacity-60 group-hover/hud:opacity-100 group-hover/hud:scale-105 transition-all duration-700 pointer-events-none"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-[var(--theme-bg)]/80 via-transparent to-transparent pointer-events-none" />
              <div className="absolute top-2 right-2 px-2 py-0.5 bg-[var(--theme-bg)]/60 border border-white/10 text-[7px] font-mono tracking-widest opacity-40 group-hover/hud:opacity-100 transition-opacity text-[var(--theme-primary)]">
                SRC: H4_ENGINE_SNAPSHOT
              </div>
            </div>
          </div>

          <div className="glass p-6 cyber-border">
            <h3 className="text-xs font-mono uppercase tracking-[0.3em] mb-4 opacity-50">Operational Ethics</h3>
            <div className="space-y-4">
              <p className="text-[10px] font-mono opacity-80 leading-relaxed">
                H4 is a sandbox for single-player manipulation and system troubleshooting.
                We assume zero liability for your actions. Secure your rigs, read the license (GPL v3),
                and act like an operator.
              </p>
              <div className="flex justify-between items-center text-[10px] font-mono border-t border-white/10 pt-4">
                <span>RESONANCE:</span>
                <span className="text-[var(--theme-primary)]">INDUSTRIAL STABLE</span>
              </div>
            </div>
          </div>
        </motion.div>
      </div>

      {/* HUD Accents */}
      <div className="fixed top-0 left-0 p-8 z-0 pointer-events-none opacity-20 hidden md:block">
        <div className="w-12 h-12 border-t-2 border-l-2 border-[var(--theme-primary)]" />
      </div>
      <div className="fixed top-0 right-0 p-8 z-0 pointer-events-none opacity-20 hidden md:block">
        <div className="w-12 h-12 border-t-2 border-r-2 border-[var(--theme-primary)]" />
      </div>
      <div className="fixed bottom-0 left-0 p-8 z-0 pointer-events-none opacity-20 hidden md:block">
        <div className="w-12 h-12 border-b-2 border-l-2 border-[var(--theme-primary)]" />
      </div>
      <div className="fixed bottom-0 right-0 p-8 z-0 pointer-events-none opacity-20 hidden md:block">
        <div className="w-12 h-12 border-b-2 border-r-2 border-[var(--theme-primary)]" />
      </div>
    </div>
  );
};

export default Home;
