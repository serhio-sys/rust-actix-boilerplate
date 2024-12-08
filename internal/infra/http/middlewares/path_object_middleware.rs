use std::sync::Arc;

use actix_web::{
    body::{ BoxBody, MessageBody },
    dev::{ ServiceRequest, ServiceResponse },
    middleware::Next,
    Error,
    HttpMessage,
    HttpResponse,
};
use serde::Serialize;

use crate::infra::http::resources::ErrorResponse;

use super::Findable;

pub async fn path_object_middleware<T, B>(
    service: Arc<dyn Findable<T>>,
    path_id_key: String,
    req: ServiceRequest,
    next: Next<B>
)
    -> Result<ServiceResponse<BoxBody>, Error>
    where B: MessageBody + 'static, T: Serialize + 'static
{
    let user_id = req.match_info().get(&path_id_key);
    if user_id.is_none() {
        return Ok(
            req.into_response(
                HttpResponse::BadRequest().json(
                    ErrorResponse::new_error(Some(format!("Not found id in path")))
                )
            )
        );
    }
    let result = path_object_insert(
        service,
        Arc::from(user_id.unwrap().parse::<i32>().unwrap()),
        &req
    );
    if result.is_err() {
        return Ok(
            req.into_response(HttpResponse::BadRequest().json(result.unwrap_err().to_string()))
        );
    }
    let res = next.call(req).await?;
    return Ok(res.map_into_boxed_body());
}

pub fn path_object_insert<T>(
    service: Arc<dyn Findable<T>>,
    user_id: Arc<i32>,
    req: &ServiceRequest
) -> Result<(), Box<dyn std::error::Error + std::marker::Send + Sync>>
    where T: Serialize + 'static
{
    match service.find_by_id(user_id) {
        Ok(obj) => {
            req.extensions_mut().insert::<T>(obj);
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}
