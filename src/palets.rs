use crate::db::{Palet, Palets};
use rocket::serde::json::{Json, Value, json};

#[post("/", format = "json", data = "<palet>")]
async fn generate_palet(palet: Json<Palet>, plates: &Palets) -> Value {
    let mut plates = plates.lock().await;
    let id = format!("P{:06}", plates.len() + 1);
    let palet = Palet::new(Some(id.clone()), palet.item_ids.clone());
    plates.push(palet);

    json!({"status": "ok", "id": id})
}

#[get("/")]
async fn get_palets(plates: &Palets) -> Json<Vec<Palet>> {
    let plates = plates.lock().await;
    Json(plates.clone())
}

#[get("/<id>")]
async fn get_palet(id: &str, plates: &Palets) -> Option<Json<Palet>> {
    let plates = plates.lock().await;
    for plate in plates.iter() {
        if plate.id.as_ref() == Some(&id.to_string()) {
            return Some(Json(plate.clone()));
        }
    }
    None
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Palets Stage", |rocket| async {
        rocket.mount("/palets", routes![generate_palet, get_palets, get_palet])
    })
}
