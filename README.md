# 🛰️ H4_GameHack // MEM_HACK_ENGINE

Welcome to the inner circle. We got tired of boring, crash-heavy grey windows, so we built a cyberpunk memory engine with actual soul. This isn't just a scanner. It is a cinematic experience built for stability, speed, and absolute control. 

Whether you are trying to give yourself infinite health to beat a punishing boss, or you need to troubleshoot a locked file deep in your system, H4 has you covered.

## 🛠️ The Architecture
Under the hood, we split the workload into three distinct nodes so the UI never blocks the logic, and the logic never crashes your screen.
* **h4_engine**: The muscle. It handles raw Win32 memory access and pattern matching.
* **h4_vision**: The brains. Built on Slint, this manages the cinematic UI, the multi-axial tearing glitch effects, and all your interactions.
* **h4_shared**: The DNA that lets teh engine and the vision communicate seamlessly.

## 🕹️ The Operator's Manual (How to use this thing)

We designed the interface to be dangerously intuitive. Here is what every button does and when you should use it.

### 1. The Process Selector (The Neon Bar)
* **What it does**: This is your entry point. It lists all running processes.
* **Use Case**: You boot up your favorite single-player RPG. You open H4, type the game's name in the bar, and hook into its memory. You are now in the matrix.

### 2. Smart Scan (Targeted Heuristics)
* **What it does**: Instead of making you hunt blindly for a specific number, this button looks for known behavior patterns (AOBs) commonly associated with Health, Mana, Money, or Stats.
* **Use Case**: You are out of healing potions and the boss is about to end your run. You hit Smart Scan, find your health pool automatically, and lock it at 999. 

### 3. Manual Scan (The TRACK Drawer)
* **What it does**: For when you know exactly what value you are looking for. It uses our Fast Path algorithm to skip over 99% of non-matching memory in milliseconds. It really bytes when you have to wait for slow scans to finish (heh <_<). 
* **Use Case**: You want to find exactly 452 gold coins. You searh 452, spend a few coins in-game, and scan the new value until you isolate the exact memory address. 

### 4. The STOP Button (The Kill Switch)
* **What it does**: Instantly cancels any active scan mid-flight. 
* **Use Case**: You accidentally searched for a value of "1" across 32GB of RAM. Instead of letting the app freeze and crash your rig, you just hit STOP. Crisis averted.

### 5. DEBUG: ON (H4_System_Terminal)
* **What it does**: Opens our built-in forensic console. You can watch the engine working in real-time and export beautifully formatted Markdown reports of your session.
* **Use Case**: A game updates and your old memory pointers break. You turn on the terminal to see exactly where the engine is failing, making you feel like an elite operator fixing the code.

### 6. Theme Selector
* **What it does**: Changes the entire aesthetic soul of the app. Your choice automatically persists for the next time you boot up.
* **Use Case**: You want your tools to match your mood. Choose *Nomad* for a gritty industrial look, *Corpo* for sterile white-collar efficiency, or *NetRunner* for pure, high-contrast neon. 

## ⚖️ The Legal Shit (Read This)

Let's get the serious stuff out of the way. 

This software is provided **"as is"**, without warranty of any kind, express or implied. By using H4_GameHack, you accept full responsibility for whatever happens next. 

If you accidentally target a critical system PID and blue-screen your rig, that is on you. If you try to use this in a multiplayer game with anti-cheat and get your account permanently banned, that is also entirely on you. We are building a sandbox tool for single-player manipulation and system troubleshooting. We assume absolutely zero liability for broken saves, corrupted files, or crying CPUs. You are the operator. Act like one.

This project is licensed under the **GPL v3**. It is open and it is free, but you must respect the license if you decide to fork or distribute it.

## 🚀 Status
This is definately a Beta build. It is spicy, it is experimental, and we are adding features constantly. If you find a bug, let me know! 

Stay glitchy.

Be Your Best 
   - h4 
