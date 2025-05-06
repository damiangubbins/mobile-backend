use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use std::borrow::Cow;

pub type ItemList = Mutex<Vec<Item<'static>>>;
pub type Items<'r> = &'r State<ItemList>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item<'r> {
    pub id: Option<String>,
    pub name: Cow<'r, str>,
    pub quantity: u32,
    pub unit: Cow<'r, str>,
}

impl Item<'_> {
    pub fn new<'r>(
        id: Option<String>,
        name: Cow<'r, str>,
        quantity: u32,
        unit: Cow<'r, str>,
    ) -> Item<'r> {
        Item {
            id,
            name,
            quantity,
            unit,
        }
    }
}

pub type OrderList = Mutex<Vec<Order<'static>>>;
pub type Orders<'r> = &'r State<OrderList>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Order<'r> {
    pub id: Option<String>,
    pub items: Vec<Item<'r>>,
}

impl Order<'_> {
    pub fn new<'r>(id: Option<String>, items: Vec<Item<'r>>) -> Order<'r> {
        Order { id, items }
    }
}
