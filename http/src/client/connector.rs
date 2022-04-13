//! Different HTTP connectors for the HTTP Client depending on features enabled.

#[cfg(feature = "hyper-rustls")]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;
#[cfg(all(feature = "hyper-tls", not(feature = "hyper-rustls")))]
type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;

#[cfg(feature = "trust-dns")]
type HttpConnector = hyper_trust_dns::TrustDnsHttpConnector;
#[cfg(not(feature = "trust-dns"))]
type HttpConnector = hyper::client::HttpConnector;

#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
pub type Connector = HttpsConnector<HttpConnector>;
#[cfg(not(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
pub type Connector = HttpConnector;

/// Build an HTTP connector for use in the Client.
pub fn build() -> Connector {
    #[cfg(feature = "trust-dns")]
    let mut connector = hyper_trust_dns::new_trust_dns_http_connector();
    #[cfg(not(feature = "trust-dns"))]
    let mut connector = hyper::client::HttpConnector::new();

    connector.enforce_http(false);

    #[cfg(feature = "rustls-native-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .wrap_connector(connector);
    #[cfg(all(feature = "rustls-webpki-roots", not(feature = "rustls-native-roots")))]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .wrap_connector(connector);
    #[cfg(all(
        feature = "hyper-tls",
        not(feature = "rustls-native-roots"),
        not(feature = "rustls-webpki-roots")
    ))]
    let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

    connector
}
