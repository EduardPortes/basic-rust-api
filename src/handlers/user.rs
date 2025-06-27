use crate::AppState;
use crate::db::user_db;
use crate::models::user::{RegisterUser, UpdateUser};
use actix_web::{HttpResponse, Responder, get, post, put, web};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = user_db::get_all(app_state).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}

#[post("/users")]
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    let result = user_db::create(app_state, user).await;

    match result {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

#[put("/users/{id}")]
async fn update_user(
    app_state: web::Data<AppState>,
    id: web::Path<u64>,
    user: web::Json<UpdateUser>,
) -> impl Responder {
    let result = user_db::update(app_state, id.into_inner(), user).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}
