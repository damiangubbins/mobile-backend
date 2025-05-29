use crate::db::{Pallet, Pallets};
use rocket::serde::json::{Json, Value, json};

#[post("/", format = "json", data = "<pallet>")]
async fn generate_pallet(pallet: Json<Pallet>, pallets: &Pallets) -> Value {
    let mut pallets = pallets.lock().await;
    let id = format!("P{:06}", pallets.len() + 1);
    let pallet = Pallet::new(
        Some(id.clone()),
        pallet.order_id.clone(),
        pallet.item_ids.clone(),
    );
    pallets.push(pallet);

    json!({"status": "ok", "id": id})
}

#[get("/")]
async fn get_pallets(pallets: &Pallets) -> Json<Vec<Pallet>> {
    let pallets = pallets.lock().await;
    Json(pallets.clone())
}

#[get("/<id>")]
async fn get_pallet(id: &str, pallets: &Pallets) -> Option<Json<Pallet>> {
    let pallets = pallets.lock().await;
    for plate in pallets.iter() {
        if plate.id.as_ref() == Some(&id.to_string()) {
            return Some(Json(plate.clone()));
        }
    }
    None
}

#[get("/by_order/<order_id>")]
async fn get_pallets_by_order(order_id: &str, pallets: &Pallets) -> Json<Vec<Pallet>> {
    let pallets = pallets.lock().await;
    let filtered: Vec<Pallet> = pallets
        .iter()
        .filter(|p| p.order_id == order_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Pallets Stage", |rocket| async {
        rocket.mount(
            "/pallets",
            routes![
                generate_pallet,
                get_pallets,
                get_pallet,
                get_pallets_by_order
            ],
        )
    })
}
