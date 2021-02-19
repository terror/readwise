use readwise::*;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
  dotenv().ok();

  let client = auth(&env::var("ACCESS_TOKEN").unwrap()).unwrap();

  for book in client.books(1).unwrap() {
    println!("{}\n", book.title);
  }
}
