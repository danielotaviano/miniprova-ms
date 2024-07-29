use reqwest::header::{HeaderValue, AUTHORIZATION};

use crate::errors::ServiceError;

#[derive(Debug, serde::Deserialize)]
pub struct GetExamApi {
    pub name: String,
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
