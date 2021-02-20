## `Readwise`

[![crates.io](https://shields.io/crates/v/readwise.svg)](https://crates.io/crates/readwise)
![Build and Test](https://github.com/terror/readwise/actions/workflows/rust.yml/badge.svg)

A rust wrapper for the Readwise API.

## Installation

Simply add readwise to your Cargo.toml file:

```
readwise = "0.1.0"
```

## Example

```rust
use readwise::auth;

extern crate dotenv;

use dotenv::dotenv;
use std::{collections::HashMap, env};

fn main() -> Result<(), anyhow::Error> {
  dotenv().ok();

  let client = auth(&env::var("ACCESS_TOKEN").unwrap()).unwrap();

  // Fetch all books on page 1
  for book in client.books(1).unwrap() {
    println!("{}", book.title);
  }

  // Fetch all highlights on page 1
  for highlight in client.highlights(1).unwrap() {
    println!("{}", highlight.id);
  }

  // Create highlight(s)
  let mut highlights = Vec::new();
  let mut highlight = HashMap::new();

  highlight.insert("text", "hello world!");
  highlights.push(highlight);

  let result = client.create(highlights)?;

  for highlight in result {
    println!("{}", highlight.text);
  }

  // Update a highlight by ID
  let mut fields = HashMap::new();
  fields.insert("text", "hello, world!");

  let _result = client.update(138105649, fields)?;

  // Delete a highlight by ID
  client.delete(136887156)?;

  Ok(())
}
```
