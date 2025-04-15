#[macro_use]
extern crate rocket;

mod items;
mod orders;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(items::stage())
        .attach(orders::stage())
}