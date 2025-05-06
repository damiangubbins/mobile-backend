use crate::db::{Order, OrderList, Orders};
use rocket::serde::Deserialize;
use rocket::serde::json::{Json, Value, json};
use rocket::tokio::time::{Duration, sleep};

#[derive(Deserialize)]
struct ValidateRequest {
    _order_id: String,
    _scanned_items: Vec<ScannedItem>,
}

#[derive(Deserialize)]
struct ScannedItem {
    _id: String,
}

#[post("/", format = "json", data = "<order>")]
async fn create_order(order: Json<Order>, orders: &Orders) -> Value {
    let mut orders = orders.lock().await;
    let id = format!("P{:06}", orders.len() + 1);
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

#[post("/validate", format = "json", data = "<_request>")]
async fn validate_items(_request: Json<ValidateRequest>) -> Value {
    sleep(Duration::from_secs(3)).await;
    json!({"status": "ok"})
}

#[post("/send", format = "json", data = "<_request>")]
async fn send_items(_request: Json<ValidateRequest>) -> Value {
    sleep(Duration::from_secs(3)).await;
    json!({"status": "ok"})
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Orders Stage", |rocket| async {
        rocket
            .mount(
                "/orders",
                routes![
                    create_order,
                    get_orders,
                    get_order,
                    validate_items,
                    send_items
                ],
            )
            .register("/", catchers![not_found])
            .manage(OrderList::default())
    })
}
