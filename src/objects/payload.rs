
#[derive(Debug)]
pub struct Payload { url: String,
    referer: String,
    useragent: String,
    cookie: String,
}

impl Payload {
    pub fn new(url: String, referer: String, 
        useragent: String, cookie: String) -> Self {
        Payload {
            url,
            referer,
            useragent,
            cookie,
        }
    }

    pub fn get_url(&self) -> &str{
        &self.url
    }

    pub fn get_referer(&self) -> &str{
        &self.referer
    }

    pub fn get_useragent(&self) -> &str{
        &self.useragent
    }

    pub fn get_cookie(&self) -> &str{
        &self.cookie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload() {
        let p = Payload::new(
            "test_url".to_string(),
            "test_referer".to_string(),
            "test_useragent".to_string(),
            "test_cookie".to_string(),
        );
        assert_eq!(p.get_url(), "test_url");
        assert_eq!(p.get_referer(), "test_referer");
        assert_eq!(p.get_useragent(), "test_useragent");
        assert_eq!(p.get_cookie(), "test_cookie");
    }
}
