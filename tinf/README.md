tinf
====

A low-level interface to terminfo databases.

[![Build Status](https://travis-ci.org/edmccard/tvis.svg?branch=master)](https://travis-ci.org/edmccard/tvis)

[Documentation](https://docs.rs/tinf)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinf = "0.14.0"
```

and this to your crate root:

```rust
extern crate tinf;
```

## Examples

```rust
use tinf::{Desc, tparm, Vars};

// Find the description for "xterm" in the default locations.
let mut file = Desc::file("xterm")?;

// Parse it into a `Desc` object.
let desc = Desc::parse(&mut file)?;

// Send the escape sequence to set foreground to red.
let stdout = &mut std::io::stdout();
let mut vars = Vars::new();
tparm(stdout, &desc[setaf], &mut params!(1), &mut vars)?;
```
