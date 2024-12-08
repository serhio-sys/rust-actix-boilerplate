use std::sync::Arc;

use actix_web::{
    body::{ BoxBody, MessageBody },
    dev::{ ServiceRequest, ServiceResponse },
    middleware::Next,
    Error,
    HttpMessage,
    HttpResponse,
};
use config::CONFIGURATION;
use jsonwebtoken::{ decode, DecodingKey, Validation };

use crate::services::{ auth_service::{ AuthService, Claims }, user_service::UserService };

pub async fn auth_middleware<B>(
    user_service: Arc<UserService>,
    auth_service: Arc<AuthService>,
    req: ServiceRequest,
    next: Next<B>
) -> Result<ServiceResponse<BoxBody>, Error>
    where B: MessageBody + 'static
{
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        let token_str = auth_header.to_str().unwrap_or("").replace("Bearer ", "");
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            &token_str,
            &DecodingKey::from_secret(CONFIGURATION.jwt_secret.as_ref()),
            &validation
        );

        match token_data {
            Ok(data) => {
                let claims = data.claims;
                if auth_service.check(claims.clone()) {
                    match user_service.find_by_id(claims.user_id.clone()) {
                        Ok(user) => {
                            req.extensions_mut().insert(user);
                            req.extensions_mut().insert(claims.clone());
                            let res = next.call(req).await?;
                            return Ok(res.map_into_boxed_body());
                        }
                        Err(e) => {
                            return Ok(
                                req.into_response(
                                    HttpResponse::BadRequest()
                                        .json(e.to_string())
                                        .map_into_boxed_body()
                                )
                            );
                        }
                    }
                } else {
                    return Ok(
                        req.into_response(
                            HttpResponse::Unauthorized().finish().map_into_boxed_body()
                        )
                    );
                }
            }
            Err(_) => {
                return Ok(
                    req.into_response(HttpResponse::Unauthorized().finish().map_into_boxed_body())
                );
            }
        }
    } else {
        return Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_boxed_body()));
    }
}
