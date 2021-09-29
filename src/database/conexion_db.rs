use mysql::*;

use std::sync::Arc;
 pub struct AppState{
  pub app_name: String,
  pub conexion: Arc<Pool>
}

pub fn connection_db()-> Option<Arc<Pool>> {
    let params = OptsBuilder::new()
                 .user(Some(""))
                 .db_name(Some(""))
                 .ip_or_hostname(Some(""))
                 .pass(Some(""));
        let pool = Pool::new(params).unwrap();
        let pool_arc = Arc::new(pool);
        println!("database conected");
      Option::Some(pool_arc)
}