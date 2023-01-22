use reqwest::{Client, Response};
use select::document::Document;
use select::predicate::{Attr, Child, Name, Predicate};
use std::{error::Error, fmt::Display};

use crate::data::DHBWCourse;
use crate::data::DHBWSemester;

#[derive(Debug)]
pub struct DualisSession<'a> {
    url: &'a str,
    arguments: String,
    client: Client,
}

impl<'a> DualisSession<'a> {
    pub async fn log_into_dualis(
        url: &'a str,
        usrname: &str,
        pass: &str,
    ) -> Result<DualisSession<'a>, Box<dyn Error>> {
        let form: [(&str, &str); 9] = [
            ("usrname", usrname),
            ("pass", pass),
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
            .post(format!("https://{}/scripts/mgrqispi.dll", url))
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
            .take(26)
            .collect();
        Ok(Self {
            url,
            arguments,
            client,
        })
    }

    async fn send_get_request(
        &self,
        prgname: &str,
        additional_arguments: &str,
    ) -> Result<Response, reqwest::Error> {
        let arguments = format!("{},{}", self.arguments, additional_arguments);

        self.client
            .get(format!("https://{}/scripts/mgrqispi.dll", self.url))
            .query(&[
                ("APPNAME", "CampusNet"),
                ("PRGNAME", prgname),
                ("ARGUMENTS", arguments.as_str()),
            ])
            .send()
            .await
    }

    pub async fn get_semesters(&self) -> Result<Vec<DHBWSemester>, Box<dyn Error>> {
        let response = self.send_get_request("COURSERESULTS", "").await?;

        let body = response.text().await?;

        let document = Document::from(body.as_str());

        let out = document
            .find(Child(Attr("id", "semester"), Name("option")))
            .map(|node| {
                DHBWSemester::new(
                    String::from(node.attr("value").unwrap_or("MISSING")),
                    node.text(),
                )
            })
            .collect();
        Ok(out)
    }

    pub async fn get_courses(&self, semester_id: &str) -> Result<Vec<DHBWCourse>, Box<dyn Error>> {
        let arguments = format!("-N{}", semester_id);
        let response = self
            .send_get_request("COURSERESULTS", arguments.as_str())
            .await?;

        let body = response.text().await?;
        let document = Document::from(body.as_str());

        let out = document
            .find(Child(Name("tbody"), Name("tr")))
            .map(|n| {
                n.children()
                    .filter(|c| c.is(Name("td")))
                    .map(|c| c.text())
                    .map(|t| String::from(t.trim()))
                    .collect::<Vec<_>>()
            })
            .filter(|v| v.len() >= 4)
            .map(|v| DHBWCourse::new(v[0].clone(), v[1].clone(), Some(v[2].clone()), v[3].clone()))
            .collect();
        Ok(out)
    }

    pub async fn get_overview(&self) -> Result<Vec<DHBWCourse>, Box<dyn Error>> {
        let response = self
            .send_get_request(
                "STUDENT_RESULT",
                "-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000",
            )
            .await?;

        let body = response.text().await?;
        let document = Document::from(body.as_str());

        let out = document
            .find(Name("tbody").child(Name("tr")))
            .skip(2)
            .filter(|n| n.children().filter(|c| c.is(Name("td"))).count() >= 6)
            .map(|n| {
                let mut it = n.children().filter(|c| c.is(Name("td")));
                let id = it.next().unwrap().text().trim().to_owned();
                let description_parent = it.next().unwrap();
                let description = if let Some(c) = description_parent.find(Name("a")).next() {
                    c
                } else {
                    description_parent
                }.text().trim().to_owned();
                it.next();
                let ects_points = it.next().unwrap().text().trim().to_owned();
                let grade = it.next().unwrap().text().trim().to_owned();
                DHBWCourse::new(id, description, Some(grade), ects_points)
            })
            .collect::<Vec<_>>();

        Ok(out)
    }
}

impl Display for DualisSession<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DualisSession{{arguments={},client_cookies=?}}",
            self.arguments
        )
    }
}
