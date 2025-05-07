use crate::db::{Order, Orders};
use rocket::serde::json::{Json, Value, json};

#[post("/", format = "json", data = "<order>")]
async fn create_order(order: Json<Order>, orders: &Orders) -> Value {
    let mut orders = orders.lock().await;
    let id = format!("O{:06}", orders.len() + 1);
    let order = Order::new(Some(id.clone()), order.items.clone());
    orders.push(order);

    json!({"status": "ok", "id": id})
}

#[get("/")]
async fn get_orders(orders: &Orders) -> Json<Vec<Order>> {
    let orders = orders.lock().await;
    Json(orders.clone())
}

#[get("/<id>")]
async fn get_order(id: &str, orders: &Orders) -> Option<Json<Order>> {
    let orders = orders.lock().await;
    for order in orders.iter() {
        if order.id.as_ref() == Some(&id.to_string()) {
            return Some(Json(order.clone()));
        }
    }
    None
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Orders Stage", |rocket| async {
        rocket.mount("/orders", routes![create_order, get_orders, get_order])
    })
}
