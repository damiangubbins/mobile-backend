use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;

pub type ItemList = Mutex<Vec<Item>>;
pub type Items = State<ItemList>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Option<String>,
    pub name: String,
    pub quantity: u32,
    pub unit: String,
}

impl Item {
    pub fn new(id: Option<String>, name: String, quantity: u32, unit: String) -> Item {
        Item {
            id,
            name,
            quantity,
            unit,
        }
    }
}

pub type OrderList = Mutex<Vec<Order>>;
pub type Orders = State<OrderList>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Option<String>,
    pub items: Vec<Item>,
}

impl Order {
    pub fn new(id: Option<String>, items: Vec<Item>) -> Order {
        Order { id, items }
    }
}

pub type PaletList = Mutex<Vec<Palet>>;
pub type Palets = State<PaletList>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Palet {
    pub id: Option<String>,
    pub item_ids: Vec<String>,
}

impl Palet {
    pub fn new(id: Option<String>, item_ids: Vec<String>) -> Palet {
        Palet { id, item_ids }
    }
}
