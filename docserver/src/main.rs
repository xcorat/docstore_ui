mod config;
mod db;
mod routes;

#[cfg(test)]
mod tests;

use rocket::{launch, routes};
use crate::config::AppState;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .manage(AppState::new())
        .mount("/", routes![
            routes::index, 
            routes::get_file, 
            routes::upload_files,
            routes::list_clients,
            routes::get_client,
            routes::list_returns,
            routes::get_return
        ])
        .mount("/config", routes![routes::get_root_path, routes::set_root_path])
}
