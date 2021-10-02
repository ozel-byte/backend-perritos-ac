use super::mascota_route;
use crate::database;
use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Cliente {
    name: String,
    phone: String,
    pet: mascota_route::MascotaBody,
}

#[get("/user")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("all user in the db")
}

#[post("/add_user")]
pub async fn add_user(
    data: web::Json<Cliente>,
    pool: web::Data<database::conexion_db::AppState>,
) -> impl Responder {
    let response = query_add_user_cita(&pool.conexion, data);
    match response {
        Ok(d) => HttpResponse::Ok().body(d),
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}

fn query_add_user_cita(
    pool: &Arc<Pool>,
    data: web::Json<Cliente>,
) -> std::result::Result<String, mysql::error::Error> {
    let pool = pool.clone();
    let mut conn = pool.get_conn().unwrap();
    let resp = conn
        .exec_drop(
            "INSERT INTO mascota (tipo,raza) VALUES (:tipo,:raza)",
            params! {"tipo" => &data.pet.tipo, "raza" => &data.pet.raza},
        )
        .and_then(|_| Ok(conn.last_insert_id()));

    match resp {
        Ok(d) => {
            let r = conn.exec_drop(
                "INSERT INTO cliente (name, phone, id_pet) VALUES (:name, :phone, :id_pet)",
                params! {
                    "name" => &data.name,
                    "phone" => &data.phone,
                    "id_pet" => d
                },
            );
            match r {
                Ok(()) => Ok(String::from("Se agrego correctamente")),
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => Err(e),
    }
}

