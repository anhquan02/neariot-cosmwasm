use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::state::{Offer, Project, User};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    /// cw20_addr is the address of the allowed cw20 token
    pub cw20_addr: String,
}
#[cw_serde]
pub enum ExecuteMsg {
    // User
    RegisterUser {},

    // Project
    CreateProject {
        metadata: String,
    },
    UpdateProject {
        id: String,
        metadata: String,
    },
    UpdateTimestamp {
        id: String,
        timestamp: u64,
    },
    DeleteProject {},
    CreateOffer {
        id: String,
        min_price: Uint128,
        metadata: String,
        expire_at: u64,
    },
    UpdateOffer {
        id: String,
        offer_id: String,
        min_price: Uint128,
        metadata: String,
        expire_at: u64,
    },
    DeleteOffer {
        id: String,
        offer_id: String,
    },
    BuyOffer {
        project_id: String,
        offer_id: String,
        metadata: String,
        rate: Uint128,
    },
    RateOffer {
        project_id: String,
        offer_id: String,
        rate: Uint128,
    },

    // Watching
    WatchProject {
        id: String,
    },
    UnwatchProject {
        id: String,
    },

    // Rating
    RateProject {
        id: String,
        rate: Uint128,
    },
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // User
    #[returns(User)]
    GetUser { id: Addr },

    #[returns(Vec<User>)]
    ListUser {},

    // Project
    #[returns(Project)]
    GetProject { id: String },

    #[returns(Vec<Project>)]
    ListProject {},

    // Project Offers
    #[returns(Vec<Offer>)]
    GetProjectOffers { project_id: String },

    #[returns(Offer)]
    GetProjectOffer {
        project_id: String,
        offer_id: String,
    },

    // Funding
    #[returns(String)]
    GetFunding {},

    #[returns(String)]
    ListFunding {},

    // Watching +
    #[returns(Project)]
    GetWatching {},

    // Rating
    #[returns(Project)]
    GetRating {},

    #[returns(Project)]
    ListRating {},
}
