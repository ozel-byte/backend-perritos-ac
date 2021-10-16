mod database;
mod routes;
use actix_cors::Cors;
use actix_web::{get,web, App, HttpServer};
use std::sync::Mutex;

struct AppStateCounter{
   counter: Mutex<i32>
}

#[get("/count+")]
async fn counter(data_count: web::Data<AppStateCounter>) -> String{
   let mut count = data_count.counter.lock().unwrap();
   *count +=1;
   format!("Request number: {}",count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   // let app_data = web::Data::new(database::conexion_db::AppState {
   //    app_name: String::from("backend"),
   //    conexion: database::conexion_db::connection_db().unwrap(),
   // });
      let counter_data = web::Data::new(AppStateCounter{
         counter: Mutex::new(0)
      });

   HttpServer::new(move || {
      println!("server on");
      let cors = Cors::permissive();
      App::new()
         .wrap(cors)
         .app_data(counter_data.clone())
         .service(counter)
         .service(routes::mascota_route::get_pets)
         .service(routes::cliente_route::get_user)
         .service(routes::cliente_route::add_user)
   })
   .bind(("localhost", 8080))?
   .run()
   .await
}

