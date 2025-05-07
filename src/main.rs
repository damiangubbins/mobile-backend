use seed::{seed_item_list, seed_order_list};

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::Response;

mod db;
mod items;
mod orders;
mod palets;
mod seed;

#[catch(404)]
fn not_found() -> rocket::serde::json::Value {
    rocket::serde::json::json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(items::stage())
        .attach(orders::stage())
        .attach(palets::stage())
        .register("/", catchers![not_found])
        .manage(seed_item_list())
        .manage(seed_order_list())
        .manage(db::PaletList::default())
}
