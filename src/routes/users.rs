use actix_web::{web};

use crate::handlers::user::{get_all_users, create_user, update_user};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
    cfg.service(update_user);
}