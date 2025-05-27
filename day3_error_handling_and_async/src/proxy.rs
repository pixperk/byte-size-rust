use axum::{body::Body, http::Request, response::IntoResponse};
use reqwest::Client;

use crate::{config::UPSTREAM_BASE, error::ProxyError};

pub async fn proxy_handler(req: Request<Body>) -> Result<impl IntoResponse, ProxyError> {
    let client = Client::new();

    // Log request details
    println!("📥 Request received: {} {}", req.method(), req.uri());
    println!("📥 Request headers:");
    for (name, value) in req.headers() {
        println!("   {}: {:?}", name, value);
    }

    //Reconstruct the URI
    let uri = req.uri();
    let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
    let upstream_uri = format!("{}{}", UPSTREAM_BASE, path_and_query);
    println!("🔀 Forwarding to: {}", upstream_uri);

    // Extract method before consuming the request
    let method = req.method().clone();

    // Convert Body -> bytes -> reqwest body
    let bytes = match axum::body::to_bytes(req.into_body(), 1024 * 1024 * 10).await {
        Ok(bytes) => bytes,
        Err(err) => return Err(ProxyError::BodyConversionError(err.to_string())),
    };

    // Forward the request to the upstream server
    let resp = client
        .request(method, upstream_uri)
        .body(bytes)
        .send()
        .await
        .map_err(ProxyError::RequestError)?;

    // Convert the response to a format that can be returned
    let status = resp.status();
    let headers = resp.headers().clone();
    println!("📤 Response received from upstream: {}", status);

    let body = resp.bytes().await.map_err(ProxyError::RequestError)?;
    println!("📦 Response body size: {} bytes", body.len());

    // Build and return the response
    let mut response_builder = axum::response::Response::builder().status(status);

    // Add all headers from the upstream response
    for (name, value) in headers.iter() {
        println!("🧾 Header: {}: {:?}", name, value);
        response_builder = response_builder.header(name, value);
    }

    let response = response_builder.body(Body::from(body)).unwrap();

    println!("✅ Successfully forwarded request and returning response");
    Ok(response)
}
