use actix_web::{get,Responder,HttpResponse};
// use super::mascota_route;

// struct Cliente{
//     name: String,
//     phone: i64,
//     pet: mascota_route::Mascota
// }

#[get("/user")]
pub async fn get_user() -> impl Responder{
    HttpResponse::Ok().body("all user in the db")
}

// #[post("add_user")]
// pub async fn add_user(bodyjson:String) -> impl Responder{
//     HttpResponse::Ok().body("created")
// }