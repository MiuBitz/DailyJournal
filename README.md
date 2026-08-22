# 📓 Daily Journal v2 (Windows)

> A fast, lightweight Windows background utility for structured daily Markdown journaling. Built with **Tauri v2 (Rust + Web UI)**.

![Windows](https://img.shields.io/badge/OS-Windows-blue?style=flat-square&logo=windows)
![Rust](https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust)
![Tauri](https://img.shields.io/badge/Framework-Tauri_v2-24C8D8?style=flat-square&logo=tauri)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)

---

## ✨ Features

- ⚡ **Ultra-Lightweight Performance**: Consumes **0.0% CPU** when idle and uses only **~18 MB RAM**.
- 📌 **Native System Tray Integration**: Minimizes silently to the system tray. Context menu supports *Open Journal*, *Write Current Block*, *Open Journal Folder*, and *Exit*.
- ⏰ **Customizable Reminder Times**: Set personalized 24-hour schedules (`HH:MM`) for **Morning**, **Afternoon**, and **Evening** check-ins.
- 📝 **Clean Markdown File Generator**: Automatically creates and opens structured Markdown files organized by block and date (`YYYY-MM-DD`).
- 🚀 **Silent Windows Startup**: Option to automatically launch with Windows silently in the tray without console window popups.
- 🎨 **Modern Dark UI**: Built with a sleek dark card aesthetic (`#12141A` background, indigo accents `#6366F1`, and green status badges).

---

## 🛠 Tech Stack

- **Backend**: Rust 2021 + Tauri v2 (`tauri`, `chrono`, `open`, `windows-sys`, `rfd`)
- **Frontend**: HTML5, CSS3 (Vanilla), JavaScript (WebView2)
- **Audio & Tray**: Windows Multimedia Sound API + Native Win32 Tray Icon

---

## 📂 Project Structure

```
Miu_Daily_Journal_v2_Windows/
├── src/                 # Rust backend logic
│   ├── lib.rs           # Tauri IPC commands & system tray setup
│   ├── config.rs        # App configuration & JSON persistence
│   ├── journal.rs       # Markdown template generator & file opening logic
│   ├── sound.rs         # Audio reminder alert triggers
│   └── startup.rs       # Windows autostart shortcut management
├── ui/                  # Web frontend assets
│   ├── index.html       # Main UI structure & modal containers
│   ├── style.css        # Modern dark theme styles
│   └── app.js           # Frontend logic & Tauri IPC bindings
├── icons/               # Multi-resolution application & tray icons
├── dist/                # Pre-built standalone executable
│   └── DailyJournal.exe # Single-file production binary
├── tauri.conf.json      # Tauri v2 application configuration
├── build_windows.bat    # One-click Windows build script
└── Cargo.toml           # Rust dependencies & manifest
```

---

## 🚀 Getting Started

### Method 1: Use Pre-built Executable
Simply run the standalone executable located at:
```
dist/DailyJournal.exe
```

### Method 2: Build From Source

#### Prerequisites
1. Install [Rust](https://www.rust-lang.org/tools/install) (with `x86_64-pc-windows-msvc` toolchain).
2. Install C++ Build Tools (included with Visual Studio or Build Tools for Visual Studio).

#### Building
Run the one-click build script:
```cmd
build_windows.bat
```

Or build manually via Cargo:
```cmd
cargo build --release
```
The compiled executable will be output to `dist/DailyJournal.exe`.

---

## 📖 Usage Guide

1. **Write Entry**: Click **✏ Write Journal** on any block card to open/create today's Markdown check-in file in your default Markdown editor (e.g., Obsidian, VS Code, Notepad).
2. **Batch Create**: Click **📝 Create Today's Files** to generate all 3 Markdown check-in files for today at once.
3. **Customize Schedule**: Open **⚙ Settings** to edit your preferred check-in reminder times (`12:00`, `18:00`, `22:00`) or toggle Windows Autostart.
4. **System Tray**: Closing the window hides the app to the system tray. Left-click the tray icon to restore the window.

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.
