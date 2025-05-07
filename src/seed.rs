use crate::db::{Item, ItemList, Order, OrderList};
use rocket::tokio::sync::Mutex;

pub fn seed_item_list() -> ItemList {
    let initial_items = vec![
        Item::new(
            Some("B000001".to_string()),
            "Huevos".to_string(),
            12,
            "unidades".to_string(),
        ),
        Item::new(
            Some("B000002".to_string()),
            "Huevos".to_string(),
            12,
            "unidades".to_string(),
        ),
        Item::new(
            Some("B000003".to_string()),
            "Huevos".to_string(),
            12,
            "unidades".to_string(),
        ),
        Item::new(
            Some("B000004".to_string()),
            "Huevos".to_string(),
            12,
            "unidades".to_string(),
        ),
        Item::new(
            Some("B000005".to_string()),
            "Huevos".to_string(),
            12,
            "unidades".to_string(),
        ),
        Item::new(
            Some("B000006".to_string()),
            "Leche en Polvo".to_string(),
            4,
            "kg".to_string(),
        ),
        Item::new(
            Some("B000007".to_string()),
            "Leche en Polvo".to_string(),
            4,
            "kg".to_string(),
        ),
        Item::new(
            Some("B000008".to_string()),
            "Leche en Polvo".to_string(),
            4,
            "kg".to_string(),
        ),
        Item::new(
            Some("B000009".to_string()),
            "Leche en Polvo".to_string(),
            4,
            "kg".to_string(),
        ),
        Item::new(
            Some("B000010".to_string()),
            "Leche en Polvo".to_string(),
            4,
            "kg".to_string(),
        ),
    ];
    Mutex::new(initial_items)
}

pub fn seed_order_list() -> OrderList {
    let initial_orders = vec![Order::new(
        Some("O000001".to_string()),
        vec![
            Item::new(None, "Huevos".to_string(), 48, "unidades".to_string()),
            Item::new(None, "Leche en Polvo".to_string(), 20, "kg".to_string()),
        ],
    )];
    Mutex::new(initial_orders)
}
