use openai_chat::{completions, chat_completions};
use serde::{Deserialize, Serialize};
use std::env;

use actix_web::{web, HttpResponse, HttpRequest};
use crate::error::Result;

lazy_static::lazy_static! {
    // when set,it will be used in method decode_zstd_body_for_server while paring request body.
    static ref KEY: String = env::var("OPEN_AI_API_KEY").unwrap_or_else(|_| "".to_string());
}
#[derive(Debug,Deserialize,Serialize)]
pub struct AskRequest {
    pub question: String,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct AskReqponse{
    pub answer: String,
}
/// make request to chatgpt and get its response and use its reponse as a response
/// 
/// get OPEN-AI-APU TOKEN from environment variable
pub async fn ask(ask:web::Json<AskRequest>)->Result<HttpResponse> {
// let prompt=&ask.as_str();
let prompt=&ask.question;
 let result= chat_completions(prompt, &KEY).await?;
    println!("{:?}",result);

    Ok(HttpResponse::Ok().json(serde_json::to_string(&AskReqponse{answer:result})?))
}

