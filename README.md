# Todo List Application

A simple, elegant todo list application built with Rust and egui for a native desktop GUI experience.

## Features

- ✅ Add new todos with Enter key or Add button
- ✅ Mark todos as complete/incomplete with checkboxes
- ✅ Delete individual todos with the trash button
- ✅ View statistics (total, pending, completed todos)
- ✅ Clear all completed todos at once
- ✅ Persistent storage (saves to `todos.json` automatically)
- ✅ Clean, intuitive user interface
- ✅ Cross-platform (Windows, macOS, Linux)

## Screenshots

The application features a clean interface with:
- Input field for new todos
- Statistics showing total, pending, and completed items
- Scrollable todo list with checkboxes and delete buttons
- Completed todos are visually distinguished with grayed-out text

## Installation

### Prerequisites
- Rust (1.70 or later)
- Cargo (comes with Rust)

### Building from Source

1. Clone or download this project
2. Navigate to the project directory:
   ```bash
   cd todo_app
   ```
3. Build and run:
   ```bash
   cargo run
   ```

### Building for Release

To build an optimized release version:
```bash
cargo build --release
```

The executable will be available at `target/release/todo_app` (or `todo_app.exe` on Windows).

## Usage

1. **Adding todos**: Type in the text field and press Enter or click the "Add" button
2. **Completing todos**: Click the checkbox next to any todo item
3. **Deleting todos**: Click the 🗑 (trash) button next to any todo item
4. **Clearing completed**: When you have completed todos, use the "Clear Completed" button
5. **Persistence**: Your todos are automatically saved to `todos.json` in the application directory

## Technical Details

### Dependencies

- **eframe/egui**: Modern immediate-mode GUI framework for Rust
- **serde**: Serialization framework for saving/loading todos
- **serde_json**: JSON serialization support

### Architecture

The application is structured around two main types:

- `Todo`: Represents individual todo items with ID, text, and completion status
- `TodoApp`: Main application state managing the todo list and UI

Data is persisted automatically to a JSON file whenever changes are made.

### File Structure

```
todo_app/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
├── todos.json          # Automatically created data file
└── src/
    └── main.rs         # Complete application source code
```

## Development

The application is built with Rust's excellent ecosystem:

- Uses `egui` for immediate-mode GUI rendering
- Implements automatic persistence with JSON serialization
- Follows Rust best practices for memory safety and performance
- Clean, readable code structure suitable for learning and modification

## License

This project is provided as-is for educational and personal use.