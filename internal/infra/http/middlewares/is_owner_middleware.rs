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

use crate::infra::{ domain::user::UserDTO, http::resources::ErrorResponse };

use super::{ path_object_middleware::path_object_insert, Findable, Userable };

pub async fn is_owner_middleware<T, B>(
    service: Arc<dyn Findable<T>>,
    path_id_key: String,
    req: ServiceRequest,
    next: Next<B>
)
    -> Result<ServiceResponse<BoxBody>, Error>
    where B: MessageBody + 'static, T: Userable + Serialize + 'static
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
    let result = path_object_insert(service, user_id.unwrap().parse::<i32>().unwrap(), &req);
    if result.is_err() {
        return Ok(
            req.into_response(HttpResponse::BadRequest().json(result.unwrap_err().to_string()))
        );
    }
    let mut is_owner = false;
    if let Some(user) = req.extensions_mut().get::<UserDTO>() {
        if let Some(obj) = req.extensions_mut().get::<T>() {
            if obj.get_user_id() == user.get_user_id() {
                is_owner = true;
            }
        }
    }
    if is_owner {
        let res = next.call(req).await?;
        return Ok(res.map_into_boxed_body());
    } else {
        return Ok(req.into_response(HttpResponse::Forbidden().json("Permission denied")));
    }
}
