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

#[derive(Debug, serde::Deserialize)]
pub struct GetClassApi {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub description: String,
    pub user_id: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetStudentByIdApi {
    pub id: i32,
    pub name: String,
}

pub async fn get_student_by_id(
    user_id: i32,
    user_jwt: String,
) -> Result<GetStudentByIdApi, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!("{}/auth/users/{}", gateway_url, user_id))
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

    let user = match body {
        Ok(u) => u,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    Ok(user)
}

pub async fn get_teacher_classes(user_jwt: String) -> Result<Vec<GetClassApi>, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!("{}/class/classes/teachers", gateway_url))
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

    let classes = match body {
        Ok(c) => c,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    Ok(classes)
}

pub async fn get_enrolled_classes(user_jwt: String) -> Result<Vec<GetClassApi>, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!("{}/class/classes/students/enrolled", gateway_url))
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

    let classes = match body {
        Ok(c) => c,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    Ok(classes)
}

pub async fn get_class_by_id(
    user_jwt: String,
    class_id: i32,
) -> Result<Option<GetClassApi>, ServiceError> {
    let gateway_url = match std::env::var("GATEWAY_URL") {
        Ok(url) => url,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    let client = reqwest::Client::new();

    let user_result = client
        .get(format!("{}/class/classes/{}", gateway_url, class_id))
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

    let class = match body {
        Ok(c) => c,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    Ok(class)
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
