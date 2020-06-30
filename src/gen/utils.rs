
use crate::objects::payload;

pub fn generate_p(urls: &[String], s: &str) -> Vec<payload::Payload> {
    urls.iter().map(
        |url| payload::Payload::new(
            format!("{}{}{}", url, "?q=", s),
            s.to_string(),
            s.to_string(),
            format!("{}{}", "vuln=", s),
        )
    ).collect()
}

