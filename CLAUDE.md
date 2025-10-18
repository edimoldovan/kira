# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Kira is a GTK4-based email client written in Rust. The application uses GTK4 widgets to create a three-panel email interface (sidebar, message list, and message reader).

## Build and Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Run a specific test
cargo test <test_name>

# Check for compilation errors without building
cargo check
```

## Architecture

### Core Structure

The codebase is organized into two main modules:

- **`ui/`**: Contains all GTK4 interface components
- **`email/`**: Contains email account models and IMAP implementation (currently minimal)

### UI Component Architecture

The UI follows a component-based architecture where each component is a separate module that builds and returns GTK4 widgets. Communication between components is handled through callbacks passed down from the parent.

**Main Window (`ui/window.rs`)**:
- Entry point for the UI, built from `main.rs`
- Creates mock account data with messages (currently hardcoded)
- Manages state with `Rc<RefCell<T>>` for shared ownership and interior mutability
- Coordinates three main components: sidebar, message list, and reader
- Implements callback chains for data flow: sidebar → message list → reader

**Component Communication Pattern**:
- Parent components pass callbacks to children using closures
- Selected state is managed by cloning `Rc` pointers to button collections
- When a button is clicked, it removes CSS classes from all buttons and adds to itself
- Callbacks are cloned and moved into event handlers

**Sidebar (`ui/sidebar.rs`)**:
- Displays account inboxes with unread counts
- Shows expandable account details with folder lists
- Takes `on_inbox_click` callback which receives account index
- Manages selected state using `suggested-action` CSS class on buttons

**Message List (`ui/message_list.rs`)**:
- Displays messages for the currently selected account
- Provides `update_messages()` function that clears and rebuilds the list
- Takes `on_message_click` callback which receives message index
- First message is selected by default

**Reader (`ui/reader.rs`)**:
- Displays the full content of the selected message
- Provides `update_message()` function that clears and rebuilds the content
- Shows from, subject, and body fields

### Data Models

**Account** (`email/account.rs`):
- Represents an email account with name, email, unread count, folders, and messages
- Currently using mock data defined in `ui/window.rs`

**Message** (`email/account.rs`):
- Contains from, subject, preview, body, and unread status

### State Management

The application uses Rust's `Rc<RefCell<T>>` pattern for shared mutable state:
- Account data is stored in `Rc<Vec<Account>>`
- Current account index tracked with `Rc<RefCell<usize>>`
- Button collections tracked with `Rc<RefCell<Vec<Button>>>` for managing selected states

This pattern is necessary because GTK4 callbacks require `'static` lifetimes, so data must be reference-counted and interior-mutable.

## Development Notes

- The IMAP module (`email/imap.rs`) is currently a placeholder
- All message data is currently mocked in `ui/window.rs:11-70`
- The application uses GTK4's CSS classes for styling (e.g., `suggested-action`, `dim-label`, `bold`)
- Component functions return tuples like `(Widget, InnerWidget)` where the inner widget is needed for later updates
