//! One-object transport signing; no credential values are logged or persisted.
//! Algorithm/test vector: https://docs.aws.amazon.com/AmazonS3/latest/developerguide/sigv4-query-string-auth.html
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hmac(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut k = [0u8; 64];
    if key.len() > 64 {
        k[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut inner = [0u8; 64];
    let mut outer = [0u8; 64];
    for i in 0..64 {
        inner[i] = k[i] ^ 0x36;
        outer[i] = k[i] ^ 0x5c;
    }
    let mut h = Sha256::new();
    h.update(inner);
    h.update(message);
    let intermediate = h.finalize();
    let mut h = Sha256::new();
    h.update(outer);
    h.update(intermediate);
    h.finalize().into()
}
fn encode(s: &str, slash: bool) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) || (!slash && b == b'/') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
pub fn url(
    method: &str,
    host: &str,
    region: &str,
    key: &str,
    access: &str,
    secret: &str,
    token: Option<&str>,
    date: &str,
    expires: u32,
) -> String {
    assert_eq!(date.len(), 16);
    assert!(expires <= 900 || cfg!(test));
    let day = &date[..8];
    let scope = format!("{day}/{region}/s3/aws4_request");
    let mut q = BTreeMap::new();
    q.insert("X-Amz-Algorithm", "AWS4-HMAC-SHA256".to_owned());
    q.insert("X-Amz-Credential", format!("{access}/{scope}"));
    q.insert("X-Amz-Date", date.to_owned());
    q.insert("X-Amz-Expires", expires.to_string());
    q.insert("X-Amz-SignedHeaders", "host".to_owned());
    if let Some(t) = token {
        q.insert("X-Amz-Security-Token", t.to_owned());
    }
    let query = q
        .iter()
        .map(|(k, v)| format!("{}={}", encode(k, true), encode(v, true)))
        .collect::<Vec<_>>()
        .join("&");
    let path = format!("/{}", encode(key, false));
    let request = format!("{method}\n{path}\n{query}\nhost:{host}\n\nhost\nUNSIGNED-PAYLOAD");
    let to_sign = format!(
        "AWS4-HMAC-SHA256\n{date}\n{scope}\n{}",
        hex(&Sha256::digest(request.as_bytes()))
    );
    let k_date = hmac(format!("AWS4{secret}").as_bytes(), day.as_bytes());
    let k_region = hmac(&k_date, region.as_bytes());
    let k_service = hmac(&k_region, b"s3");
    let k_sign = hmac(&k_service, b"aws4_request");
    let signature = hex(&hmac(&k_sign, to_sign.as_bytes()));
    format!("https://{host}{path}?{query}&X-Amz-Signature={signature}")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hmac_rfc4231() {
        assert_eq!(
            hex(&hmac(&[0x0b; 20], b"Hi There")),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }
    #[test]
    fn aws_documented_signature() {
        let u = url(
            "GET",
            "examplebucket.s3.amazonaws.com",
            "us-east-1",
            "test.txt",
            "AKIAIOSFODNN7EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            None,
            "20130524T000000Z",
            86400,
        );
        assert!(u.ends_with(
            "X-Amz-Signature=aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404"
        ));
    }
    #[test]
    fn uri_rules() {
        assert_eq!(encode("+/=%", true), "%2B%2F%3D%25");
        assert_eq!(encode("a/b c", false), "a/b%20c");
    }
}
