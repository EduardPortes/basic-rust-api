use crate::models::user::{RegisterUser, UpdateUser};
use crate::services::user_service::UserService;
use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let user_service = UserService::new(app_state);
    let result = user_service.get_all().await;

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
    let user_service = UserService::new(app_state);
    let result = user_service.create(user).await;

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
    let user_service = UserService::new(app_state);
    let result = user_service.update(id.into_inner(), user).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}
