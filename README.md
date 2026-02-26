# Markdown to Anki

A small rust crate to convert Obsidian callouts notes to formats that can be imported by the [`Obsidian_to_Anki`](https://github.com/ObsidianToAnki/Obsidian_to_Anki) Obsidian plugin or to interact with `AnkiConnect` directly.

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

## Markdown Notes Format

Markdown to Anki looks for callout blocks to convert into flashcards.
It only converts callouts that have a `CalloutType` that is `Word` or `Rule`.
Currently the following text is recognized as these types:

- `Word`: `word`, `단어`
- `Rule`: `rule`, `규칙`

It can handle nested callouts.
The top-level/most outer callout will be the base content for the flashcard.
All nested callouts will be embedded in the flashcard as callouts.

Markdown to Anki creates a simple Note Type with 3 fields:

- MarkdownID: uses the last block ID found in the callout.
- Front: the word text in the callout block header.
- Back: everything inside the callout that is not the header or MarkdownID.

### Syntax Overview

```markdown
> [!CalloutType](+|-)? (Front)
> (Back)
> ^(MarkdownID)
```

### Basic Example

```markdown
> [!word]- 단어
> word
> ^daneo
```

This would be parsed as:

- MarkdownID: daneo
- Front: 단어
- Back: word

### Nested Callout Example

> [!규칙] VS + 자
> [[반말]] Let's do V
>
> > [!예]
> > 가자
> ---
> > [!예문]
> > 집에 가자.
> > Let's go home.
> > ---
> > 자! 밥먹자!
> > Hey! Let's eat!
> ---
> > [!인용]
> > [[KMS2 Chapter 12 Let's]]
>
> ^y6d5vj487m

This would be parsed as:

- MarkdownID: y6d5vj487m
- Front: VS + 자
- Back:
```
[[반말]] Let's do V

> [!예]
> 가자
---
> [!예문]
> 집에 가자.
> Let's go home.
> ---
> 자! 밥먹자!
> Hey! Let's eat!
---
> [!인용]
> [[KMS2 Chapter 12 Let's]]
```

Which in the Anki HTML format would be:
```html
<p>[[반말|C]] Let's do V</p>

<details data-callout="example" class="callout"><summary class="callout-title"><div class="callout-icon"></div>Example</summary><p>가자</p></details>
<hr>
<details data-callout="example-sentence" class="callout"><summary class="callout-title"><div class="callout-icon"></div>Example Sentence</summary><p>집에 가자.</p>
<p>Let's go home.</p>
<hr>
<p>자! 밥먹자!.</p>
<p>Hey! Let's eat!</p></details>
<hr>
<details data-callout="cite" class="callout"><summary class="callout-title"><div class="callout-icon"></div>Cite</summary><p>[[KMS2 Chapter 12]]</p></details>
```
