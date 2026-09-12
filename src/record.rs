use serde::{Deserialize, Serialize};

/// Minimal DNS record fields we need from Cloudflare.
///
/// Note: as of Cloudflare's Nov 2024 deprecation, list/get/update DNS record
/// responses no longer include `zone_id` / `zone_name`. We keep `zone_id` on
/// this type for internal use and fill it from the zone path after fetching.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DnsRecord {
    pub id: String,
    #[serde(default)]
    pub zone_id: String,
    pub name: String,
    pub content: String,
    #[serde(rename = "type")]
    pub record_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiListResponse<T> {
    pub success: bool,
    pub result: Option<Vec<T>>,
    #[serde(default)]
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ApiSingleResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    #[serde(default)]
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
}

/// Body sent when updating an A record (matches legacy Node payload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateRecordPayload {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,
}

impl UpdateRecordPayload {
    pub fn a_record(name: &str, ip: &str) -> Self {
        Self {
            record_type: "A".to_string(),
            name: name.to_string(),
            content: ip.to_string(),
        }
    }
}

/// Resolve the DNS name used for matching / updates.
pub fn record_fqdn(record: &str, domain: &str) -> String {
    if record.is_empty() {
        domain.to_string()
    } else {
        format!("{record}.{domain}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fqdn_subdomain() {
        assert_eq!(record_fqdn("www", "acme.com"), "www.acme.com");
    }

    #[test]
    fn fqdn_root() {
        assert_eq!(record_fqdn("", "acme.com"), "acme.com");
    }

    #[test]
    fn update_payload_shape() {
        let payload = UpdateRecordPayload::a_record("www", "1.2.3.4");
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "A",
                "name": "www",
                "content": "1.2.3.4"
            })
        );
    }

    #[test]
    fn update_payload_root_uses_empty_name() {
        let payload = UpdateRecordPayload::a_record("", "9.9.9.9");
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["name"], "");
        assert_eq!(json["type"], "A");
        assert_eq!(json["content"], "9.9.9.9");
    }

    #[test]
    fn deserializes_record_without_zone_id() {
        let json = serde_json::json!({
            "id": "rec1",
            "name": "www.acme.com",
            "content": "1.2.3.4",
            "type": "A"
        });
        let record: DnsRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.id, "rec1");
        assert_eq!(record.zone_id, "");
        assert_eq!(record.name, "www.acme.com");
        assert_eq!(record.content, "1.2.3.4");
        assert_eq!(record.record_type, "A");
    }
}
