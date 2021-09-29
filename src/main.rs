mod routes;
mod database;
use actix_web::{web,App, HttpServer, };

//tipo de mascota
//raza
//nombre cliente
//telfonos


#[actix_web::main]
async fn main() -> Result<(),std::io::Error> {
   listener_server().await
}



async fn listener_server() -> std::io::Result<()>{
    println!("server conected");
    let app_data = web::Data::new(
       database::conexion_db::AppState{
       app_name:String::from("backend"),
       conexion: database::conexion_db::connection_db().unwrap()
    });

   HttpServer::new(move || {
       App::new()
            .app_data(app_data.clone())
            .service(routes::mascota_route::get_pets)
            .service(routes::cliente_route::get_user)
            .service(routes::mascota_route::add_pet)
   })
   .bind("localhost:8080")?
   .run()
   .await
}
