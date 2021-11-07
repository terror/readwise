//! Rust wrapper for the Readwise public API. The official readwise public API
//! documentation can be found [here](https://readwise.io/api_deets).
//! This wrapper supports retrieving Book information and CRUD functionality for
//! Highlights.
//!
//! ## Installation
//! Simply add `readwise = "0.3.1"` to your Cargo.toml
//!
//! ## Example
//! ```no_run
//! use readwise::auth;
//!
//! extern crate dotenv;
//!
//! use dotenv::dotenv;
//! use std::{collections::HashMap, env};
//!
//! fn main() {
//!   dotenv().ok();
//!
//!   let client = auth(&env::var("ACCESS_TOKEN").unwrap()).unwrap();
//!
//!   // Fetch all books on page 1
//!   for book in client.get_books(1).unwrap() {
//!     println!("{}", book.title);
//!   }
//!
//!   // Fetch all highlights on page 1
//!   for highlight in client.get_highlights(1).unwrap() {
//!     println!("{}", highlight.id);
//!   }
//!
//!   // Create highlight(s)
//!   let mut new_highlight = HashMap::new();
//!   new_highlight.insert("text", "hello world!");
//!
//!   for highlight in client.create_highlights(vec![new_highlight]).unwrap() {
//!     println!("{}", highlight.text);
//!   }
//!
//!   // Update a highlight by ID
//!   let mut fields = HashMap::new();
//!   fields.insert("text", "hello, world!");
//!   client.update_highlight(138105649, fields).unwrap();
//!
//!   // Delete a highlight by ID
//!   client.delete_highlight(136887156).unwrap();
//! }
//! ```
mod common;
mod request;
mod url;

pub mod auth;
pub mod client;
pub mod error;
pub mod model;

pub use {crate::auth::auth, client::Client, error::Error, model::*};
