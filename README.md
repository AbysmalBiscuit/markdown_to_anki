# Markdown to Anki

A small rust crate to convert Obsidian callouts notes to formats that can be imported by `ObsidianToAnki` or to interact with `AnkiConnect` directly.

## Installing

To install the `md2anki` binary:

```bash
cargo install --path .
```

## Usage

To see help instructions just run:

```bash
md2anki --help
```

Or without installing:

```bash
cargo run -- --help
```

## ObsidianToAnki

```bash
md2anki markdown input [output_file]
```

### Markdown Demo

To demo how the project works, run the following command:

```bash
cargo run markdown demo
```

This will generate the following file: `markdown_to_anki/Anki cards.md`
This file can be parsed using the `ObsidianToAnki` plugin from inside of obsidian, or using the associated Python scripts.

## AnkiConnect

Make sure Anki desktop is installed.

Install the AnkiConnect plugin by:

1. Tools > Add-ons
2. Click on `Get Add-ons`
3. Type in `2055492159`
4. Click `Ok`
5. Restart Anki

### AnkiConnect Demo

To demo how the project works, run the following command:

```bash
cargo run -- sync --css cards_style.css demo
```

This will create a deck called `md2anki` with cards created from the markdown notes in `demo/`.
