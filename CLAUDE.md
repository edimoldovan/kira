# CLAUDE.md

Kira - email client written in Rust using the Iced GUI framework.

## Commands

```bash
cargo build    # Build
cargo run      # Run
cargo check    # Check for errors
```

## Architecture

**Elm-style pattern**: `Message` enum → `update()` → `view()`

**Core modules**:
- `main.rs` - App entry, state (`Kira` struct), update/view loop
- `config.rs` - Loads accounts from `~/.config/kira/accounts.toml` (TOML)
- `theme.rs` - Dark theme colors
- `email/account.rs` - `Account`, `AccountConfig`, `Message` structs
- `email/imap.rs` - IMAP sync (async-imap + async-std), body fetching, HTML→text
- `email/oauth.rs` - Google OAuth2 flow, XOAUTH2 SASL auth, token refresh
- `email/cache.rs` - JSON cache in `~/.cache/kira/`
- `ui/sidebar.rs` - Account list, folder tree
- `ui/message_list.rs` - Message preview list
- `ui/reader.rs` - Message body viewer

**State** (`main.rs`): accounts, current_account, current_message, expanded_accounts, sync_error, is_syncing

**Message flow**: UI events → Message enum → update() mutates state → view() re-renders

## Current status

**Working**: IMAP read-only (TLS, port 993), last 100 messages, incremental sync, on-demand body fetch, local JSON cache, three-pane UI, dark theme, Google OAuth2 (XOAUTH2 IMAP auth)

**Auth**: Supports `auth = "password"` (plain IMAP login) and `auth = "google_oauth"` (OAuth2 with XOAUTH2). OAuth accounts need `client_id`, `client_secret` from Google Cloud Console (Desktop app type). Refresh tokens saved to accounts.toml automatically.

**Not implemented**: SMTP/sending, compose UI, multi-folder (INBOX only), attachments, search, message actions (reply/forward/delete are stubbed)

**Important**: async-imap requires reading the server greeting via `client.read_response()` before calling `authenticate`. IMAP operations are wrapped in `async_std::task::spawn` because iced's executor doesn't run async-std's reactor. OAuth token exchange uses blocking reqwest via `spawn_blocking` (same reason).

## Key deps

iced 0.13, async-imap, async-native-tls, async-std, mailparse, ammonia (HTML sanitize), chrono, serde/serde_json, toml, directories, oauth2 4.4 (with reqwest), open, form_urlencoded
