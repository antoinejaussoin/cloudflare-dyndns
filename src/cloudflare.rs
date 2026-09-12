use crate::record::{
    record_fqdn, ApiListResponse, ApiSingleResponse, DnsRecord, UpdateRecordPayload, Zone,
};
use thiserror::Error;

pub const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

#[derive(Debug, Error)]
pub enum CloudflareError {
    #[error("HTTP error talking to Cloudflare: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Cloudflare API error: {0}")]
    Api(String),
    #[error("Cannot find zone (domain) {0}")]
    ZoneNotFound(String),
    #[error("Cannot find record {0} in zone (domain) {1}")]
    RecordNotFound(String, String),
}

#[derive(Clone)]
pub struct CloudflareClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

impl CloudflareClient {
    pub fn new(http: reqwest::Client, base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub async fn get_dns_record(&self, domain: &str, record: &str) -> Result<DnsRecord, CloudflareError> {
        let zones = self.list_zones().await?;
        let zone = zones
            .into_iter()
            .find(|z| z.name == domain)
            .ok_or_else(|| CloudflareError::ZoneNotFound(domain.to_string()))?;

        let records = self.list_dns_records(&zone.id).await?;
        let expected = record_fqdn(record, domain);
        let mut found = records
            .into_iter()
            .find(|r| r.name == expected)
            .ok_or_else(|| CloudflareError::RecordNotFound(record.to_string(), domain.to_string()))?;
        // Cloudflare no longer returns zone_id on DNS records; keep it from the zone lookup.
        found.zone_id = zone.id;
        Ok(found)
    }

    pub async fn update_record(
        &self,
        zone_id: &str,
        record_id: &str,
        name: &str,
        ip: &str,
    ) -> Result<DnsRecord, CloudflareError> {
        let payload = UpdateRecordPayload::a_record(name, ip);
        let url = format!(
            "{}/zones/{}/dns_records/{}",
            self.base_url, zone_id, record_id
        );
        let response = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let body: ApiSingleResponse<DnsRecord> = response.json().await?;
        if !body.success {
            return Err(CloudflareError::Api(format!("{:?}", body.errors)));
        }
        let mut record = body
            .result
            .ok_or_else(|| CloudflareError::Api("missing result".into()))?;
        if record.zone_id.is_empty() {
            record.zone_id = zone_id.to_string();
        }
        Ok(record)
    }

    async fn list_zones(&self) -> Result<Vec<Zone>, CloudflareError> {
        let url = format!("{}/zones", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await?
            .error_for_status()?;

        let body: ApiListResponse<Zone> = response.json().await?;
        if !body.success {
            return Err(CloudflareError::Api(format!("{:?}", body.errors)));
        }
        Ok(body.result.unwrap_or_default())
    }

    async fn list_dns_records(&self, zone_id: &str) -> Result<Vec<DnsRecord>, CloudflareError> {
        let url = format!("{}/zones/{}/dns_records", self.base_url, zone_id);
        let response = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await?
            .error_for_status()?;

        let body: ApiListResponse<DnsRecord> = response.json().await?;
        if !body.success {
            return Err(CloudflareError::Api(format!("{:?}", body.errors)));
        }
        Ok(body.result.unwrap_or_default())
    }
}

/// Decide whether the DNS record needs updating.
pub fn needs_update(public_ip: &str, dns_content: &str) -> bool {
    public_ip != dns_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Matches current Cloudflare API responses (no zone_id / zone_name).
    fn sample_record(name: &str, content: &str) -> serde_json::Value {
        json!({
            "id": "rec1",
            "name": name,
            "content": content,
            "type": "A",
            "ttl": 1,
            "proxied": false,
            "proxiable": true
        })
    }

    async fn mock_zones_and_records(server: &MockServer, domain: &str, record_name: &str, ip: &str) {
        Mock::given(method("GET"))
            .and(path("/zones"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": [{ "id": "zone1", "name": domain }]
            })))
            .mount(server)
            .await;

        Mock::given(method("GET"))
            .and(path("/zones/zone1/dns_records"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": [sample_record(record_name, ip)]
            })))
            .mount(server)
            .await;
    }

    #[test]
    fn needs_update_when_different() {
        assert!(needs_update("1.1.1.1", "2.2.2.2"));
        assert!(!needs_update("1.1.1.1", "1.1.1.1"));
    }

    #[tokio::test]
    async fn get_dns_record_subdomain() {
        let server = MockServer::start().await;
        mock_zones_and_records(&server, "acme.com", "www.acme.com", "1.2.3.4").await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let record = client.get_dns_record("acme.com", "www").await.unwrap();
        assert_eq!(record.name, "www.acme.com");
        assert_eq!(record.content, "1.2.3.4");
        assert_eq!(record.id, "rec1");
        assert_eq!(record.zone_id, "zone1");
    }

    #[tokio::test]
    async fn get_dns_record_root() {
        let server = MockServer::start().await;
        mock_zones_and_records(&server, "acme.com", "acme.com", "5.5.5.5").await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let record = client.get_dns_record("acme.com", "").await.unwrap();
        assert_eq!(record.name, "acme.com");
        assert_eq!(record.content, "5.5.5.5");
    }

    #[tokio::test]
    async fn zone_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/zones"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": [{ "id": "zone1", "name": "other.com" }]
            })))
            .mount(&server)
            .await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let err = client.get_dns_record("acme.com", "www").await.unwrap_err();
        assert!(matches!(err, CloudflareError::ZoneNotFound(_)));
    }

    #[tokio::test]
    async fn record_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/zones"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": [{ "id": "zone1", "name": "acme.com" }]
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/zones/zone1/dns_records"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": [sample_record("mail.acme.com", "1.1.1.1")]
            })))
            .mount(&server)
            .await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let err = client.get_dns_record("acme.com", "www").await.unwrap_err();
        assert!(matches!(err, CloudflareError::RecordNotFound(_, _)));
    }

    #[tokio::test]
    async fn update_sends_legacy_payload() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/zones/zone1/dns_records/rec1"))
            .and(header("Authorization", "Bearer test-token"))
            .and(body_json(json!({
                "type": "A",
                "name": "www",
                "content": "9.9.9.9"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": sample_record("www.acme.com", "9.9.9.9")
            })))
            .mount(&server)
            .await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let updated = client
            .update_record("zone1", "rec1", "www", "9.9.9.9")
            .await
            .unwrap();
        assert_eq!(updated.content, "9.9.9.9");
    }

    #[tokio::test]
    async fn skip_update_when_ip_matches() {
        // Unit-level decision: no HTTP when IPs match.
        assert!(!needs_update("1.2.3.4", "1.2.3.4"));
    }

    #[tokio::test]
    async fn put_when_ip_differs_flow() {
        let server = MockServer::start().await;
        mock_zones_and_records(&server, "acme.com", "www.acme.com", "1.2.3.4").await;
        Mock::given(method("PUT"))
            .and(path("/zones/zone1/dns_records/rec1"))
            .and(body_json(json!({
                "type": "A",
                "name": "www",
                "content": "8.8.8.8"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "errors": [],
                "result": sample_record("www.acme.com", "8.8.8.8")
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = CloudflareClient::new(reqwest::Client::new(), server.uri(), "test-token");
        let record = client.get_dns_record("acme.com", "www").await.unwrap();
        assert!(needs_update("8.8.8.8", &record.content));
        let updated = client
            .update_record(&record.zone_id, &record.id, "www", "8.8.8.8")
            .await
            .unwrap();
        assert_eq!(updated.content, "8.8.8.8");
    }
}
