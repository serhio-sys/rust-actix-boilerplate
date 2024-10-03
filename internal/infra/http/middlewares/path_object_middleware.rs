use actix_web::{
    body::{ BoxBody, MessageBody },
    dev::{ ServiceRequest, ServiceResponse },
    middleware::Next,
    web,
    Error,
    HttpMessage,
    HttpResponse,
};
use serde::Serialize;

use super::Findable;

pub async fn is_owner_middleware<T, B>(
    service: &dyn Findable<T>,
    user_id: web::Path<i32>,
    req: ServiceRequest,
    next: Next<B>
)
    -> Result<ServiceResponse<BoxBody>, Error>
    where B: MessageBody + 'static, T: Serialize + 'static
{
    match service.find_by_id(*user_id) {
        Ok(obj) => {
            req.extensions_mut().insert::<T>(obj);
        }
        Err(e) => {
            return Ok(req.into_response(HttpResponse::BadRequest().json(e.to_string())));
        }
    }
    let res = next.call(req).await?;
    return Ok(res.map_into_boxed_body());
}
