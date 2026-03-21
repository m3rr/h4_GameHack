import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

export const SplashScreen = ({ onComplete }: { onComplete: () => void }) => {
  const [step, setStep] = useState(0);

  useEffect(() => {
    const timer1 = setTimeout(() => setStep(1), 1500);
    const timer2 = setTimeout(() => setStep(2), 4000);
    const timer3 = setTimeout(() => onComplete(), 5500);

    return () => {
      clearTimeout(timer1);
      clearTimeout(timer2);
      clearTimeout(timer3);
    };
  }, [onComplete]);

  return (
    <motion.div
      className="fixed inset-0 bg-[var(--theme-bg)] z-[100] flex flex-col items-center justify-center p-8 text-center"
      initial={{ opacity: 1 }}
      exit={{ opacity: 0, scale: 1.1, filter: 'blur(20px)' }}
      transition={{ duration: 1 }}
    >
      <AnimatePresence mode="wait">
        {step === 0 && (
          <motion.div
            key="step0"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20, filter: 'hue-rotate(90deg)' }}
            className="space-y-4"
          >
            <h1 className="text-4xl md:text-6xl font-black text-[var(--theme-primary)] glitch-text tracking-[0.3em]">
              WELCOME
            </h1>
            <p className="text-gray-400 font-mono text-sm tracking-widest uppercase">
              Thank you for coming by.
            </p>
          </motion.div>
        )}

        {step === 1 && (
          <motion.div
            key="step1"
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, x: 100, filter: 'blur(10px)' }}
            className="max-w-2xl space-y-8 glass p-8 cyber-border"
          >
            <div className="space-y-2 border-b border-white/10 pb-4 text-left">
              <h2 className="text-xl font-bold text-[var(--theme-primary)] tracking-widest uppercase">
                H4 UTILITY PROTOCOL
              </h2>
              <div className="flex gap-2">
                <span className="h-1 w-12 bg-[var(--theme-primary)]/20" />
                <span className="h-1 w-4 bg-[var(--theme-primary)]" />
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
              <div className="space-y-1">
                <span className="text-[10px] text-gray-500 uppercase">01 / ENGINE</span>
                <p className="text-xs font-mono text-gray-300 italic">"The Muscle": Industrial Win32 memory access, process snapshots, and pattern matching.</p>
              </div>
              <div className="space-y-1">
                <span className="text-[10px] text-gray-500 uppercase">02 / VISION</span>
                <p className="text-xs font-mono text-gray-300 italic">"The Brains": Cinematic Slint UI with multi-axial tearing and chromatic aberration.</p>
              </div>
              <div className="space-y-1">
                <span className="text-[10px] text-gray-500 uppercase">03 / SHARED</span>
                <p className="text-xs font-mono text-gray-300 italic">"The DNA": Seamless synchronization between engine logic and visual feedback.</p>
              </div>
            </div>

            <p className="text-[10px] text-[var(--theme-primary)]/30 font-mono text-right italic">
              "You're only at your best, when you've been through the worst"
            </p>
          </motion.div>
        )}

        {step === 2 && (
          <motion.div
            key="step2"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="flex flex-col items-center gap-4"
          >
            <div className="w-64 h-1 bg-[var(--theme-primary)]/10 relative overflow-hidden">
              <motion.div
                className="absolute inset-0 bg-[var(--theme-primary)]"
                initial={{ x: '-100%' }}
                animate={{ x: '100%' }}
                transition={{ duration: 1.5, repeat: Infinity, ease: "linear" }}
              />
            </div>
            <span className="text-[10px] text-[var(--theme-primary)]/50 font-mono animate-pulse uppercase tracking-[0.5em]">
              INITIALIZING INTERFACE...
            </span>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Decorative lines */}
      <div className="fixed top-0 bottom-0 left-12 w-[1px] bg-[var(--theme-primary)]/5 pointer-events-none" />
      <div className="fixed top-0 bottom-0 right-12 w-[1px] bg-[var(--theme-primary)]/5 pointer-events-none" />
      <div className="fixed left-0 right-0 top-12 h-[1px] bg-[var(--theme-primary)]/5 pointer-events-none" />
      <div className="fixed left-0 right-0 bottom-12 h-[1px] bg-[var(--theme-primary)]/5 pointer-events-none" />
    </motion.div>
  );
};
