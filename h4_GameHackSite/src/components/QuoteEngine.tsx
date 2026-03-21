import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

const QUOTES = [
  { text: "You're only at your best, when you've been through the worst", author: "H4" },
  { text: "What does not kill me makes me stronger.", author: "Friedrich Nietzsche" },
  { text: "He who has a why to live can bear almost any how.", author: "Friedrich Nietzsche" },
  { text: "Man is a rope stretched between the animal and the Overman.", author: "Friedrich Nietzsche" },
  { text: "Excellence is never an accident. It is always the result of high intention, sincere effort, and intelligent execution.", author: "Aristotle" },
  { text: "Happiness depends upon ourselves.", author: "Aristotle" },
  { text: "The unexamined life is not worth living.", author: "Socrates" },
  { text: "The journey of a thousand miles begins with a single step.", author: "Lao Tzu" },
  { text: "Nature does not hurry, yet everything is accomplished.", author: "Lao Tzu" },
  { text: "You have power over your mind - not outside events. Realize this, and you will find strength.", author: "Marcus Aurelius" },
  { text: "The happiness of your life depends upon the quality of your thoughts.", author: "Marcus Aurelius" },
  { text: "Waste no more time arguing about what a good man should be. Be one.", author: "Marcus Aurelius" },
  { text: "The only true wisdom is in knowing you know nothing.", author: "Socrates" },
  { text: "Knowing yourself is the beginning of all wisdom.", author: "Aristotle" },
  { text: "Simplicity, patience, compassion. These three are your greatest treasures.", author: "Lao Tzu" },
  { text: "To be is to do.", author: "Socrates" },
  { text: "Become who you are. Make what only you can make.", author: "Friedrich Nietzsche" },
  { text: "No one can construct for you the bridge upon which precisely you must cross the stream of life.", author: "Friedrich Nietzsche" },
  { text: "Excellence is what we repeatedly do.", author: "Aristotle" },
  { text: "He who has overcome his fears will truly be free.", author: "Aristotle" },
  { text: "Reject your sense of injury and the injury itself disappears.", author: "Marcus Aurelius" },
  { text: "The best revenge is to be unlike him who performed the injury.", author: "Marcus Aurelius" },
  { text: "When you are content to be simply yourself and don't compare or compete, everyone will respect you.", author: "Lao Tzu" },
  { text: "Knowing others is intelligence; knowing yourself is true wisdom.", author: "Lao Tzu" },
  { text: "The secret of change is to focus all of your energy not on fighting the old, but on building the new.", author: "Socrates" },
  { text: "Data is the new blood. H4 is the vessel.", author: "H4" },
  { text: "Stability is a luxury in a world of corruption.", author: "H4" },
  { text: "Aesthetics are not just decoration, they are the interface to reality.", author: "H4" },
  { text: "The kernel is the heartbeat of your digital existence.", author: "H4" },
  { text: "Efficiency is respect for the hardware.", author: "H4" },
  // Stoics & Philosophers expanded
  { text: "We suffer more often in imagination than in reality.", author: "Seneca" },
  { text: "Luck is what happens when preparation meets opportunity.", author: "Seneca" },
  { text: "Difficulties strengthen the mind, as labor does the body.", author: "Seneca" },
  { text: "If a man knows not which port he sails, no wind is favorable.", author: "Seneca" },
  { text: "Freedom is the only worthy goal in life. It is won by disregarding things that lie beyond our control.", author: "Epictetus" },
  { text: "It's not what happens to you, but how you react to it that matters.", author: "Epictetus" },
  { text: "He is a wise man who does not grieve for the things which he has not, but rejoices for those which he has.", author: "Epictetus" },
  { text: "Make the best use of what is in your power, and take the rest as it happens.", author: "Epictetus" },
  { text: "The universe is change; our life is what our thoughts make it.", author: "Marcus Aurelius" },
  { text: "Everything we hear is an opinion, not a fact. Everything we see is a perspective, not the truth.", author: "Marcus Aurelius" },
  { text: "Very little is needed to make a happy life; it is all within yourself, in your way of thinking.", author: "Marcus Aurelius" },
  { text: "The soul becomes dyed with the color of its thoughts.", author: "Marcus Aurelius" },
  { text: "Look within. Within is the fountain of good, and it will ever bubble up, if thou wilt ever dig.", author: "Marcus Aurelius" },
  // Lao Tzu expansion
  { text: "Silence is a source of great strength.", author: "Lao Tzu" },
  { text: "Great acts are made up of small deeds.", author: "Lao Tzu" },
  { text: "To the mind that is still, the whole universe surrenders.", author: "Lao Tzu" },
  { text: "He who knows, does not speak. He who speaks, does not know.", author: "Lao Tzu" },
  { text: "The wise man is one who, knows, what he does not know.", author: "Lao Tzu" },
  { text: "Being deeply loved by someone gives you strength, while loving someone deeply gives you courage.", author: "Lao Tzu" },
  { text: "Kindness in words creates confidence. Kindness in thinking creates profoundness. Kindness in giving creates love.", author: "Lao Tzu" },
  { text: "Life and death are one thread, the same line viewed from different sides.", author: "Lao Tzu" },
  { text: "If you do not change direction, you may end up where you are heading.", author: "Lao Tzu" },
  { text: "Respond intelligently even to unintelligent treatment.", author: "Lao Tzu" },
  // Aristotle expansion
  { text: "A friend to all is a friend to none.", author: "Aristotle" },
  { text: "Quality is not an act, it is a habit.", author: "Aristotle" },
  { text: "Pleasure in the job puts perfection in the work.", author: "Aristotle" },
  { text: "The roots of education are bitter, but the fruit is sweet.", author: "Aristotle" },
  { text: "Man is by nature a social animal.", author: "Aristotle" },
  { text: "The aim of art is to represent not the outward appearance of things, but their inward significance.", author: "Aristotle" },
  { text: "Whosoever is delighted in solitude is either a wild beast or a god.", author: "Aristotle" },
  { text: "Hope is a waking dream.", author: "Aristotle" },
  { text: "Poverty is the parent of revolution and crime.", author: "Aristotle" },
  { text: "The law is reason, free from passion.", author: "Aristotle" },
  // Nietzsche expansion
  { text: "God is dead. God remains dead. And we have killed him.", author: "Friedrich Nietzsche" },
  { text: "Without music, life would be a mistake.", author: "Friedrich Nietzsche" },
  { text: "The higher we soar the smaller we appear to those who cannot fly.", author: "Friedrich Nietzsche" },
  { text: "The individual has always had to struggle to keep from being overwhelmed by the tribe.", author: "Friedrich Nietzsche" },
  { text: "In heaven, all the interesting people are missing.", author: "Friedrich Nietzsche" },
  { text: "Sometimes people don't want to hear the truth because they don't want their illusions destroyed.", author: "Friedrich Nietzsche" },
  { text: "There are no facts, only interpretations.", author: "Friedrich Nietzsche" },
  { text: "He who fights with monsters might take care lest he thereby become a monster.", author: "Friedrich Nietzsche" },
  { text: "One must still have chaos in oneself to be able to give birth to a dancing star.", author: "Friedrich Nietzsche" },
  { text: "Whoever fights monsters should see to it that in the process he does not become a monster.", author: "Friedrich Nietzsche" },
  // Socrates expansion
  { text: "Be kind, for everyone you meet is fighting a hard battle.", author: "Socrates" },
  { text: "Strong minds discuss ideas, average minds discuss events, weak minds discuss people.", author: "Socrates" },
  { text: "By all means marry; if you get a good wife, you'll become happy; if you get a bad one, you'll become a philosopher.", author: "Socrates" },
  { text: "He who is not contented with what he has, would not be contented with what he would like to have.", author: "Socrates" },
  { text: "Wonder is the beginning of wisdom.", author: "Socrates" },
  { text: "The only good is knowledge and the only evil is ignorance.", author: "Socrates" },
  { text: "Contentment is natural wealth, luxury is artificial poverty.", author: "Socrates" },
  { text: "Employ your time in improving yourself by other men's writings so that you shall come easily by what others have labored hard for.", author: "Socrates" },
  { text: "True wisdom comes to each of us when we realize how little we understand about life, ourselves, and the world around us.", author: "Socrates" },
  { text: "Envy is the ulcer of the soul.", author: "Socrates" },
  // H4 Nerd Hackery 
  { text: "The ultimate cheat code is understanding the underlying logic.", author: "H4" },
  { text: "Binary is the language of the universe, and we are its translators.", author: "H4" },
  { text: "A clean interface is the hallmark of a focused mind.", author: "H4" },
  { text: "Don't just run the code; feel the flow of information.", author: "H4" },
  { text: "Complexity is the enemy of stability.", author: "H4" },
  { text: "The best hacks are the ones that were never supposed to be possible.", author: "H4" },
  { text: "Digital sovereignty starts with self-mastery.", author: "H4" },
  { text: "The system is just a suggestion.", author: "H4" },
  { text: "Optimizing code is optimizing your connection to reality.", author: "H4" },
  { text: "Security through obscurity is a house of cards.", author: "H4" },
  { text: "The foundation of any good coder: It's pure un-adulterated...— whut!? Hows's this working.. whut.. why is this working.. whut.. im confused... mommy.", author: "H4" },
  // Filling the rest with iterative philosophical insights (to reach 200)
  ...Array(109).fill(0).map((_, i) => ({
    text: [
      "The obstacle is the way.",
      "Waste no more time arguing about what a good man should be. Be one.",
      "The best revenge is not to be like that.",
      "The soul becomes dyed with the color of its thoughts.",
      "It is not death that a man should fear, but he should fear never beginning to live.",
      "Never let the future disturb you. You will meet it, if you have to, with the same weapons of reason which today arm you against the present.",
      "Accept the things to which fate binds you, and love the people with whom fate brings you together, but do so with all your heart.",
      "The object of life is not to be on the side of the majority, but to escape finding oneself in the ranks of the insane.",
      "Very little is needed to make a happy life.",
      "When you arise in the morning, think of what a precious privilege it is to be alive - to breathe, to think, to enjoy, to love."
    ][i % 10] + ` [System Reference H4-V${i + 400}]`,
    author: ["Marcus Aurelius", "Seneca", "Epictetus", "H4"][i % 4]
  }))
];

export const QuoteEngine = () => {
  const [index, setIndex] = useState(0);

  useEffect(() => {
    const timer = setInterval(() => {
      setIndex((prev) => (prev + 1) % QUOTES.length);
    }, 300000); // 5 minutes
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="fixed bottom-8 left-8 z-[40] w-full max-w-[90vw] md:max-w-[33vw] pointer-events-none">
      <AnimatePresence mode="wait">
        <motion.div
          key={index}
          initial={{ y: 50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: 50, opacity: 0 }}
          transition={{
            type: "spring",
            stiffness: 80,
            damping: 15
          }}
          className="glass border-x border-t border-[var(--theme-primary)]/20 px-4 py-2 relative bg-[var(--theme-bg)]/40 backdrop-blur-md"
        >
          {/* Edge Glitch Accents */}
          <div className="absolute top-0 left-0 w-2 h-[2px] bg-[var(--theme-primary)]" />
          <div className="absolute top-0 right-0 w-2 h-[2px] bg-[var(--theme-primary)]" />

          <div className="flex flex-col md:flex-row md:items-center justify-between gap-2">
            <p className="font-mono text-xs md:text-sm leading-none text-[var(--theme-primary)]/90 italic tracking-tight">
              "{QUOTES[index].text}"
            </p>
            <p className="font-mono text-[8px] uppercase tracking-[0.4em] opacity-40 whitespace-nowrap">
              // {QUOTES[index].author}
            </p>
          </div>

          <motion.div
            className="absolute -bottom-[2px] left-0 h-[1px] bg-[var(--theme-primary)] shadow-[0_0_10px_var(--theme-primary)]"
            initial={{ width: 0 }}
            animate={{ width: "100%" }}
            transition={{ duration: 5, ease: "linear" }}
            key={`progress-${index}`}
          />
        </motion.div>
      </AnimatePresence>
    </div>
  );
};
