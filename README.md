# Markdown to Anki

A small rust crate to convert Obsidian callouts notes to formats that can be imported by `ObsidianToAnki` or to interact with `AnkiConnect` directly.

## Usage

## ObsidianToAnki

```bash
md2anki markdown input [output_file]
```

### Demo

To demo how the project works, run the following command:

```rust
cargo run markdown demo
```

This will generate the following file: `markdown_to_anki/Anki cards.md`
This file can be parsed using the `ObsidianToAnki` plugin from inside of obsidian, or using the associated Python scripts.

## AnkiConnect

WIP
