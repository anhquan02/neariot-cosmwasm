#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, to_binary, Addr, Timestamp};
use cw_utils::NativeBalance;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::*;
use crate::utils::generate_id;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:neariot-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.save(_deps.storage, &_info.sender)?;
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::RegisterUser{}=>execute_register_user(_deps,_env,_info),
        ExecuteMsg::CreateProject { metadata } => execute_create_project(_deps, _env, _info, metadata),
        ExecuteMsg::UpdateProject { id, metadata } => execute_update_project(_deps, _env, _info, id, metadata),
        ExecuteMsg::UpdateTimestamp { id, timestamp } => execute_update_timestamp(_deps, _env, _info, id, timestamp),
        ExecuteMsg::DeleteProject {  } => todo!(),
        ExecuteMsg::CreateOffer { id, min_price, metadata,expire_at } => execute_create_project_offer(_deps, _env, _info, id, min_price, metadata,expire_at),
        ExecuteMsg::UpdateOffer { id, offer_id, min_price, metadata,expire_at } => execute_update_project_offer(_deps, _env, _info, id, offer_id, min_price, metadata,expire_at),
        ExecuteMsg::DeleteOffer { id, offer_id } => execute_delete_project_offer(_deps, _env, _info, id, offer_id),
        ExecuteMsg::BuyOffer { project_id, offer_id, metadata, rate } => execute_buy_project_offer(_deps, _env, _info, project_id, offer_id, metadata, rate),
        ExecuteMsg::RateOffer { project_id, offer_id, rate } => execute_rate_project_offer(_deps, _env, _info, project_id, offer_id, rate),
        ExecuteMsg::WatchProject { id } => execute_watch_project(_deps, _env, _info, id),
        ExecuteMsg::UnwatchProject { id } => execute_unwatch_project(_deps, _env, _info, id),
        ExecuteMsg::RateProject { id, rate } => execute_rate_project(_deps, _env, _info, id, rate),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::GetUser{id}=>to_binary(&query_get_user(_deps,id)?),
        QueryMsg::ListUser {  } => to_binary(&query_list_user(_deps)?),
        QueryMsg::GetProject { id } => to_binary(&query_get_project(_deps, id)?),
        QueryMsg::ListProject {  } => to_binary(&query_list_project(_deps)?),
        QueryMsg::GetProjectOffers { project_id} => to_binary(&query_get_project_offers(_deps, project_id)?),
        QueryMsg::GetProjectOffer { project_id, offer_id  } => to_binary(&query_get_project_offer(_deps, project_id, offer_id)?),
        QueryMsg::GetFunding {  } => todo!(),
        QueryMsg::ListFunding {  } => todo!(),
        QueryMsg::GetWatching {  } => todo!(),
        QueryMsg::GetRating {  } => todo!(),
        QueryMsg::ListRating {  } => todo!(),
        QueryMsg::GetBalance {  } => todo!(),
        QueryMsg::GetAdmin {  } => to_binary(&query_get_admin(_deps)?),
    }
}

pub fn execute_register_user(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let user = User {
        address: _info.sender.clone(),
        name: _info.sender.to_string(),
        total_spent: Uint128::zero(),
        project_funded: vec![],
        project_watched: vec![],
        project_owned: vec![],
    };
    let mut users = USERS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    if users.is_empty(){
        users = vec![];
    }
    users.push(user);
    USERS.save(_deps.storage, &users)?;
    Ok(Response::default())

}

pub fn execute_create_project(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _metadata: String,
) -> Result<Response, ContractError> {
    let block_info = _env.block.clone();
    let project = Project {
        owner: _info.sender.clone(),
        id: generate_id(_info.sender.clone(),_env.block),
        metadata: _metadata,
        avg_rate: Uint128::zero(),
        create_at: block_info.time,
        watchers: vec![],
        offers: vec![],
        total_pledged: Uint128::zero(),
        bougth_offers: vec![],
        milestone: block_info.time,
    };
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    if projects.is_empty(){
        projects = vec![];
    }
    let mut users = USERS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    if users.is_empty(){
        users = vec![];
    }
    users.iter_mut().for_each(|user|{
        if user.address.eq(&_info.sender.clone()){
            user.project_owned.push(project.id.clone());
        }
    });
    USERS.save(_deps.storage, &users)?;
    projects.push(project.clone());
    PROJECTS.save(_deps.storage, &projects)?;
    let res = Response::new()
        .add_attribute("action", "create_project")
        .add_attribute("project_id", project.clone().id);
    Ok(res)
}

pub fn execute_update_project(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _metadata: String,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");

    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            assert!(project.owner == _info.sender, "not owner");
            project.metadata = _metadata.to_owned();
        }
    });

    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}


pub fn execute_update_timestamp(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _timestamp: u64,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            assert!(project.owner == _info.sender, "not owner");
            project.milestone = Timestamp::from_seconds(_timestamp.to_owned());
        }
    });

    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_create_project_offer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _price: Uint128,
    _metadata: String,
    _expire: u64,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            assert!(project.owner == _info.sender.clone(), "not owner");
            let offer = Offer {
                id: generate_id(_info.sender.clone(),_env.block.clone()),
                metadata: _metadata.clone(),
                min_price: _price,
                create_at: _env.block.time,
                expire_at: Timestamp::from_seconds(_expire),
            };
            project.offers.push(offer);
        }
    });

    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_update_project_offer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _offer_id: String,
    _price: Uint128,
    _metadata: String,
    _expire: u64,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            assert!(project.owner == _info.sender, "not owner");
            project.offers.iter_mut().for_each(|offer|{
                if offer.id == _offer_id{
                    offer.min_price = _price;
                    offer.metadata = _metadata.clone();
                    offer.expire_at = Timestamp::from_seconds(_expire);
                }
            });
        }
    });

    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_delete_project_offer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _offer_id: String,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            assert!(project.owner == _info.sender, "not owner");
            project.offers.retain(|offer| offer.id != _offer_id);
        }
    });

    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_buy_project_offer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _offer_id: String,
    _metadata: String,
    _rate: Uint128,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    let amount = _info.funds[0].amount;
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            let offer = project.offers.iter().find(|offer| offer.id == _offer_id).unwrap();
            assert!(amount >= offer.min_price, "{}", ContractError::InsufficientFunds {});
            let bougth_offer = BougthOffer {
                id: generate_id(_info.sender.clone(),_env.block.clone()),
                price: amount,
                buyer: _info.sender.clone(),
                create_at: _env.block.time,
                rate:_rate,
                metadata: _metadata.clone(),
            };
            project.bougth_offers.push(bougth_offer);
        }
    });
    // let user = USERS.load(_deps.storage)?;
    // let mut user = user;
    // user.total_spent = user.total_spent + amount;
    // user.project_funded.iter().for_each(|project|{
    //     if project != &_id{
    //         user.project_funded.push(_id);
    //     }
    // });
    // user.project_watched.iter().for_each(|project|{
    //     if project != &_id{
    //         user.project_watched.push(_id);
    //     }
    // });
    // USERS.save(_deps.storage,  &user)?;
    let mut users = USERS.load(_deps.storage).unwrap_or_default();
    users.iter_mut().for_each(|user|{
        if user.address == _info.sender{
            user.total_spent = user.total_spent + amount;
            user.clone().project_funded.iter_mut().for_each(|project|{
                if project != &_id{
                    user.project_funded.push(_id.clone());
                }
            });
            user.clone().project_watched.iter().for_each(|project|{
                if project != &_id{
                    user.project_watched.push(_id.clone());
                }
            });
        }
    });
    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn must_pay_funds(balance:&NativeBalance,denom:&str) -> Result<Uint128,ContractError>{
    match balance.0.len() {
        0 => Err(ContractError::NoFunds {}),
        1 => {
            let balance = &balance.0;
            let payment = balance[0].amount;
            if balance[0].denom == denom {
                Ok(payment)
            } else {
                Err(ContractError::MissingDenom(denom.to_string()))
            }
        }
        _ => Err(ContractError::ExtraDenoms(denom.to_string())),
    }
}

pub fn execute_rate_project_offer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _offer_id: String,
    _rate: Uint128,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            project.bougth_offers.iter_mut().for_each(|offer|{
                if offer.id == _offer_id{
                    offer.rate = _rate;
                }
            });
        }
    });
    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_watch_project(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            project.watchers.iter().for_each(|watcher|{
                if watcher.eq(&_info.sender.clone()){
                    project.clone().watchers.push(_info.sender.clone());
                }
            });
        }
    });
    // let mut user = USERS.load(_deps.storage)?;
    // user.project_watched.iter().for_each(|project|{
    //     if project != &_id{
    //         user.project_watched.push(_id);
    //     }
    // });
    let mut users = USERS.load(_deps.storage).unwrap_or_default();
    users.iter_mut().for_each(|user|{
        if user.address == _info.sender{
            user.clone().project_watched.iter().for_each(|project|{
                if !project.clone().eq(&_id.clone()){
                    user.project_watched.push(_id.clone());
                }
            });
        }
    });
    USERS.save(_deps.storage,  &users)?;
    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_unwatch_project(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            project.watchers.retain(|watcher| watcher != &_info.sender);
        }
    });
    // let mut user = USERS.load(_deps.storage)?;
    // user.project_watched.retain(|project| project != &_id);
    let mut users = USERS.load(_deps.storage).unwrap_or_default();
    users.iter_mut().for_each(|user|{
        if user.address == _info.sender{
            user.project_watched.retain(|project| project != &_id);
        }
    });
    USERS.save(_deps.storage,  &users)?;
    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn execute_rate_project(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _id: String,
    _rate: Uint128,
) -> Result<Response, ContractError> {
    let mut projects = PROJECTS.load(_deps.storage).unwrap_or_default();
    //if load fail, create new
    assert!(projects.is_empty(), "project not found");
    assert!(projects.iter().any(|project| project.id == _id), "project not found");
    projects.iter_mut().for_each(|project|{
        if project.id == _id{
            project.avg_rate = project.bougth_offers.iter().fold(Uint128::zero(),|acc,offer|{
                acc + offer.rate
            }) / Uint128::from(project.bougth_offers.len() as u128);
        }
    });
    PROJECTS.save(_deps.storage, &projects)?;
    Ok(Response::default())
}

pub fn query_get_user(_deps: Deps, _id: Addr) -> StdResult<User> {
    let users = USERS.load(_deps.storage)?;
    let user = users.iter().find(|user| user.name == _id).unwrap();
    Ok(user.to_owned())
}

pub fn query_list_user(_deps: Deps) -> StdResult<Vec<User>> {
    let users = USERS.load(_deps.storage)?;
    Ok(users.to_owned())
}

pub fn query_get_project(_deps: Deps, _id: String) -> StdResult<Project> {
    let projects = PROJECTS.load(_deps.storage)?;
    let project = projects.iter().find(|project| project.id == _id).unwrap();
    Ok(project.to_owned())
}

pub fn query_list_project(_deps: Deps) -> StdResult<Vec<Project>> {
    let projects = PROJECTS.load(_deps.storage)?;
    Ok(projects.to_owned())
}

pub fn query_get_project_offers(_deps: Deps, _id: String) -> StdResult<Vec<Offer>> {
    let projects = PROJECTS.load(_deps.storage)?;
    let project = projects.iter().find(|project| project.id == _id).unwrap();
    Ok(project.offers.to_owned())
}

pub fn query_get_project_offer(_deps: Deps, _id: String, _offer_id: String) -> StdResult<Offer> {
    let projects = PROJECTS.load(_deps.storage)?;
    let project = projects.iter().find(|project| project.id == _id).unwrap();
    let offer = project.offers.iter().find(|offer| offer.id == _offer_id).unwrap();
    Ok(offer.to_owned())
}

pub fn query_get_admin(_deps: Deps) -> StdResult<Addr> {
    let admin = ADMIN.load(_deps.storage)?;
    Ok(admin.to_owned())
}


// #[inline]
// fn coin_to_string(amount: Uint128, denom: &str) -> String {
//     format!("{} {}", amount, denom)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, CosmosMsg,  Coin, BankMsg};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "orai"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn register_user() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "orai"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::RegisterUser {};
        let info = mock_info("ciuz", &coins(1000, "orai"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = QueryMsg::GetUser { id: Addr::unchecked("ciuz") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: User = from_binary(&res).unwrap();
        assert_eq!(user.total_spent, Uint128::zero());
        assert_eq!(user.project_funded.len(), 0);
        assert_eq!(user.project_watched.len(), 0);
        assert_eq!(user.project_owned.len(), 0);
    }

    #[test]
    fn create_project(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "orai"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::RegisterUser {};
        let info = mock_info("ciuz", &coins(1000, "orai"));
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::CreateProject {
            metadata: "example".to_string(),
        };
        let info = mock_info("ciuz", &coins(1000, "orai"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let project_id = res.attributes[1].clone().value;

        let msg = QueryMsg::GetProject { id: project_id };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let project: Project = from_binary(&res).unwrap();
        assert_eq!(project.metadata, "example".to_string());
    }

    // #[test]
    // fn create_offer(){
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg {};
    //     let info = mock_info("creator", &coins(1000, "orai"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::RegisterUser {};
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::CreateProject {
    //         metadata: "example".to_string(),
    //     };
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     let project_id = _res.attributes[1].clone().value;

    //     let msg = ExecuteMsg::CreateOffer {
    //         id: "test".to_string(),
    //         metadata: "example".to_string(),
    //         min_price: Uint128::from(1000u128),
    //         expire_at: 0,
    //     };
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     assert_eq!(0, res.messages.len());

    //     let msg = QueryMsg::GetProject { id: project_id};
    //     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    //     let project: Project = from_binary(&res).unwrap();
    //     assert_eq!(project.offers.len(), 1);
    //     assert_eq!(project.offers[0].id, "test-0".to_string());
    //     assert_eq!(project.offers[0].metadata, "example".to_string());
    //     assert_eq!(project.offers[0].min_price, Uint128::from(1000u128));

    // }

    // #[test]
    // fn buy_offer(){
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg {};
    //     let info = mock_info("creator", &coins(1000, "orai"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::RegisterUser {};
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::CreateProject {
    //         metadata: "example".to_string(),
    //     };
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::CreateOffer {
    //         id: "test".to_string(),
    //         metadata: "example".to_string(),
    //         min_price: Uint128::from(1000u128),
    //         expire_at: 0,
    //     };
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     let msg = ExecuteMsg::BuyOffer {
    //         project_id: "test".to_string(),
    //         offer_id: "test-0".to_string(),
    //         metadata: "example".to_string(),
    //         rate: Uint128::from(4u128),
    //     };
    //     let info = mock_info("ciuz", &coins(1000, "orai"));
    //     let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     assert_eq!(1, res.messages.len());
    //     assert_eq!(res.messages[0].msg, CosmosMsg::Bank(BankMsg::Send {
    //         to_address: "ciuz".to_string(),
    //         amount: vec![Coin {
    //             denom: "orai".to_string(),
    //             amount: Uint128::from(1000u128),
    //         }],
    //     }));

    // }

}
