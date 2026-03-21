import React from 'react';
import { motion } from 'framer-motion';
import { Link } from 'react-router-dom';

export const DetailedBreakdown = () => {
    return (
        <div className="min-h-screen p-4 md:p-12 bg-[var(--theme-bg)] text-[var(--theme-primary)] relative overflow-x-hidden">
            {/* HUD Backdrop */}
            <div className="fixed inset-0 pointer-events-none opacity-5">
                <div className="absolute inset-0 grid grid-cols-12 gap-1 px-4">
                    {[...Array(12)].map((_, i) => (
                        <div key={i} className="h-full border-x border-white/10" />
                    ))}
                </div>
            </div>

            <div className="max-w-6xl mx-auto z-10 relative">
                {/* Navigation Header */}
                <div className="flex justify-between items-center mb-12 border-b border-white/10 pb-6">
                    <Link to="/" className="text-[var(--theme-primary)] font-mono text-sm hover:opacity-100 transition-opacity no-underline flex items-center gap-2">
                        <span>{"<"} BACK_TO_DASHBOARD</span>
                    </Link>
                    <div className="text-right">
                        <h2 className="text-2xl font-black italic tracking-tighter">TECHNICAL_DEEP_DIVE</h2>
                        <p className="text-[10px] font-mono opacity-50 uppercase tracking-[0.3em]">Module: Detailed_Breakdown_v1.0</p>
                    </div>
                </div>

                {/* Modular Schematic Section */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-16">
                    {/* Engine Module */}
                    <motion.div
                        className="glass p-8 cyber-border group hover:bg-[var(--theme-primary)]/5 transition-colors"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.1 }}
                    >
                        <div className="text-[var(--theme-primary)] mb-6">
                            <svg className="w-12 h-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                            </svg>
                        </div>
                        <h3 className="text-xl font-bold mb-4 tracking-widest uppercase">H4_ENGINE</h3>
                        <p className="text-xs font-mono opacity-80 leading-relaxed mb-6">
                            "The Muscle": A low-level Rust implementation utilizing Win32 APIs for total memory dominance.
                            Features a modular architecture for process snapshots and byte-pattern matching.
                        </p>
                        <div className="space-y-2 border-t border-white/10 pt-4">
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Kernel_Proxy:</span>
                                <span className="text-green-500">ACTIVE</span>
                            </div>
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Memory_Skip:</span>
                                <span className="text-yellow-500">82% EFFICIENCY</span>
                            </div>
                        </div>
                    </motion.div>

                    {/* Vision Module */}
                    <motion.div
                        className="glass p-8 cyber-border group hover:bg-[var(--theme-primary)]/5 transition-colors"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.2 }}
                    >
                        <div className="text-[var(--theme-primary)] mb-6">
                            <svg className="w-12 h-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                            </svg>
                        </div>
                        <h3 className="text-xl font-bold mb-4 tracking-widest uppercase">H4_VISION</h3>
                        <p className="text-xs font-mono opacity-80 leading-relaxed mb-6">
                            "The Brains": Driven by the Slint UI framework to provide industrial-grade visual feedback.
                            Implements multi-axial tearing and chromatic aberration for a cinematic operator experience.
                        </p>
                        <div className="space-y-2 border-t border-white/10 pt-4">
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Renderer:</span>
                                <span className="text-blue-500">GPU_ACCEL</span>
                            </div>
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Glitch_Engine:</span>
                                <span className="text-[var(--theme-primary)]">SYNCHRONIZED</span>
                            </div>
                        </div>
                    </motion.div>

                    {/* Shared Module */}
                    <motion.div
                        className="glass p-8 cyber-border group hover:bg-[var(--theme-primary)]/5 transition-colors"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.3 }}
                    >
                        <div className="text-[var(--theme-primary)] mb-6">
                            <svg className="w-12 h-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M13 10V3L4 14h7v7l9-11h-7z" />
                            </svg>
                        </div>
                        <h3 className="text-xl font-bold mb-4 tracking-widest uppercase">H4_SHARED</h3>
                        <p className="text-xs font-mono opacity-80 leading-relaxed mb-6">
                            "The DNA": Centralized synchronization layer. Manages the dual-thread integrity between
                            the Rust memory logic and the visual UI callbacks.
                        </p>
                        <div className="space-y-2 border-t border-white/10 pt-4">
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Sync_Heartbeat:</span>
                                <span className="text-green-500">LIVE</span>
                            </div>
                            <div className="flex justify-between text-[10px] font-mono">
                                <span>Packet_Loss:</span>
                                <span className="text-[var(--theme-primary)]">0.00ms</span>
                            </div>
                        </div>
                    </motion.div>
                </div>

                {/* Feature Breakdown Sections */}
                <div className="space-y-12 mb-24">
                    <section className="glass p-8 md:p-12 cyber-border">
                        <h2 className="text-3xl font-black mb-8 italic text-[var(--theme-primary)]">01_CORE_DISCOVERY_ENGINE</h2>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
                            <div className="space-y-6">
                                <h4 className="text-sm font-mono uppercase tracking-widest border-l-2 border-[var(--theme-primary)] pl-4">AOB Heuristics (60% Behavioral Score)</h4>
                                <p className="text-sm font-mono opacity-70 leading-relaxed">
                                    Unlike traditional scanners, H4 uses behavioral pattern matching to identify critical memory nodes.
                                    By analyzing how data structures change in real-time, we achieve a 60% success rate on initial AOB
                                    discovery for obfuscated variables.
                                </p>
                                <div className="p-4 bg-white/5 border border-white/10 font-mono text-[10px] italic opacity-50">
                                    Note: Obfuscated memory blocks still present challenges. Behavioral scoring is currently
                                    a manual assist tool rather than a fully automated bypass.
                                </div>
                            </div>
                            <div className="space-y-6">
                                <h4 className="text-sm font-mono uppercase tracking-widest border-l-2 border-[var(--theme-primary)] pl-4">Fast Path Algorithm</h4>
                                <p className="text-sm font-mono opacity-70 leading-relaxed">
                                    Our proprietary Fast Path logic enables high-speed memory skipping by filtering out non-executable blocks
                                    and focusing exclusively on RWX (Read/Write/Execute) regions mapped to the target process.
                                </p>
                                <ul className="space-y-2 text-xs font-mono">
                                    <li className="flex gap-2 text-green-400"><span>[SUCCESS]</span> <span>Filtered 1.2GB of junk memory in 0.04ms.</span></li>
                                    <li className="flex gap-2 text-[var(--theme-primary)] opacity-50"><span>[WARN]</span> <span>Failed to resolve pointer chain at offset 0x4F.</span></li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <section className="glass p-8 md:p-12 cyber-border">
                        <h2 className="text-3xl font-black mb-8 italic text-[var(--theme-primary)]">02_STABILITY_PROTOCOLS</h2>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
                            <div className="space-y-6 text-right">
                                <h4 className="text-sm font-mono uppercase tracking-widest border-r-2 border-[var(--theme-primary)] pr-4 italic">Nuclear Debugging Protocol</h4>
                                <p className="text-sm font-mono opacity-70 leading-relaxed">
                                    Every operation in H4 is wrapped in a stability interlock. If the discovery engine returns more than
                                    1,000 matches, the "Match Cap" terminates the cycle to prevent process locking and UI degradation.
                                </p>
                                <div className="p-4 bg-white/5 border border-white/10 font-mono text-[10px] text-right">
                                    <span className="text-red-500 animate-pulse uppercase tracking-widest">CRITICAL: Stability Interlock at 85% reliability.</span>
                                </div>
                            </div>
                            <div className="space-y-6">
                                <h4 className="text-sm font-mono uppercase tracking-widest border-l-2 border-[var(--theme-primary)] pl-4">Interactive System Terminal</h4>
                                <div className="bg-[var(--theme-bg)]/80 p-6 border border-white/10 rounded-sm font-mono text-[11px] space-y-2 relative overflow-hidden h-48 group">
                                    <div className="absolute top-0 right-0 p-2 text-[8px] opacity-20">SIM_V1</div>
                                    <motion.p
                                        initial={{ opacity: 0 }}
                                        animate={{ opacity: 1 }}
                                        transition={{ repeat: Infinity, duration: 1, repeatDelay: 0.5 }}
                                        className="text-green-500"
                                    >
                                        {">"} h4 --simulate-integrity-check
                                    </motion.p>
                                    <p className="text-blue-400">{">"} Initializing Win32 Snapshot...</p>
                                    <p className="text-[var(--theme-primary)]">{">"} Found 483 Active Modules.</p>
                                    <p className="text-[var(--theme-primary)]">{">"} Integrity Status: 85% STABLE (Buffer Load: 0.12%)</p>
                                    <p className="text-[var(--theme-primary)] animate-pulse">{">"} _</p>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>

                {/* Closing CTA */}
                <div className="text-center pb-24 border-t border-white/10 pt-16">
                    <Link
                        to="/"
                        className="inline-block px-12 py-4 bg-[var(--theme-primary)] text-[var(--theme-bg)] font-black uppercase tracking-[0.4em] hover:scale-110 hover:-rotate-1 transition-all active:scale-95 no-underline"
                    >
                        RETURN_TO_STATION
                    </Link>
                    <p className="mt-8 text-[10px] font-mono opacity-30 tracking-widest uppercase italic">
                        "You're only at your best, when you've been through the worst" — H4
                    </p>
                </div>
            </div>

            {/* Side Accents */}
            <div className="fixed top-1/2 left-0 -translate-y-1/2 p-4 z-0 pointer-events-none opacity-10">
                <div className="h-64 w-[1px] bg-[var(--theme-primary)] mb-4" />
                <div className="h-32 w-[1px] bg-[var(--theme-primary)]" />
            </div>
            <div className="fixed top-1/2 right-0 -translate-y-1/2 p-4 z-0 pointer-events-none opacity-10">
                <div className="h-32 w-[1px] bg-[var(--theme-primary)] mb-4" />
                <div className="h-64 w-[1px] bg-[var(--theme-primary)]" />
            </div>
        </div>
    );
};

export default DetailedBreakdown;
