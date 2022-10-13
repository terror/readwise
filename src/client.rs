use crate::common::*;

pub struct Client {
  /// A readwise access token
  access_token: String,
}

impl Client {
  /// Create and authenticate a new Readwise client from a specified access token
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// ```
  pub fn new(access_token: &str) -> Result<Self> {
    let url = format!("{}/api/v2{}", &request_url(), "/auth");

    let mut headers = header::HeaderMap::new();

    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&format!("Token {}", access_token))?,
    );

    let client = blocking::Client::builder()
      .default_headers(headers)
      .build()?;

    let response = client.get(&url).send()?;

    match response.status().is_success() {
      true => Ok(()),
      false => Err(error::Error::BadRequest {
        status: response.status(),
      }),
    }?;

    Ok(Self {
      access_token: access_token.to_string(),
    })
  }

  /// Fetch all books from a specified page
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// let books = client.books(1).unwrap();
  /// ```
  pub fn books(&self, page: u64) -> Result<Vec<Book>> {
    Ok(
      serde_json::from_str::<BooksResponse>(
        &self
          .request(&format!("/books?page={}", page), Method::GET, None)?
          .text()?,
      )?
      .results,
    )
  }

  /// Fetch all highlights from a specified page
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// let highlights = client.highlights(1).unwrap();
  /// ```
  pub fn highlights(&self, page: u64) -> Result<Vec<Highlight>> {
    Ok(
      serde_json::from_str::<HighlightsResponse>(
        &self
          .request(&format!("/highlights?page={}", page), Method::GET, None)?
          .text()?,
      )?
      .results,
    )
  }

  /// Fetch a single book by identifier
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// let book = client.book(1).unwrap();
  /// ```
  pub fn book(&self, id: u64) -> Result<Book> {
    Ok(serde_json::from_str::<Book>(
      &self
        .request(&format!("/books/{}", id), Method::GET, None)?
        .text()?,
    )?)
  }

  /// Fetch a single highlight by identifier
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// let highlight = client.highlight(1).unwrap();
  /// ```
  pub fn highlight(&self, id: u64) -> Result<Highlight> {
    Ok(serde_json::from_str::<Highlight>(
      &self
        .request(&format!("/highlights/{}", id), Method::GET, None)?
        .text()?,
    )?)
  }

  /// Create and return one or more highlights
  ///
  /// ```no_run
  /// use {
  ///   std::collections::HashMap,
  ///   readwise::client::Client
  /// };
  ///
  /// let client = Client::new("token").unwrap();
  ///
  /// let mut new_highlight = HashMap::new();
  ///
  /// new_highlight.insert("text", "hello world!");
  ///
  /// for highlight in client.create_highlights(vec![new_highlight]).unwrap() {
  ///   println!("{}", highlight.text);
  /// }
  /// ```
  pub fn create_highlights(
    &self,
    highlights: Vec<HashMap<&str, &str>>,
  ) -> Result<Vec<Highlight>> {
    let mut body = HashMap::new();

    body.insert("highlights", highlights);

    let identifiers = serde_json::from_str::<Vec<HighlightCreateResponse>>(
      &self
        .request("/highlights", Method::POST, Some(body))?
        .text()?,
    )?
    .into_iter()
    .flat_map(|item| item.modified_highlights)
    .collect::<Vec<u64>>();

    identifiers
      .iter()
      .map(|identifier| self.highlight(*identifier))
      .collect::<Result<Vec<Highlight>, _>>()
  }

  /// Update a single highlight by identifier
  ///
  /// ```no_run
  /// use {
  ///   std::collections::HashMap,
  ///   readwise::client::Client
  /// };
  ///
  /// let client = Client::new("token").unwrap();
  ///
  /// let mut fields = HashMap::new();
  /// fields.insert("text", "hello, world!");
  ///
  /// client.update_highlight(1, fields).unwrap();
  /// ```
  pub fn update_highlight(
    &self,
    id: i64,
    body: HashMap<&str, &str>,
  ) -> Result<Highlight> {
    let mut container = HashMap::new();

    container.insert("body", vec![body]);

    Ok(serde_json::from_str::<Highlight>(
      &self
        .request(
          &format!("/highlights/{}", id),
          Method::PATCH,
          Some(container),
        )?
        .text()?,
    )?)
  }

  /// Delete a single highlight by identifier
  ///
  /// ```no_run
  /// use readwise::client::Client;
  ///
  /// let client = Client::new("token").unwrap();
  /// client.delete_highlight(1).unwrap();
  /// ```
  pub fn delete_highlight(&self, id: i64) -> Result {
    self.request(&format!("/highlights/{}", id), Method::DELETE, None)?;
    Ok(())
  }

  fn request(
    &self,
    endpoint: &str,
    method: Method,
    body: Option<HashMap<&str, Vec<HashMap<&str, &str>>>>,
  ) -> Result<Response> {
    let url = format!("{}/api/v2{}", &request_url(), endpoint);

    let mut headers = header::HeaderMap::new();

    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&format!("Token {}", self.access_token))?,
    );

    let request_client = blocking::Client::builder()
      .default_headers(headers)
      .build()?;

    let request = match method {
      Method::GET => Ok(request_client.get(&url)),
      Method::POST => Ok(request_client.post(&url).json(&body.unwrap())),
      Method::PATCH => {
        Ok(request_client.patch(&url).json(&body.unwrap()["body"][0]))
      }
      Method::DELETE => Ok(request_client.delete(&url)),
      _ => Err(error::Error::UnsupportedRequest { method }),
    };

    let response = request?.send()?;

    match response.status().is_success() {
      true => Ok(response),
      false => Err(error::Error::BadRequest {
        status: response.status(),
      }),
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, mockito::mock};

  fn client() -> Client {
    Client {
      access_token: String::new(),
    }
  }

  fn get_book_as_string() -> String {
    serde_json::to_string(&Book::default()).unwrap()
  }

  fn get_highlight_as_string() -> String {
    serde_json::to_string(&Highlight::default()).unwrap()
  }

  #[test]
  fn authenticate() {
    let _m = mock("GET", "/api/v2/auth").with_status(204).create();

    let result = Client::new("token");

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());

    let client = result.unwrap();

    assert_eq!("token", client.access_token);
  }

  #[test]
  fn authenticate_bad_token() {
    let _m = mock("GET", "/api/v2/auth").with_status(401).create();

    let result = Client::new("token");

    assert!(result.is_err(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn books() {
    let _m = mock("GET", "/api/v2/books?page=1")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        &get_book_as_string()
      ))
      .create();

    let result = client().books(1);

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn highlights() {
    let _m = mock("GET", "/api/v2/highlights?page=1")
      .with_status(200)
      .with_body(format!(
        r#" {{ "count": 1, "next": null, "previous": null, "results": [{}] }} "#,
        get_highlight_as_string()
      ))
      .create();

    let result = client().highlights(1);

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn single_book() {
    let _m = mock("GET", "/api/v2/books/1")
      .with_status(200)
      .with_body(get_book_as_string())
      .create();

    let result = client().book(1);

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn single_highlight() {
    let _m = mock("GET", "/api/v2/highlights/1")
      .with_status(200)
      .with_body(get_highlight_as_string())
      .create();

    let result = client().highlight(1);

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn create_highlights() {
    let _m = mock("POST", "/api/v2/highlights")
      .with_status(200)
      .with_body(
        r#"
        [ { "id": 1,
          "title": "Quotes",
          "author": null,
          "category": "books",
          "num_highlights": 5,
          "last_highlight_at": "2021-02-20T16:28:53.900414Z",
          "updated": "2021-02-20T16:35:41.793746Z",
          "cover_image_url": "https://readwise-assets.s3.amazonaws.com/static/images/default-book-icon-7.09749d3efd49.png",
          "highlights_url": "https://readwise.io/bookreview/7843339",
          "source_url": null,
          "modified_highlights": [] }
        ]"#,
      )
      .create();

    let result = client().create_highlights(Vec::new());

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn update_highlight() {
    let _m = mock("PATCH", "/api/v2/highlights/0")
      .with_status(200)
      .with_body(get_highlight_as_string())
      .create();

    let result = client().update_highlight(0, HashMap::new());

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }

  #[test]
  fn delete_highlight() {
    let _m = mock("DELETE", "/api/v2/highlights/1")
      .with_status(200)
      .create();

    let result = client().delete_highlight(1);

    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
  }
}
