import React from 'react';
import { motion } from 'framer-motion';
import { Link } from 'react-router-dom';

const Download = () => {
  return (
    <div className="min-h-screen p-4 md:p-12 flex flex-col items-center bg-[var(--theme-bg)] text-white relative overflow-hidden">
      {/* HUD Corners */}
      <div className="fixed top-0 left-0 p-8 z-0 pointer-events-none opacity-20">
        <div className="w-16 h-16 border-t-2 border-l-2 border-[var(--theme-primary)]" />
      </div>
      <div className="fixed bottom-0 right-0 p-8 z-0 pointer-events-none opacity-20">
        <div className="w-16 h-16 border-b-2 border-r-2 border-[var(--theme-primary)]" />
      </div>

      <motion.div
        className="w-full max-w-6xl z-10 space-y-12"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <div className="flex flex-col md:flex-row justify-between items-end gap-6 border-b border-white/10 pb-8">
          <div>
            <h2 className="text-xs font-mono uppercase tracking-[0.5em] text-[var(--theme-primary)] mb-2">Systems Deployment</h2>
            <h1 className="text-4xl md:text-6xl font-black italic tracking-tighter uppercase">Download Suite</h1>
          </div>
          <Link to="/" className="text-xs font-mono text-[var(--theme-primary)] hover:underline flex items-center gap-2 mb-2 no-underline">
            <span>[ RETURN TO HUD ]</span>
          </Link>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div className="space-y-6">
            <div className="glass p-8 cyber-border bg-white/[0.01]">
              <h3 className="text-xl font-bold mb-4 flex items-center gap-4 italic uppercase">
                <span className="w-2 h-6 bg-[var(--theme-primary)] inline-block skew-x-12" />
                Latest Stable Release
              </h3>
              <p className="font-mono text-sm opacity-60 mb-8 leading-relaxed">
                Version 1.0.0-BETA // Windows x64 Architecture.
                Experience the next generation of memory discovery with our custom heuristics engine.
              </p>

              <div className="space-y-4">
                <a
                  href="https://github.com/m3rr/h4_GameHack/raw/main/h4_GameHack.exe"
                  className="w-full py-4 bg-[var(--theme-primary)] text-[var(--theme-bg)] font-black text-center block uppercase tracking-widest hover:brightness-110 active:scale-95 transition-all no-underline"
                >
                  Download .EXE (Direct)
                </a>
                <div className="flex justify-between items-center px-4 py-2 border border-white/5 bg-black/20 font-mono text-[10px] opacity-40">
                  <span>SHA-256: 8A2F...3B91</span>
                  <span>SIZE: 12.4 MB</span>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <a
                href="https://github.com/m3rr/h4_GameHack"
                target="_blank"
                rel="noreferrer"
                className="glass p-4 cyber-border text-center group hover:bg-[var(--theme-primary)]/5 transition-colors cursor-pointer no-underline block"
              >
                <div className="text-[10px] font-mono opacity-50 mb-1 uppercase text-white">Source</div>
                <div className="font-bold text-sm tracking-tight group-hover:text-[var(--theme-primary)] text-white">GITHUB REPO</div>
              </a>
              <Link
                to="/changelog"
                className="glass p-4 cyber-border text-center group hover:bg-[var(--theme-primary)]/5 transition-colors cursor-pointer no-underline block"
              >
                <div className="text-[10px] font-mono opacity-50 mb-1 uppercase text-white">Updates</div>
                <div className="font-bold text-sm tracking-tight group-hover:text-[var(--theme-primary)] text-white">CHANGELOG</div>
              </Link>
            </div>
          </div>

          <motion.div
            className="relative"
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.2 }}
          >
            <div className="glass p-2 cyber-border overflow-hidden rotate-1 bg-black/40 shadow-2xl">
              <img
                src="/screenshots/main_ui.png"
                alt="Main Interface"
                className="w-full h-auto grayscale transition-all hover:grayscale-0 duration-500"
              />
            </div>
            <div className="absolute -bottom-6 -left-6 w-1/2 glass p-1 cyber-border -rotate-3 bg-black/60 shadow-xl hidden md:block z-20">
              <img src="/screenshots/process_list.png" alt="Process Node List" className="w-full h-auto opacity-80" />
            </div>
          </motion.div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 pt-12">
          {[
            {
              image: "/screenshots/manual_scan.png",
              title: "Autonomous Probes",
              desc: "Deploy smart heuristics to locate critical game variables automatically via behavioral analysis."
            },
            {
              image: "/screenshots/process_list.png",
              title: "Kernel Synchronization",
              desc: "Seamlessly hook into system processes with our zero-trace architecture and real-time status telemetry."
            },
            {
              image: "/screenshots/main_ui.png",
              title: "Cinematic Workspace",
              desc: "Manage your hacking session through a glassmorphic interface designed for elite operators."
            }
          ].map((feature, i) => (
            <motion.div
              key={i}
              className="glass cyber-border overflow-hidden group hover:border-[var(--theme-primary)]/40 transition-colors"
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 0.4 + i * 0.1 }}
            >
              <div className="h-40 overflow-hidden relative">
                <img src={feature.image} alt={feature.title} className="w-full h-full object-cover grayscale group-hover:grayscale-0 group-hover:scale-110 transition-all duration-700" />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
              </div>
              <div className="p-6">
                <h4 className="font-bold italic uppercase mb-2 tracking-tight">{feature.title}</h4>
                <p className="text-[10px] font-mono opacity-60 leading-relaxed">{feature.desc}</p>
              </div>
            </motion.div>
          ))}
        </div>

        <div className="flex justify-center pt-12 pb-24">
          <div className="font-mono text-[9px] opacity-20 hover:opacity-100 transition-opacity uppercase tracking-[0.2em] cursor-default">
            System Ready // Waiting for Operator Interaction...
          </div>
        </div>
      </motion.div>
    </div>
  );
};

export default Download;
