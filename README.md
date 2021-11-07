## Readwise

<p>
  A rust wrapper for the <a href="https://readwise.io/" target="_blank">Readwise</a> API.
  <br/><br/>
  <a href="https://crates.io/crates/readwise" target="_blank">
    <img src="https://shields.io/crates/v/readwise.svg"/>
  </a>
  <a href="https://github.com/terror/readwise/blob/master/.github/workflows/rust.yml" target="_blank">
    <img src="https://github.com/terror/readwise/actions/workflows/rust.yml/badge.svg"/>
  </a>
</p>


### Installation

Simply add readwise to your Cargo.toml file:

```
readwise = "0.3.1"
```

### Example

Here is a small example showcasing the main functionality of the library.

```rust
use readwise::auth;

extern crate dotenv;

use dotenv::dotenv;
use std::{collections::HashMap, env};

fn main() {
  dotenv().ok();

  let client = auth(&env::var("ACCESS_TOKEN").unwrap()).unwrap();

  // Fetch all books on page 1
  for book in client.get_books(1).unwrap() {
    println!("{}", book.title);
  }

  // Fetch all highlights on page 1
  for highlight in client.get_highlights(1).unwrap() {
    println!("{}", highlight.id);
  }

  // Create highlight(s)
  let mut highlight = HashMap::new();
  highlight.insert("text", "hello world!");

  for highlight in client.create_highlights(vec![highlights]).unwrap();
    println!("{}", highlight.text);
  }

  // Update a highlight by ID
  let mut fields = HashMap::new();
  fields.insert("text", "hello, world!");
  client.update_highlight(138105649, fields).unwrap();

  // Delete a highlight by ID
  client.delete_highlight(136887156).unwrap();
}
```
