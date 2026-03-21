import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

const KIRBY_VARIATIONS = [
  "(b ' . ' )b", "d( ' . ' d)", "t( -_- t)", "p( ' . ' p)", "┻━┻︵ヽ(`Д´)ﾉ︵ ┻━┻",
  "(ノಠ益ಠ)ノ", "⊂(◉‿◉)つ", "(づ｡◕‿‿◕｡)づ", "~(˘▾˘~)", "(っ◕‿◕)っ",
  "(╯°□°)╯︵ ┻━┻", "ᕙ(⇀‸↼)ᕗ", "o(╥﹏╥)o", "(*^‿^*)", "(◕‿◕✿)",
  "(∩｀-´)⊃━☆ﾟ.*･｡ﾟ", "(¬_¬)", "( ͡° ͜ʖ ͡°)", "ʕ•ᴥ•ʔ", "(ᵔᴥᵔ)",
  "p(^_^)q", "ᕦ(ò_óˇ)ᕤ", "(☞ﾟ∀ﾟ)☞", "☜(˚▽˚)☞", "v( ' . ' v)",
  "<(' . '<)", "(> ' . ')>", "^(' . ')^", "v(' . ')^", "(;´༎ຶД༎ຶ`)",
  "ಠ_ಠ", "¯\\_(ツ)_/¯", "凸(^_^)凸", "(⊙_⊙)", "(^_-)", "(o^^o)", "(*^^*)"
];

const THEME_NAMES = [
  "NOMAD_ORANGE", "CORPO_CYAN", "NET_RUNNER", "SPECTER_GOLD", "DECAY_RED",
  "PHANTOM_VIOLET", "CASCADE_AZURE", "NEON_WAVE", "MINT_REFRESH", "SAFFRON_GLOW",
  "ICE_BREAKER", "CRIMSON_VALLEY", "COBALT_STRIKE", "EMERALD_EYE", "ROSE_PROTOCOL",
  "ONYX_METALLIC", "TITAN_ELECTRUM", "AMBER_PULSE", "TEAL_TRAIL", "ORCHID_DEEP",
  "FORRESTER", "VOLCANO", "MIDNIGHT_LUSTRE", "SAKURA_VIBE", "GLACIER_SHARD",
  "SANDSTORM", "VOID_PHASE", "ELECTRON_BLUE", "PLASMA_GREEN", "BLIGHT_TOXIN",
  "MERCURY_FLOW", "OBSIDIAN_GREY", "QUARTZ_DUST", "TOPAZ_GLOW", "RUBY_CORR",
  "SAPPHIRE_HEART", "PYRITE_DUST"
];

// Curated colors for each of the 37 themes. Never using pure #000 or #FFF.
// Ensuring all 'Primary' colors are bright enough to be legible as text.
const THEME_CONFIGS = [
  { primary: "#cc8d00", bg: "#0f0900", accent: "rgba(204, 141, 0, 0.1)" }, // NOMAD
  { primary: "#00d1db", bg: "#02070a", accent: "rgba(0, 209, 219, 0.1)" }, // CORPO
  { primary: "#00ff88", bg: "#000804", accent: "rgba(0, 255, 136, 0.1)" }, // NETRUNNER
  { primary: "#ffd700", bg: "#0a0a00", accent: "rgba(255, 215, 0, 0.1)" }, // SPECTER
  { primary: "#ff4d4d", bg: "#0d0000", accent: "rgba(255, 77, 77, 0.1)" }, // DECAY
  { primary: "#a78bfa", bg: "#080612", accent: "rgba(167, 139, 250, 0.1)" }, // PHANTOM
  { primary: "#38bdf8", bg: "#020b12", accent: "rgba(56, 189, 248, 0.1)" }, // CASCADE
  { primary: "#f472b6", bg: "#0d0208", accent: "rgba(244, 114, 182, 0.1)" }, // NEON
  { primary: "#4ade80", bg: "#020d06", accent: "rgba(74, 222, 128, 0.1)" }, // MINT
  { primary: "#fbbf24", bg: "#0d0a02", accent: "rgba(251, 191, 36, 0.1)" }, // SAFFRON
  { primary: "#99f6e4", bg: "#020d0b", accent: "rgba(153, 246, 228, 0.1)" },
  { primary: "#ef4444", bg: "#0d0202", accent: "rgba(239, 68, 68, 0.1)" },
  { primary: "#3b82f6", bg: "#02060d", accent: "rgba(59, 130, 246, 0.1)" },
  { primary: "#10b981", bg: "#020d08", accent: "rgba(16, 185, 129, 0.1)" },
  { primary: "#ec4899", bg: "#0d0206", accent: "rgba(236, 72, 153, 0.1)" },
  { primary: "#a5b4fc", bg: "#040510", accent: "rgba(165, 180, 252, 0.1)" }, // Restored Visibility
  { primary: "#d1d5db", bg: "#0b0c0f", accent: "rgba(209, 213, 219, 0.1)" }, // Restored Visibility
  { primary: "#f59e0b", bg: "#0d0802", accent: "rgba(245, 158, 11, 0.1)" },
  { primary: "#14b8a6", bg: "#020d0b", accent: "rgba(20, 184, 166, 0.1)" },
  { primary: "#8b5cf6", bg: "#080612", accent: "rgba(139, 92, 246, 0.1)" },
  { primary: "#10b981", bg: "#020d08", accent: "rgba(16, 185, 129, 0.1)" },
  { primary: "#ff2e2e", bg: "#0d0202", accent: "rgba(255, 46, 46, 0.1)" },
  { primary: "#6366f1", bg: "#02020a", accent: "rgba(99, 102, 241, 0.1)" },
  { primary: "#fbcfe8", bg: "#0a0208", accent: "rgba(251, 207, 232, 0.1)" },
  { primary: "#7dd3fc", bg: "#020c15", accent: "rgba(125, 211, 252, 0.1)" },
  { primary: "#fcd34d", bg: "#0d0a02", accent: "rgba(252, 211, 77, 0.1)" },
  { primary: "#d8b4fe", bg: "#05020d", accent: "rgba(216, 180, 254, 0.1)" },
  { primary: "#38bdf8", bg: "#020a0d", accent: "rgba(56, 189, 248, 0.1)" },
  { primary: "#e879f9", bg: "#0a020d", accent: "rgba(232, 121, 249, 0.1)" },
  { primary: "#ff8484", bg: "#0d0202", accent: "rgba(255, 132, 132, 0.1)" },
  { primary: "#d4d4d8", bg: "#0a0a0d", accent: "rgba(212, 212, 216, 0.1)" },
  { primary: "#a1a1aa", bg: "#050508", accent: "rgba(161, 161, 170, 0.1)" }, // Restored Visibility
  { primary: "#e2e2e7", bg: "#0d0e11", accent: "rgba(226, 226, 231, 0.1)" },
  { primary: "#fde68a", bg: "#0d0d0a", accent: "rgba(253, 230, 138, 0.1)" },
  { primary: "#fecaca", bg: "#0d0a0a", accent: "rgba(254, 202, 202, 0.1)" },
  { primary: "#bfdbfe", bg: "#0a0c0f", accent: "rgba(191, 219, 254, 0.1)" },
  { primary: "#fef08a", bg: "#0d0d05", accent: "rgba(254, 240, 138, 0.1)" }
];

export const KirbySwitcher = () => {
  const [index, setIndex] = useState(0);

  const handleClick = () => {
    const nextIndex = (index + 1) % KIRBY_VARIATIONS.length;
    setIndex(nextIndex);

    const config = THEME_CONFIGS[nextIndex];
    const root = document.documentElement;
    root.style.setProperty('--theme-bg', config.bg);
    root.style.setProperty('--theme-primary', config.primary);
    root.style.setProperty('--theme-accent', config.accent);
  };

  return (
    <motion.div
      className="fixed bottom-6 right-6 z-50 cursor-pointer select-none text-right"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
      onClick={handleClick}
    >
      <div className="text-[var(--theme-primary)] font-mono text-xs whitespace-nowrap overflow-hidden mb-1 drop-shadow-[0_0_5px_var(--theme-primary)]/50">
        <AnimatePresence mode="wait">
          <motion.span
            key={index}
            initial={{ y: 2, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: -2, opacity: 0 }}
            transition={{ duration: 0.15 }}
            className="inline-block"
          >
            {KIRBY_VARIATIONS[index]}
          </motion.span>
        </AnimatePresence>
      </div>
      <div className="text-[8px] opacity-40 font-mono uppercase tracking-[0.15em]">
        PROTOCOL: {THEME_NAMES[index] || "UNKNOWN"}
      </div>
    </motion.div>
  );
};
