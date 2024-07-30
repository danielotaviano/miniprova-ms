use reqwest::header::{HeaderValue, AUTHORIZATION};

use crate::errors::ServiceError;

#[derive(Debug, serde::Deserialize)]
pub struct GetExamApi {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetExamQuestionAnswerApi {
    pub id: i32,
    pub answer: String,
    pub is_correct: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetExamQuestionApi {
    pub id: i32,
    pub question: String,
    pub answers: Vec<GetExamQuestionAnswerApi>,
}

pub async fn get_exam_questions(
    exam_id: i32,
    user_jwt: String,
) -> Result<Vec<GetExamQuestionApi>, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!(
            "{}/question/exams/{}/questions",
            gateway_url, exam_id
        ))
        .header(
            AUTHORIZATION.as_str(),
            HeaderValue::from_str(&user_jwt).unwrap(),
        )
        .send()
        .await;

    let response = match user_result {
        Ok(r) => r,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let body = response.json().await;

    let questions = match body {
        Ok(q) => q,
        Err(e) => return Err(ServiceError::InternalServerError),
    };

    Ok(questions)
}

pub async fn get_exam(exam_id: i32, user_jwt: String) -> Result<GetExamApi, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!("{}/question/exams/{}", gateway_url, exam_id))
        .header(
            AUTHORIZATION.as_str(),
            HeaderValue::from_str(&user_jwt).unwrap(),
        )
        .send()
        .await;

    println!("result:{:?}", user_result);

    let response = match user_result {
        Ok(r) => r,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let body = response.json().await;

    let exam = match body {
        Ok(e) => e,
        Err(e) => return Err(ServiceError::InternalServerError),
    };

    Ok(exam)
}
