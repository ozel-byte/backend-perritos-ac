
use actix_web::{HttpResponse, Responder, get};


use std::sync::Arc;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct Mascota {
    pub id: i32,
    pub tipo: String,
    pub raza: String
 
 }

 #[derive(Serialize,Deserialize)]
pub struct MascotaBody{
   pub  tipo: String,
   pub  raza: String
 }

#[get("/pets")]
pub async fn get_pets( ) -> impl Responder{
    // match query_get_pet(&data.conexion){
    //     Ok(data) => HttpResponse::Ok().json(data),
    //     Err(_) => HttpResponse::Ok().body("No se pudo hacer la peticion")
    // }
    HttpResponse::Ok().body("ven ven")
}

// fn query_get_pet(pool: &Arc<Pool>) -> Result<Vec<Mascota>>{
//     let pool = pool.clone();
//     let mut conn = pool.get_conn().unwrap();
//    conn.query_map("SELECT * FROM mascota",
//      |(id,tipo,raza)| {
//         Mascota{
//             id,
//             tipo,
//             raza
//         }
//     })
// }



