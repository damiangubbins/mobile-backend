use rocket::State;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::tokio::time::sleep;
use std::time::Duration;

type Id = String;
type ItemList = Mutex<Vec<Item>>;
type Items = State<ItemList>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Item {
    id: Option<Id>,
    name: String,
    quantity: u32,
    unit: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ValidateRequest {
    _items: Vec<String>,
}

#[post("/", format = "json", data = "<item>")]
async fn create_item(item: Json<Item>, items: &Items) -> Value {
    let mut items = items.lock().await;
    let id = format!("b{:06}", items.len() + 1);
    let item = Item {
        id: Some(id.clone()),
        name: item.name.clone(),
        quantity: item.quantity,
        unit: item.unit.clone(),
    };
    items.push(item);

    json!({"status": "ok", "id": id})
}

#[get("/")]
async fn get_items(items: &Items) -> Json<Vec<Item>> {
    let items = items.lock().await;
    Json(items.clone())
}

#[get("/<id>")]
async fn get_item(id: &str, items: &Items) -> Option<Json<Item>> {
    let items = items.lock().await;
    for item in items.iter() {
        if item.id.as_ref() == Some(&id.to_string()) {
            return Some(Json(item.clone()));
        }
    }
    None
}

#[post("/validate", format = "json", data = "<_request>")]
async fn validate_items(_request: Json<ValidateRequest>, _items: &Items) -> Value {
    sleep(Duration::from_secs(3)).await;
    json!({"status": "ok"})
}

#[post("/send", format = "json", data = "<_request>")]
async fn send_items(_request: Json<ValidateRequest>, _items: &Items) -> Value {
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
    rocket::fairing::AdHoc::on_ignite("Items Stage", |rocket| async {
        rocket
            .mount(
                "/items",
                routes![create_item, get_items, get_item, validate_items, send_items],
            )
            .register("/items", catchers![not_found])
            .manage(ItemList::new(vec![]))
    })
}
