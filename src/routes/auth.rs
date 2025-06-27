use crate::auth::handler::oauth_token;
use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(oauth_token);
}
