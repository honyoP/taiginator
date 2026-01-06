![taiga_banner](https://github.com/user-attachments/assets/5ff181bd-05c2-454a-a2d0-1c3e91eeb19c)


# Taiga

> **CodeName: YATTA (Yet Another Terminal Task App)**

A task organizer for the "mentally deficit monkey" in all of us. 🐒

**Taiga** is a CLI task manager built in Rust. It does one thing, and it does it fast: it manages your tasks without forcing you to leave the terminal or wait for a heavy web app to load.

If you like **Vim**, **Markdown**, and **not using a mouse**, you're home.

---

## 🤷 Why?

I got tired of opening a browser tab just to write down "Buy Milk."

I wanted a tool that:

1. **Starts instantly.** (No Electron bloat).
2. **Stores data in plain text.** (I want to own my data, not lock it in a database).
3. **Doesn't judge me** for having 50 overdue tasks.

## ✨ Features

* **⚡ Blazingly Fast:** It's Rust. It finishes executing before your finger leaves the Enter key.
* **📄 Plain Text Storage:** Tasks are saved in a `.md` file. You can `cat` it, `grep` it, or edit it manually if you're brave.
* **🧠 Human Scheduling:** Understands "tomorrow", "next friday", and "2024-01-01".
* **🛡️ ID-Based:** Every task gets a unique ID. No ambiguities.
* **🦾 Regex Powered:** Uses a custom regex parser to read your markdown file, because XML parsers are for cowards.

---

## 📦 Installation

### Option 1: Pre-compiled Binaries (Easiest)

Go to the [Releases Page](https://github.com/honyoP/taiga/releases) and grab the binary for your OS.

**Linux / Mac:**

```bash
chmod +x taiga
mv taiga /usr/local/bin/

```

### Option 2: Build from Source (For the cool kids)

You need [Rust](https://www.rust-lang.org/) installed.

```bash
git clone https://github.com/YOUR_USERNAME/taiga.git
cd taiga
cargo install --path .

```

---

## 🎮 Usage

Taiga uses a natural subcommand structure.

### 1. Add a Task

Just type.

```bash
taiga add "Fix the production bug"

```

**With a Schedule:**
Use the `when` keyword to attach a date.

```bash
taiga add "Buy groceries" when "tomorrow"
taiga add "Submit report" when "next friday"

```

### 2. List Tasks

See what you've been putting off.

```bash
taiga list           # Show all tasks
taiga list open      # Show only incomplete tasks
taiga list done      # Show completed tasks

```

*Output:*

```text
[ID:1] - [ ] Fix the production bug
[ID:2] - [ ] Buy groceries (Scheduled: 2024-03-20)

```

### 3. Get Stuff Done

Mark a task as complete using its **ID**.

```bash
taiga check 2

```

### 4. Nuke It

Delete a task forever.

```bash
taiga remove 1

```

---

## ⚙️ Under the Hood

Taiga stores your tasks in a simple Markdown file located in your system's default data directory (managed by `confy`).

* **Linux:** `~/.config/taiginator/taiga.md` (or similar)
* **Mac:** `~/Library/Application Support/rs.taiginator/taiga.md`
* **Windows:** `%APPDATA%\taiginator\taiga.md`

Because it's just a file, you can back it up with Git, sync it via Dropbox, or print it out and eat it.

## 🛠 Building & Contributing

Found a bug? Want to add a feature?
PRs are welcome. Just please run `cargo fmt` before you push or the CI will yell at you.

```bash
# Run locally
cargo run -- add "Test task"

# Run tests
cargo test

```

## 📜 License

MIT. Do whatever you want with it. Just don't blame me if you miss your dentist appointment.
