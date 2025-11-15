use crate::api::{CampusQuery, CampusesQuery, CollegeQuery, CollegesQuery};
use crate::error::Result;
use crate::{GroupsQuery, ScheduleQuery, error::Error};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) base_url: String,
    pub http_client: reqwest::Client,
    pub(crate) default_college_id: Option<u32>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
            default_college_id: None,
        }
    }

    pub fn with_client(base_url: &str, http_client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client,
            default_college_id: None,
        }
    }

    pub fn with_college(mut self, college_id: u32) -> Self {
        self.default_college_id = Some(college_id);
        self
    }

    pub fn colleges(&self) -> CollegesQuery {
        CollegesQuery::new(self)
    }

    pub fn college(&self) -> Result<CollegeQuery> {
        let college_id = self.default_college_id.ok_or_else(|| {
            Error::Validation(("No default college set. Use client.with_college() firsh".into()))
        })?;
        Ok(CollegeQuery::new(self, college_id))
    }

    pub fn campuses(&self) -> Result<CampusesQuery> {
        let college_id = self
            .default_college_id
            .ok_or_else(|| Error::Validation(("No default college set".into())))?;
        Ok(CampusesQuery::new(self, college_id))
    }

    pub fn campus(&self, campus_id: u32) -> Result<CampusQuery> {
        let _ = self.default_college_id.ok_or_else(|| {
            Error::Validation("No default college set. Use client.with_college() first".into())
        })?;
        Ok(CampusQuery::new(self, campus_id))
    }

    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.get(&url).send().await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let body = response.text().await?;
            Err(crate::error::Error::from_response(status.as_u16(), body))
        }
    }

    pub fn groups(&self, campus_id: u32) -> GroupsQuery {
        GroupsQuery::new(self, campus_id)
    }

    pub fn schedule(&self, group_id: u32) -> ScheduleQuery {
        ScheduleQuery::new(self, group_id)
    }

    pub fn today(&self, group_id: u32) -> ScheduleQuery {
        self.schedule(group_id).today()
    }

    pub fn tomorrow(&self, group_id: u32) -> ScheduleQuery {
        self.schedule(group_id).tomorrow()
    }
}
