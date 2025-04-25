Zeno
A command-line Zettelkasten system for organizing notes.
Installation

Clone the repository:
git clone <repository-url>
cd zeno

Build the project:
cargo build

Usage
Create a new note
cargo run -- create timestamp

Create a note with a timestamp-based ID. You can specify a custom inbox path:
cargo run -- create timestamp --inbox-path custom/inbox

Archive a note
cargo run -- archive 20250425120917

Archive a note by ID. Specify custom paths if needed:
cargo run -- archive 20250425120917 --inbox-path custom/inbox --archive-path custom/archive

Export a note
cargo run -- export 20250425120917

Export a note to Markdown. Specify a custom export path:
cargo run -- export 20250425120917 --export-path custom/export

Development

Build: cargo build
Test: cargo test
Run: cargo run -- --help
Benchmark: cargo bench
