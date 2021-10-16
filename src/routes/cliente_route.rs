use super::mascota_route;
use actix_web::{get,post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
// use mysql::prelude::*;
// use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cliente {
    name: String,
    phone: String,
    pet: mascota_route::MascotaBody,
}

#[derive(Serialize)]
pub struct ClienteGet {
    id_c: i32,
    name: String,
    phone: String,
    pet: mascota_route::Mascota,
}

#[get("/user")]
pub async fn get_user() -> impl Responder {
    let params = OptsBuilder::new()
        .user(Some(""))
        .db_name(Some(""))
        .ip_or_hostname(Some(""))
        .pass(Some(""));
    let pool = Pool::new(params).unwrap();
    let mut pool = pool.get_conn().unwrap();
    let result = pool.query_map("SELECT mascota.id as id_m, tipo,raza, cliente.id as id_c, name,phone from mascota inner join cliente ON mascota.id = cliente.id_pet",
     |(id_m, tipo, raza, id_c, name, phone):(i32,String,String,i32,String,String)| {
        ClienteGet{
                        id_c,
                        name,
                        phone,
                        pet: mascota_route::Mascota{
                            id: id_m,
                            tipo,
                            raza
                        }
                    }
     }
    ).unwrap();
    //     let pool = &pool.conexion;
    //     let pool = pool.clone();
    //     let mut conn = pool.get_conn().unwrap();
    //     let result = conn.query_map("SELECT mascota.id as id_m, tipo,raza, cliente.id as id_c, name,phone from mascota inner join cliente ON mascota.id = cliente.id_pet",
    //     |(id_m, tipo, raza, id_c, name, phone):(i32,String,String,i32,String,String)| {
    //         ClienteGet{
    //             id_c,
    //             name,
    //             phone,
    //             pet: mascota_route::Mascota{
    //                 id: id_m,
    //                 tipo,
    //                 raza
    //             }
    //         }
    //     },
    // ).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/add_user")]
pub async fn add_user(data: web::Json<Cliente>) -> impl Responder {
    let params = OptsBuilder::new()
        .user(Some("administrador"))
        .db_name(Some("perritosac"))
        .ip_or_hostname(Some("167.172.146.90"))
        .pass(Some("p0Hb67269.YCx1fcqwS540Obv"));
    let pool = Pool::new(params).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let resp = conn.exec_drop("INSERT INTO mascota (tipo,raza) VALUES (:tipo,:raza)", 

        params!{"tipo" => &data.pet.tipo, "raza" => &data.pet.raza}

    ).and_then(|_| Ok(conn.last_insert_id()));

    let q = resp.unwrap();
    let r = conn.exec_drop(
        "INSERT INTO cliente (name, phone, id_pet) VALUES (:name, :phone, :id_pet)",
        params! {
            "name" => &data.name,
            "phone" => &data.phone,
            "id_pet" => q
        },
    );
    let resp_v2 = match r {
        Ok(_) => {"Se registro"},
        Err(_) => {"Error al registro"}
    } ;
    HttpResponse::Ok().body(resp_v2)
}

// async fn query_add_user_cita(
//     pool: &Arc<Pool>,
//     data: web::Json<Cliente>,
// ) -> std::result::Result<String, mysql::error::Error> {
//     let pool = pool.clone();
//     let mut conn = pool.get_conn().unwrap();
//     let resp = conn
//         .exec_drop(
//             "INSERT INTO mascota (tipo,raza) VALUES (:tipo,:raza)",
//             params! {"tipo" => &data.pet.tipo, "raza" => &data.pet.raza},
//         )
//         .and_then(|_| Ok(conn.last_insert_id()));

//     match resp {
//         Ok(d) => {
//             let r = conn.exec_drop(
//                 "INSERT INTO cliente (name, phone, id_pet) VALUES (:name, :phone, :id_pet)",
//                 params! {
//                     "name" => &data.name,
//                     "phone" => &data.phone,
//                     "id_pet" => d
//                 },
//             );
//             match r {
//                 Ok(()) => Ok(String::from("Se agrego correctamente")),
//                 Err(e) => {
//                     Err(e)
//                 }
//             }
//         }
//         Err(e) => Err(e),
//     }
// }
