use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Child, Element};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DualisSession {
    arguments: String,
    client: Client,
}

impl DualisSession {
    pub async fn log_into_dualis() -> Result<Self, Box<dyn Error>> {
        let usrname = std::env::var("USRNAME").map_err(|_| "USRNAME environment variable not specified...")?;
        let pass = std::env::var("PASS").map_err(|_| "PASS environment variable not specified...")?;
        let form: [(&str, &str); 9] = [
            ("usrname", usrname.as_str()),
            ("pass", pass.as_str()),
            ("APPNAME", "CampusNet"),
            ("PRGNAME", "LOGINCHECK"),
            (
                "ARGUMENTS",
                "clino,usrname,pass,menuno,menu_type,browser,platform",
            ),
            ("clino", "000000000000001"),
            ("menuno", "000324"),
            ("browser", ""),
            ("platform", ""),
        ];

        let client = Client::builder().cookie_store(true).build()?;
        let response = client
            .post(format!("https://{}/scripts/mgrqispi.dll", "dualis.dhbw.de"))
            .form(&form)
            .send()
            .await?
            .error_for_status()?;

        if !response.headers().contains_key("REFRESH") {
            return Err("No refresh. Indicates invalid credentials".into());
        }

        let arguments = response.headers()["REFRESH"]
            .to_str()?
            .chars()
            .skip_while(|c| *c != '-')
            .collect();
        Ok(Self { arguments, client })
    }

    pub async fn get_semesters(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("https://{}/scripts/mgrqispi.dll", "dualis.dhbw.de"))
            .query(&[
                ("APPNAME", "CampusNet"),
                ("PRGNAME", "COURSERESULTS"),
                ("ARGUMENTS", self.arguments.as_str()),
            ])
            .send()
            .await?;

        let body = response.text().await?;

        let document = Document::from(body.as_str());

        let out = document
            .find(Child(Attr("id", "semester"), Element))
            .map(|node| node.text())
            .collect();
        Ok(out)
    }
}

impl Display for DualisSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DualisSession{{arguments={},client_cookies=?}}",
            self.arguments
        )
    }
}
