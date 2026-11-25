// xeton_core::privacy — Privacy-First Download Policies & Telemetry Stripping
//
// Eliminates tracker queries, sanitizes Referer leakage, and strips telemetry params
// before passing requests to connection pools or persisting to the database.

use std::collections::HashSet;
use std::sync::LazyLock;

/// Tracking parameters commonly appended by advertising and tracking networks.
static TRACKING_PARAMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    // Google Analytics / Ads
    s.insert("utm_source");
    s.insert("utm_medium");
    s.insert("utm_campaign");
    s.insert("utm_term");
    s.insert("utm_content");
    s.insert("gclid");
    s.insert("gclsrc");
    s.insert("dclid");
    // Facebook / Meta
    s.insert("fbclid");
    // Twitter / X
    s.insert("twclid");
    // Microsoft / Bing
    s.insert("msclkid");
    // Yandex
    s.insert("yclid");
    s.insert("ymclid");
    // TikTok
    s.insert("ttclid");
    // Hubspot, Marketo & others
    s.insert("_hsenc");
    s.insert("_hsmi");
    s.insert("mc_cid");
    s.insert("mc_eid");
    s.insert("igshid");
    s
});

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivacyDownloadPolicy {
    /// Strip analytics and ad-click tracking query parameters (utm_*, fbclid, etc.).
    pub strip_tracking_params: bool,
    /// Sanitize Referer to origin only (scheme://host) rather than full URL path with tokens.
    pub sanitize_referer: bool,
    /// When true, strictly verify TLS certificates without allowing any bypass.
    pub strict_tls: bool,
}

impl Default for PrivacyDownloadPolicy {
    fn default() -> Self {
        Self {
            strip_tracking_params: true,
            sanitize_referer: true,
            strict_tls: true,
        }
    }
}

impl PrivacyDownloadPolicy {
    /// Sanitize a target URL by removing tracking query parameters if configured.
    pub fn sanitize_url(&self, raw_url: &str) -> String {
        if !self.strip_tracking_params {
            return raw_url.to_string();
        }

        let Ok(mut parsed) = reqwest::Url::parse(raw_url) else {
            return raw_url.to_string();
        };

        let has_query = parsed.query().is_some();
        if !has_query {
            return raw_url.to_string();
        }

        let filtered_pairs: Vec<(String, String)> = parsed
            .query_pairs()
            .filter(|(k, _)| !TRACKING_PARAMS.contains(k.as_ref()))
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        if filtered_pairs.is_empty() {
            parsed.set_query(None);
        } else {
            parsed
                .query_pairs_mut()
                .clear()
                .extend_pairs(filtered_pairs);
        }

        parsed.to_string()
    }

    /// Sanitize Referer header to canonical origin only (RFC 6454) to avoid leaking session tokens or path info.
    pub fn sanitize_referer_header(&self, raw_referer: &str) -> Option<String> {
        if !self.sanitize_referer {
            return Some(raw_referer.to_string());
        }

        let parsed = reqwest::Url::parse(raw_referer).ok()?;
        let ascii = parsed.origin().ascii_serialization();
        if ascii == "null" { None } else { Some(ascii) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_tracking_params() {
        let policy = PrivacyDownloadPolicy::default();
        let url = "https://example.com/download.zip?utm_source=twitter&fbclid=12345&file_id=9988&gclid=abc";
        let cleaned = policy.sanitize_url(url);
        assert_eq!(cleaned, "https://example.com/download.zip?file_id=9988");
    }

    #[test]
    fn test_strip_all_params_if_only_tracking() {
        let policy = PrivacyDownloadPolicy::default();
        let url = "https://example.com/file.mp4?utm_source=newsletter&utm_medium=email";
        let cleaned = policy.sanitize_url(url);
        assert_eq!(cleaned, "https://example.com/file.mp4");
    }

    #[test]
    fn test_sanitize_referer() {
        let policy = PrivacyDownloadPolicy::default();
        let referer = "https://private.site.com/user/dashboard/secret-token?ref=leak";
        let cleaned = policy.sanitize_referer_header(referer);
        assert_eq!(cleaned, Some("https://private.site.com".to_string()));
    }
}
