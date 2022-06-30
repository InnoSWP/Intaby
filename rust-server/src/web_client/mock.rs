use async_trait::async_trait;

use crate::model::*;
use crate::web_client::Result as WebResult;

pub struct MockWebClient {}

impl MockWebClient {
    pub fn new() -> Self {
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
                    question_type: QuestionType::Poll,
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
                    question_type: QuestionType::Quiz,
                    quiz_id: 1,
                    text: "question2".to_owned(),
                    time: 15,
                },
            ],
        })
    }
}
