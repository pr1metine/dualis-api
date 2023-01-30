use crate::error::Error;
use log::info;
#[derive(Debug, Clone)]
pub struct DualisCredentials {
    url: String,
    usrname: String,
    pass: String,
}

impl DualisCredentials {
    pub fn get_hostname() -> String {
        std::env::var("HOST").unwrap_or("127.0.0.1".into())
    }
    pub fn get_port() -> u16 {
        std::env::var("PORT")
            .unwrap_or("8080".into())
            .parse()
            .unwrap_or(8080)
    }
    pub fn get_root_path() -> String {
        format!("v{}", std::env!("CARGO_PKG_VERSION"))
    }
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        fn get_required_env(env_name: &'static str) -> Result<String, Error> {
            std::env::var(env_name).map_err(|e| Error::env_error(e.into(), env_name))
        }
        let url = std::env::var("DUALIS_URL").unwrap_or("dualis.dhbw.de".into());
        let usrname = get_required_env("USRNAME")?;
        let pass = get_required_env("PASS")?;

        info!("Loaded dualis credentials from environment variables!");
        Ok(Self { url, usrname, pass })
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    pub fn usrname(&self) -> &str {
        self.usrname.as_ref()
    }

    pub fn pass(&self) -> &str {
        self.pass.as_ref()
    }
}
