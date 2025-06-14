use serde::{Deserialize, Serialize};
use actix_web::{post, web, Responder};
use actix_web::http::StatusCode;
use crate::controller::build_rpc_response;
use crate::service::rag::handler::conversation_handler::conversation_call;
use crate::service::rag::handler::retriever_handler::similarity_search;

pub fn scope() -> actix_web::Scope {
    web::scope("/ai").service(recall).service(conversation)
}

#[post("/conversation.json")]
async fn conversation(request_data: web::Json<Conversation>) -> impl Responder {
    let request_data = request_data.into_inner();
    let collection_name = request_data.collection;
    let message = request_data.message;
    let answer = conversation_call(&message, &collection_name).await;
    match answer {
        Ok(answer) => {
            build_rpc_response(StatusCode::OK, None, Some(answer))
        }
        Err(err) => {
            build_rpc_response(StatusCode::INTERNAL_SERVER_ERROR, Some(err), None)
        }
    }
}

#[post("/rag/recall.json")]
async fn recall(request_data: web::Json<Conversation>) -> impl Responder {
    let request_data = request_data.into_inner();
    let collection_name = request_data.collection;
    let message = request_data.message;
    let search_result = similarity_search(&message, &collection_name).await;
    match search_result {
        Ok(docs) => {
            build_rpc_response(StatusCode::OK, None, Some(docs) )
        }
        Err(err) => {
            build_rpc_response(StatusCode::INTERNAL_SERVER_ERROR, Some(err), None)
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Conversation {
    collection: String,
    message: String,
}
