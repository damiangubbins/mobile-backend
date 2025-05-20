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
    let initial_orders = vec![
        Order::new(
            Some("O000001".to_string()),
            vec![
                Item::new(None, "Huevos".to_string(), 24, "unidades".to_string()),
                Item::new(None, "Leche en Polvo".to_string(), 12, "kg".to_string()),
            ],
        ),
        Order::new(
            Some("O000002".to_string()),
            vec![
                Item::new(
                    None,
                    "100300 - IPAC M4 - TAPA ENVASE YOGUR LITRO ARTISAN".to_string(),
                    20,
                    "unidades".to_string(),
                ),
                Item::new(
                    None,
                    "100700 - IPAC M4 - ENVASE YOGUR LITRO ARTISAN".to_string(),
                    20,
                    "unidades".to_string(),
                ),
                Item::new(
                    None,
                    "100908 - IPAC CH1 - IGF - ENVASE YOGUR GRIEGO FRUTILLA".to_string(),
                    100,
                    "unidades".to_string(),
                ),
                Item::new(
                    None,
                    "101108 - IPAC CH3 - ENVASE YOGUR DESCREMADO 360 G".to_string(),
                    100,
                    "unidades".to_string(),
                ),
                Item::new(
                    None,
                    "101108 - IPAC CH3 - ENVASE YOGUR GRIEGO  360 G".to_string(),
                    100,
                    "unidades".to_string(),
                ),
            ],
        ),
    ];
    Mutex::new(initial_orders)
}
