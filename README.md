# qn: Quick Note CLI & TUI in Rust

**A minimalist, terminal-based note-taking tool for markdown lovers, designed to create quick notes without friction—so you never have to leave your terminal.**

`qn` is a fast, keyboard-driven application for creating, managing, and editing markdown notes directly from your terminal. Whether you prefer a **CLI** for quick note creation or a **TUI** for interactive browsing and editing, `qn` keeps your thoughts organized and accessible.

## Features

✔ **CLI Mode**: Create markdown notes instantly with a single command.  
✔ **TUI Mode**: Navigate, search, edit, and preview notes in a user-friendly terminal interface.  
✔ **Markdown Support**: Write and render markdown with live previews (TUI).  
✔ **Lightweight & Fast**: Built in Rust for performance and efficiency.  
✔ **Cross-Platform**: Works on Linux, macOS, and Windows.

## Installation

```sh
cargo install qn
```

or build from source:

```sh
git clone https://github.com/JulesEfrei/qn
cd qn
cargo build --release
```

## Usage

### CLI

```sh
# Create a new note
qn new "My note title"

# List all notes
qn list
```

### TUI

```sh
qn
```

Use Vim motion (h,j,k,l) to navigate, `/` to search, `n` to create a new note and `Enter` to edit in your favorite editor.

## Why `qn`?

- **No distractions**: Focus on writing without leaving the terminal.
- **Offline-first**: Your notes stay local by default.
- **Extensible**: Easy to integrate with your workflow (e.g., sync with Git, cloud storage).

---

_Built with ❤️ in Rust. Contributions welcome!_
