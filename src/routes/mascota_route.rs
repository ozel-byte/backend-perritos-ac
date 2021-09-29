
use actix_web::{HttpResponse, Responder, get, post, web::{self}};
use mysql::*;
use mysql::prelude::*;
use crate::database;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct Mascota {
    pub id: i32,
    pub tipo: String,
    pub raza: String
 
 }

 #[derive(Deserialize)]
pub struct MascotaBody{
   pub  tipo: String,
   pub  raza: String
 }

#[get("/pets")]
pub async fn get_pets(data: web::Data<database::conexion_db::AppState> ) -> impl Responder{
    let pool = &data.conexion;
    let pool = pool.clone();
    let mut conn = pool.get_conn().unwrap();
    let pets_array = conn.query_map("select * from mascota",
     |(id,tipo,raza)| {
        Mascota{
            id,
            tipo,
            raza
        }
     }).unwrap();

     HttpResponse::Ok().json(pets_array)   
}

#[post("/add_pet")]
pub async fn add_pet(body: web::Json<MascotaBody>, data: web::Data<database::conexion_db::AppState>) -> impl Responder{
    
    println!("tipo: {}",body.tipo);
    println!("Raza: {}",body.raza);
    HttpResponse::Ok().body("se agrego correctamente")
}



