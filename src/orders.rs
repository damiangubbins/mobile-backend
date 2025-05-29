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

#[delete("/<id>")]
async fn delete_order(id: &str, orders: &Orders) -> Value {
    let mut orders = orders.lock().await;
    if let Some(pos) = orders
        .iter()
        .position(|x| x.id.as_ref() == Some(&id.to_string()))
    {
        orders.remove(pos);
        json!({"status": "ok", "message": "Order deleted"})
    } else {
        json!({"status": "error", "message": "Order not found"})
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Orders Stage", |rocket| async {
        rocket.mount(
            "/orders",
            routes![create_order, get_orders, get_order, delete_order],
        )
    })
}
