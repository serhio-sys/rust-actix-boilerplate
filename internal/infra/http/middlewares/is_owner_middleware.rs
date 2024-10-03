use actix_web::{
    body::{ BoxBody, MessageBody },
    dev::{ ServiceRequest, ServiceResponse },
    middleware::Next,
    Error,
    HttpMessage,
    HttpResponse,
};
use serde::Serialize;

use crate::infra::domain::user::UserDTO;

use super::Userable;

pub async fn is_owner_middleware<T, B>(
    req: ServiceRequest,
    next: Next<B>
)
    -> Result<ServiceResponse<BoxBody>, Error>
    where B: MessageBody + 'static, T: Userable + Serialize + 'static
{
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
