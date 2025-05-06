use crate::db::{Item, ItemList, Items};
use rocket::serde::json::{Json, Value, json};

#[post("/", format = "json", data = "<item>")]
async fn create_item(item: Json<Item<'static>>, items: Items<'_>) -> Value {
    let mut items = items.lock().await;
    let id = format!("B{:06}", items.len() + 1);
    let item = Item::new(
        Some(id.clone()),
        item.name.clone(),
        item.quantity,
        item.unit.clone(),
    );
    items.push(item);

    json!({"status": "ok", "id": id})
}

#[get("/")]
async fn get_items(items: Items<'_>) -> Json<Vec<Item>> {
    let items = items.lock().await;
    Json(items.clone())
}

#[get("/<id>")]
async fn get_item(id: &str, items: Items<'_>) -> Option<Json<Item<'static>>> {
    let items = items.lock().await;
    for item in items.iter() {
        if item.id.as_ref() == Some(&id.to_string()) {
            return Some(Json(item.clone()));
        }
    }
    None
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
            .mount("/items", routes![create_item, get_items, get_item])
            .register("/items", catchers![not_found])
            .manage(ItemList::new(vec![]))
    })
}
