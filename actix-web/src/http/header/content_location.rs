use super::{Uri, CONTENT_LOCATION};

crate::http::header::common_header! {
    /// `Content-Location` header, defined
    /// in [RFC 7231 §3.1.4.2](https://datatracker.ietf.org/doc/html/rfc7231#section-3.1.4.2)
    ///
    /// The "Content-Location" header field references a URI that can be used
    /// as an identifier for a specific resource corresponding to the
    /// representation in this message's payload.
    ///
    /// # ABNF
    /// ```plain
    /// Content-Location = absolute-URI / partial-URI
    /// ```
    ///
    /// # Example Values
    /// * `http://www.example.org/hypertext/Overview.html`
    ///
    /// # Examples
    ///
    /// ```
    /// use actix_web::HttpResponse;
    /// use actix_http::Uri;
    /// use actix_web::http::header::ContentLocation;
    ///
    /// let mut builder = HttpResponse::Created();
    /// builder.insert_header(
    ///     ContentLocation("http://www.example.org".parse::<Uri>().unwrap())
    /// );
    /// ```
    (ContentLocation, CONTENT_LOCATION) => [Uri]

    test_parse_and_format {
        crate::http::header::common_header_test!(test1, [b"http://www.example.org/hypertext/Overview.html"]);
    }
}