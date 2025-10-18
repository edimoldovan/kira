# CLAUDE.md

Kira - email client written in Rust using the Iced framework.

## Commands

```bash
cargo build    # Build
cargo run      # Run
cargo check    # Check for errors
```

## Architecture

**Elm-style pattern**: `Message` enum → `update()` → `view()`

**Core modules**:
- `ui/` - Three components: sidebar, message_list, reader
- `email/` - Account models, IMAP sync, caching
- `config.rs` - Loads accounts from `~/.config/kira/accounts.toml`
- `main.rs` - App entry point, state management

**State** (in `main.rs:18-25`):
- Simple struct with accounts, indices, sync status
- No Rc/RefCell needed (Iced handles it)

**IMAP** (`email/imap.rs`):
- Async operations using `async-imap` and `async-std`
- Syncs last 100 messages on startup
- Fetches message bodies on-demand when clicked
- Messages cached locally as JSON

**Config** (`config.rs`):
- TOML format with IMAP credentials
- Falls back to example accounts if no config exists

**Message flow**: UI events → Message enum → update() mutates state → view() re-renders
