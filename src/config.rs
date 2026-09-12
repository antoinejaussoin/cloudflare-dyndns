use std::env;
use std::path::PathBuf;

const DEFAULT_TOKEN: &str = "<your cloudflare token>";
const DEFAULT_DOMAIN: &str = "acme.com";
const DEFAULT_RECORD: &str = "www";
const DEFAULT_CHECK_INTERVAL_SEC: u64 = 900;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub cloudflare_api_token: String,
    pub domain: String,
    pub record: String,
    pub debug: bool,
    pub check_interval_sec: u64,
}

impl Config {
    /// Load config from environment variables, optionally hydrating from a `.env` file first.
    pub fn from_env() -> Self {
        load_dotenv();
        Self::from_env_vars()
    }

    pub fn from_env_vars() -> Self {
        Self {
            cloudflare_api_token: env_or("CLOUDFLARE_API_TOKEN", DEFAULT_TOKEN),
            domain: env_or("DOMAIN", DEFAULT_DOMAIN),
            record: env_or("RECORD", DEFAULT_RECORD),
            debug: env_bool("DEBUG", false),
            check_interval_sec: env_u64("CHECK_INTERVAL_SEC", DEFAULT_CHECK_INTERVAL_SEC),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => default.to_string(),
    }
}

/// Matches the Node behavior: only the exact string `"true"` enables DEBUG.
fn env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(v) => v == "true",
        Err(_) => default,
    }
}

fn env_u64(key: &str, default: u64) -> u64 {
    match env::var(key) {
        Ok(v) => v.parse().unwrap_or(default),
        Err(_) => default,
    }
}

/// Walk up to 5 parents looking for `.env` then `.env.example` (legacy Node behavior).
fn load_dotenv() {
    if let Some(path) = find_dotenv_path() {
        let _ = dotenvy::from_path(path);
    }
}

fn find_dotenv_path() -> Option<PathBuf> {
    let mut current = env::current_dir().ok()?;
    for _ in 0..5 {
        let custom = current.join(".env");
        if custom.is_file() {
            return Some(custom);
        }
        let example = current.join(".env.example");
        if example.is_file() {
            return Some(example);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_vars;

    #[test]
    fn defaults_when_env_unset() {
        with_vars(
            [
                ("CLOUDFLARE_API_TOKEN", None::<&str>),
                ("DOMAIN", None),
                ("RECORD", None),
                ("DEBUG", None),
                ("CHECK_INTERVAL_SEC", None),
            ],
            || {
                let cfg = Config::from_env_vars();
                assert_eq!(cfg.cloudflare_api_token, DEFAULT_TOKEN);
                assert_eq!(cfg.domain, DEFAULT_DOMAIN);
                assert_eq!(cfg.record, DEFAULT_RECORD);
                assert!(!cfg.debug);
                assert_eq!(cfg.check_interval_sec, DEFAULT_CHECK_INTERVAL_SEC);
            },
        );
    }

    #[test]
    fn debug_only_true_exact() {
        with_vars([("DEBUG", Some("true"))], || {
            assert!(Config::from_env_vars().debug);
        });
        with_vars([("DEBUG", Some("TRUE"))], || {
            assert!(!Config::from_env_vars().debug);
        });
        with_vars([("DEBUG", Some("1"))], || {
            assert!(!Config::from_env_vars().debug);
        });
        with_vars([("DEBUG", Some("false"))], || {
            assert!(!Config::from_env_vars().debug);
        });
    }

    #[test]
    fn parses_interval_and_custom_values() {
        with_vars(
            [
                ("CLOUDFLARE_API_TOKEN", Some("tok")),
                ("DOMAIN", Some("example.com")),
                ("RECORD", Some("home")),
                ("CHECK_INTERVAL_SEC", Some("60")),
            ],
            || {
                let cfg = Config::from_env_vars();
                assert_eq!(cfg.cloudflare_api_token, "tok");
                assert_eq!(cfg.domain, "example.com");
                assert_eq!(cfg.record, "home");
                assert_eq!(cfg.check_interval_sec, 60);
            },
        );
    }

    #[test]
    fn empty_record_is_root() {
        with_vars(
            [("DOMAIN", Some("acme.com")), ("RECORD", Some(""))],
            || {
                let cfg = Config::from_env_vars();
                assert_eq!(cfg.record, "");
                assert_eq!(crate::record::record_fqdn(&cfg.record, &cfg.domain), "acme.com");
            },
        );
    }

    #[test]
    fn subdomain_fqdn() {
        with_vars(
            [("DOMAIN", Some("acme.com")), ("RECORD", Some("www"))],
            || {
                let cfg = Config::from_env_vars();
                assert_eq!(
                    crate::record::record_fqdn(&cfg.record, &cfg.domain),
                    "www.acme.com"
                );
            },
        );
    }

    #[test]
    fn invalid_interval_falls_back_to_default() {
        with_vars([("CHECK_INTERVAL_SEC", Some("not-a-number"))], || {
            assert_eq!(
                Config::from_env_vars().check_interval_sec,
                DEFAULT_CHECK_INTERVAL_SEC
            );
        });
    }
}
