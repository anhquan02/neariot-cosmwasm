use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Offer {
    pub id: String,
    pub min_price: Uint128,
    pub metadata: String,
    pub create_at: Timestamp,
    pub expire_at: Timestamp,
}
#[cw_serde]
pub struct BougthOffer {
    pub id: String,
    pub price: Uint128,
    pub create_at: Timestamp,
    pub metadata: String,
    pub rate: Uint128,
    pub buyer: Addr,
}
#[cw_serde]
pub struct User {
    pub address: Addr,
    pub name: String,
    pub total_spent: Uint128,
    pub project_funded: Vec<String>,
    pub project_watched: Vec<String>,
    pub project_owned: Vec<String>,
}

#[cw_serde]
pub struct Project {
    pub owner: Addr,
    pub id: String,
    pub metadata: String,
    pub avg_rate: Uint128,
    pub create_at: Timestamp,
    pub total_pledged: Uint128,
    pub watchers: Vec<Addr>,
    pub offers: Vec<Offer>,
    pub bougth_offers: Vec<BougthOffer>,
    pub milestone: Timestamp,
}

pub const USERS: Item<Vec<User>> = Item::new("users");
pub const PROJECTS: Item<Vec<Project>> = Item::new("projects");
