use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::response;

#[derive(Deserialize, Serialize, Debug)]
pub struct SdpRequest {
    pub sdp_type: String,
    pub sdp: String,
}

pub async fn sdp_service(
    payload: web::Json<SdpRequest>,
) -> web::Json<response::Response<SdpRequest>> {
    web::Json(response::Response {
        data: payload.into_inner(),
        error: None,
    })
}
