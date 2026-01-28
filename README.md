# Library Project (CLI)

Simple Rust command-line library system that lets an admin add books and users borrow/return them. Data is persisted to a local `library.json` file using `serde`.

## Features
- Add books via admin panel (`ID,Title,Author` input)
- Borrow and return books with basic status checks
- View the full inventory report in the user panel
- JSON persistence between runs (`library.json`)
- Basic unit tests for add/borrow logic

## Requirements
- Rust (stable, edition 2024)

## Getting Started
```bash
git clone <repo-url>
cd library_project
cargo run
```

On first run, a fresh library starts. If `library.json` already exists, it is loaded automatically.

## Usage
Run the app:
```bash
cargo run
```

Main menu:
- `1` Admin: add a book (input format: `101,Rust Pro,Hacker`)
- `2` User: view, borrow, or return
- `3` Exit: data is already saved after each action

Data file: `library.json` sits in the project root and is updated after each add/borrow/return action.

## Testing
```bash
cargo test
```

## Project Structure
- `src/main.rs` — CLI flow and menu handling
- `src/library.rs` — core library logic, persistence, and tests
- `src/models.rs` — data models (`Book`, `Status`)
- `library.json` — saved inventory (auto-created)

## Notes
- Input parsing is minimal; ensure the admin input uses commas.
- Borrowing fails if the book is already borrowed; returning resets status to available.
