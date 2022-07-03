use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{Extension, Path, RawQuery},
    http::StatusCode,
    Router,
    Extension as AddExtensionLayer,
    routing::get,
    response::{IntoResponse, Response},
};
use isahc::{
    config::{DnsCache, RedirectPolicy, VersionNegotiation},
    prelude::*,
    HttpClient,
};

pub const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36 Edg/103.0.1264.44";

#[tokio::main]
async fn main() {
    let http_client = HttpClient::builder()
                    .default_header("user-agent", USER_AGENT)
                    .max_connections(10)
                    .max_connections_per_host(10)
                    .connection_cache_size(10)
                    .tcp_keepalive(Duration::from_secs(60))
                    .dns_cache(DnsCache::Forever)
                    //.dns_resolve(ResolveMap::new()
                    // Send requests for example.org on port 80 to 127.0.0.1.
                    //.add("www.example.org", 8080, [127, 0, 0, 1]))
                    .version_negotiation(VersionNegotiation::http2())
                    .cookies()
                    .redirect_policy(RedirectPolicy::Follow)
                    .connect_timeout(Duration::from_secs(60))
                    .timeout(Duration::from_secs(60))
                    .auto_referer()
                    .automatic_decompression(true)
                    .build()
                    .unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/*url", get(handler))
        // .layer(AddExtensionLayer("https://www.nseindia.com/api/getNifty50VsindiaVix"))
        .layer(AddExtensionLayer(http_client));

    // run it
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on {}", addr);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Path(url): Path<String>,
    RawQuery(query): RawQuery,
    // Extension(uri): Extension<&str>,
    Extension(http_client): Extension<HttpClient>,
) -> Response {
    // println!("{:?}", url);
    // println!("{:?}", query);
    // let uri = uri.parse::<Uri>().unwrap();
    let mut nse_base_url = String::from("https://www.nseindia.com/");
    let _ = http_client
        .get_async(&nse_base_url)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Fetching {:?} hasn't worked...", &nse_base_url)).into_response());
    nse_base_url.push_str(&url);
    if query.is_some() {
    nse_base_url.push_str("?");
    nse_base_url.push_str(&query.as_ref().unwrap());
    }
    let response = http_client
        .get_async(nse_base_url)
        .await
        .map_err(|_| (StatusCode::GATEWAY_TIMEOUT, "Fetching hasn't worked...").into_response());

    let body = response.expect("Fetching hasn't worked...")
        .text()
        .await
        .map_err(|_| (StatusCode::NO_CONTENT, "Fetching text hasn't worked...").into_response());

    body.into_response()
}

