
use actix_web::{web,get, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use crate::database;
use serde::{Serialize,Deserialize};


pub struct Mascota {
    pub tipo: TipoMascota,
    pub raza: String
 
 }

 pub enum TipoMascota{
     Perro,
     Gatto
 }

#[get("/mascotas")]
pub async fn get_pets(data: web::Data<database::conexion_db::AppState> ) -> impl Responder{
//     let pool = &data.conexion;
//     let pool = pool.clone();
//     let mut conn = pool.get_conn().unwrap();
//     let selected = conn.query_map("SELECT * FROM categoria", 
//     |(id_categoria,clasificacion,categoria,sub_categoria)|{
//         Mascota{
//             id_categoria,
//            clasificacion,
//            categoria,
//            sub_categoria
//        }
//     } 
// ).unwrap();

// HttpResponse::Ok().json(selected)
    // web::Json(Mascota{
    //     id_categoria: String::from("a"),
    //     clasificacion:String::from("a"),
    //     categoria:String::from("v"),
    //     sub_categoria:String::from("t"),
    // })
    HttpResponse::Ok().body("ey")
}



