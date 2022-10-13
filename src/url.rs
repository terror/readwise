pub(crate) fn request_url() -> String {
  #[cfg(not(test))]
  let url = "https://readwise.io".to_string();
  #[cfg(test)]
  let url = mockito::server_url();
  url
}
