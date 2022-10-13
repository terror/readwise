//! A rust wrapper for the [Readwise](https://readwise.io/) public API.
//! The official readwise public API documentation can be
//! found [here](https://readwise.io/api_deets).
//!
//! This wrapper supports retrieving book information in addition to CRUD
//! functionality for highlights.
//!
//! ## Installation
//! Simply add `readwise = "0.4.0"` to your Cargo.toml
//!
//! ## Example
//! ```no_run
//! use {
//!   std::{collections::HashMap, env},
//!   dotenv::dotenv,
//!   readwise::client::Client
//! };
//!
//! dotenv().ok();
//!
//! let client = Client::new(&env::var("ACCESS_TOKEN").unwrap()).unwrap();
//!
//! // Fetch all books on page 1
//! for book in client.books(1).unwrap() {
//!   println!("{}", book.title);
//! }
//!
//! // Fetch all highlights on page 1
//! for highlight in client.highlights(1).unwrap() {
//!   println!("{}", highlight.id);
//! }
//!
//! // Create highlight(s)
//! let mut new_highlight = HashMap::new();
//! new_highlight.insert("text", "hello world!");
//!
//! for highlight in client.create_highlights(vec![new_highlight]).unwrap() {
//!   println!("{}", highlight.text);
//! }
//!
//! // Update a highlight by ID
//! let mut fields = HashMap::new();
//! fields.insert("text", "hello, world!");
//! client.update_highlight(138105649, fields).unwrap();
//!
//! // Delete a highlight by ID
//! client.delete_highlight(136887156).unwrap();
//! ```
mod common;
mod url;

pub mod client;
pub mod error;
pub mod model;
