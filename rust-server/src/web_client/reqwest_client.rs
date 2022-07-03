use reqwest::*;

use super::*;
use super::{Error, Result};

pub struct ReqwestClient {
    client: Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl WebClient for ReqwestClient {
    async fn get_quiz(&self, user_id: UserId, quiz_id: QuizId) -> Result<QuizConfig> {
        let url = format!("http://python_service:8888/api/user/{user_id}/quiz/{quiz_id}");
        println!("Sending a GET request to {url}");
        let body = self.client.get(url).send().await?.text().await?;
        println!("Received: {body}");
        let config = serde::json::from_str(&body)?;
        Ok(config)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Other(Box::new(error))
    }
}
