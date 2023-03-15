# get_frontmatter

Get the `yaml` frontmatter from a given `markdown` file.

## Usage

You can clone the repository onto your local machine through the following steps:

```zsh
git clone https://github.com/preetiman-misra/get_frontmatter.git
cd get_frontmatter
cargo run
```

## Output

By providing a `path` to the `jsonify_frontmatter<P: <AsRef<Path>>>(path: P)` function, you get
a corresponding `output.json`.

For an input file `hello.md` containing:

```commonmark
---
title: Welcome to Rust Get Frontmatter
date: 2023-03-02
author: Preetiman Misra
categories: casual, coding, rust
---

# Hello, world!

This is just another one of my markdown files.
```

The output is:

```json
{
  "title": "Welcome to Rust Get Frontmatter",
  "author": "Preetiman Misra",
  "date": "2023-03-02",
  "categories": "casual, coding, rust"
}
```

<br />

Enjoy!
