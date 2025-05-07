use seed::{seed_item_list, seed_order_list};

#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(items::stage())
        .attach(orders::stage())
        .attach(palets::stage())
        .register("/", catchers![not_found])
        .manage(seed_item_list())
        .manage(seed_order_list())
        .manage(db::PaletList::default())
}
