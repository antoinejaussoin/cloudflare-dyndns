use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpError {
    #[error("failed to fetch public IP: {0}")]
    Request(#[from] reqwest::Error),
    #[error("ipify returned empty body")]
    Empty,
}

/// Fetch the public IPv4 address from ipify (plain text).
pub async fn get_public_ip(client: &reqwest::Client, base_url: &str) -> Result<String, IpError> {
    let url = base_url.trim_end_matches('/');
    let text = client.get(url).send().await?.error_for_status()?.text().await?;
    let ip = text.trim().to_string();
    if ip.is_empty() {
        return Err(IpError::Empty);
    }
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn fetches_and_trims_ip() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("  203.0.113.42\n"))
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let ip = get_public_ip(&client, &server.uri()).await.unwrap();
        assert_eq!(ip, "203.0.113.42");
    }

    #[tokio::test]
    async fn empty_body_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("   "))
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let err = get_public_ip(&client, &server.uri()).await.unwrap_err();
        assert!(matches!(err, IpError::Empty));
    }
}
