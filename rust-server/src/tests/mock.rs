use async_trait::async_trait;

use super::*;
use crate::web_client::Result as WebResult;

struct MockDatabase {}

impl MockDatabase {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl crate::database::DBAccessor for MockDatabase {}

struct MockWebClient {}

impl MockWebClient {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl crate::web_client::WebClient for MockWebClient {
    async fn get_quiz(&self, _user_id: UserId, _quiz_id: QuizId) -> WebResult<QuizConfig> {
        Ok(QuizConfig {
            name: "Test Quiz".to_owned(),
            questions: vec![
                Question {
                    answers: vec![
                        Answer {
                            correct_answer: true,
                            question_id: 1,
                            text: "Hello".to_owned(),
                        },
                        Answer {
                            correct_answer: false,
                            question_id: 1,
                            text: "World".to_owned(),
                        },
                    ],
                    quiestion_type: QuestionType::Poll,
                    quiz_id: 1,
                    text: "question1".to_owned(),
                    time: 60,
                },
                Question {
                    answers: vec![
                        Answer {
                            correct_answer: true,
                            question_id: 2,
                            text: "Hello".to_owned(),
                        },
                        Answer {
                            correct_answer: false,
                            question_id: 2,
                            text: "World".to_owned(),
                        },
                    ],
                    quiestion_type: QuestionType::Quiz,
                    quiz_id: 1,
                    text: "question2".to_owned(),
                    time: 15,
                },
            ],
        })
    }
}

pub async fn setup_mock() -> Client {
    let database = Box::new(MockDatabase::new());
    let web_client = Box::new(MockWebClient::new());
    setup(database, web_client).await
}
