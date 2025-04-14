#[macro_use]
extern crate rocket;

mod items;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(items::stage())
}
