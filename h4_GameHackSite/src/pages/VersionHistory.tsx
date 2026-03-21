import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Link } from 'react-router-dom';

interface Commit {
    sha: string;
    commit: {
        message: string;
        author: {
            name: string;
            date: string;
        };
    };
    html_url: string;
}

export const VersionHistory = () => {
    const [commits, setCommits] = useState<Commit[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchCommits = async () => {
            try {
                const response = await fetch('https://api.github.com/repos/m3rr/h4_GameHack/commits?per_page=10');
                if (!response.ok) throw new Error('Failed to fetch from logic core.');
                const data = await response.json();
                setCommits(data);
            } catch (err: any) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        fetchCommits();
    }, []);

    return (
        <div className="min-h-screen p-4 md:p-12 relative overflow-hidden bg-[var(--theme-bg)] text-[var(--theme-primary)]">
            {/* HUD Backdrop */}
            <div className="fixed inset-0 pointer-events-none opacity-5">
                <div className="absolute top-0 right-0 w-full h-[1px] bg-[var(--theme-primary)] animate-scanline" />
            </div>

            <div className="max-w-4xl mx-auto z-10 relative">
                <div className="flex justify-between items-center mb-12 border-b border-[var(--theme-primary)]/10 pb-6">
                    <Link to="/" className="text-[var(--theme-primary)] font-mono text-xs hover:opacity-100 transition-opacity no-underline flex items-center gap-2">
                        <span>{"<"} BACK_TO_INTERFACE</span>
                    </Link>
                    <div className="text-right">
                        <h2 className="text-3xl font-black italic tracking-tighter">SOFTWARE_CHANGELOG</h2>
                        <p className="text-[10px] font-mono opacity-50 uppercase tracking-[0.3em]">Source: GitHub_m3rr_h4_GameHack</p>
                    </div>
                </div>

                <div className="space-y-6">
                    {loading && (
                        <div className="flex flex-col items-center justify-center p-20 gap-4 opacity-30">
                            <div className="w-12 h-1 bg-[var(--theme-primary)]/20 relative overflow-hidden">
                                <motion.div
                                    className="absolute inset-0 bg-[var(--theme-primary)]"
                                    initial={{ x: '-100%' }}
                                    animate={{ x: '100%' }}
                                    transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                                />
                            </div>
                            <span className="text-[10px] font-mono uppercase tracking-[0.3em]">Synchronizing with repository DNA...</span>
                        </div>
                    )}

                    {error && (
                        <div className="glass p-8 border-red-500/50 text-red-500 font-mono text-center">
                            <p className="text-sm">PROTOCOL_ERROR: {error}</p>
                            <button
                                onClick={() => window.location.reload()}
                                className="mt-4 px-6 py-2 border border-red-500/30 text-[10px] hover:bg-red-500/10 transition-colors uppercase"
                            >
                                Retry Handshake
                            </button>
                        </div>
                    )}

                    <AnimatePresence>
                        {!loading && !error && commits.map((item, i) => (
                            <motion.a
                                key={item.sha}
                                href={item.html_url}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="glass p-6 cyber-border block group hover:bg-[var(--theme-primary)]/[0.03] transition-all no-underline overflow-hidden relative"
                                initial={{ opacity: 0, x: -20 }}
                                animate={{ opacity: 1, x: 0 }}
                                transition={{ delay: i * 0.1 }}
                            >
                                <div className="flex flex-col md:flex-row md:items-start justify-between gap-4">
                                    <div className="flex-1 space-y-2">
                                        <div className="flex items-center gap-3">
                                            <span className="text-[10px] bg-[var(--theme-primary)] text-[var(--theme-bg)] px-2 py-0.5 font-bold uppercase tracking-widest">
                                                COMMIT_{item.sha.substring(0, 7)}
                                            </span>
                                            <span className="text-[10px] font-mono opacity-40 uppercase tracking-widest">
                                                {new Date(item.commit.author.date).toLocaleDateString()}
                                            </span>
                                        </div>
                                        <p className="text-sm font-mono text-[var(--theme-primary)]/90 group-hover:text-[var(--theme-primary)] transition-colors line-clamp-2">
                                            {item.commit.message}
                                        </p>
                                    </div>
                                    <div className="text-right">
                                        <span className="text-[10px] font-mono opacity-50 block group-hover:opacity-100 transition-opacity uppercase">
                                            Authored By: {item.commit.author.name}
                                        </span>
                                        <span className="text-[9px] text-[var(--theme-primary)] opacity-0 group-hover:opacity-100 transition-opacity">
                                            RESOLVE_GTS {">"}
                                        </span>
                                    </div>
                                </div>

                                {/* Aesthetic Detail */}
                                <div className="absolute bottom-0 right-0 w-24 h-[1px] bg-[var(--theme-primary)]/20" />
                                <div className="absolute top-0 right-0 w-[1px] h-4 bg-[var(--theme-primary)]/20" />
                            </motion.a>
                        ))}
                    </AnimatePresence>
                </div>

                {/* Footer info */}
                <div className="mt-12 text-center opacity-30 font-mono text-[9px] uppercase tracking-[0.4em] pb-32">
                    End of dynamic data stream // h4_integ_version_v1.0
                </div>
            </div>

            {/* Grid Accents */}
            <div className="fixed top-0 left-0 w-32 h-32 border-t border-l border-[var(--theme-primary)]/5 pointer-events-none" />
            <div className="fixed bottom-0 right-0 w-32 h-32 border-b border-r border-[var(--theme-primary)]/5 pointer-events-none" />
        </div>
    );
};

export default VersionHistory;
