const COOKIE_KEY: &str = "id";

#[derive(Debug, Default)]
pub struct Cookie {
    value: String,
    domain: String,
    path: String,
    expires: Option<String>,
    max_age: Option<u64>,
    http_only: bool,
    secure: bool,
    same_site: SameSite,
}

impl Cookie {
    pub fn new(value: String) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.to_string();
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    pub fn set_expires(&mut self, expires: &str) {
        self.expires = Some(expires.to_string());
    }

    pub fn set_max_age(&mut self, max_age: u64) {
        self.max_age = Some(max_age);
    }

    pub fn set_http_only(&mut self) {
        self.http_only = true;
    }

    pub fn set_secure(&mut self) {
        self.secure = true;
    }

    pub fn set_same_site(&mut self, same_site: SameSite) {
        self.same_site = same_site;
    }

    pub fn build(&self) -> String {
        let mut cookie_header = format!("{}={}", COOKIE_KEY, self.value);

        cookie_header.push_str(&format!("; Domain={}", self.domain));

        cookie_header.push_str(&format!("; Path={}", self.path));

        if let Some(ref expires) = self.expires {
            cookie_header.push_str(&format!("; Expires={}", expires));
        }

        if let Some(max_age) = self.max_age {
            cookie_header.push_str(&format!("; Max-Age={}", max_age));
        }

        if self.http_only {
            cookie_header.push_str("; HttpOnly");
        }

        if self.secure {
            cookie_header.push_str("; Secure");
        }

        cookie_header.push_str(&format!("; SameSite={}", self.same_site.as_str()));

        cookie_header
    }

    pub fn clear(domain: &str, path: &str) -> Self {
        Self {
            value: "".to_string(),
            domain: domain.to_string(),
            path: path.to_string(),
            expires: Some("Thu, 01 Jan 1970 00:00:00 GMT".to_string()),
            max_age: None,
            http_only: true,
            secure: true,
            same_site: SameSite::None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum SameSite {
    #[default]
    Strict,
    Lax,
    None,
}

impl SameSite {
    pub fn as_str(&self) -> &'static str {
        match self {
            SameSite::Strict => "Strict",
            SameSite::Lax => "Lax",
            SameSite::None => "None",
        }
    }
}
