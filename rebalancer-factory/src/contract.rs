use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, ContractInfo, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, Uint256,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::{
    Config, ContractStatus, Portfolio, PortfolioConfig, CONFIG, PORTFOLIO_LIST, REGISTERED_ASSETS,
    UNUPDATED_LIST, VIEWING_KEY,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = Config {
        admin: info.sender,
        swap_factory: msg.swap_factory,
        withdraw_fee: msg.withdraw_fee,
        create_fee: msg.create_fee,
        snip20_code_id: msg.snip20_code_id,
        portfolio_code_id: msg.portfolio_code_id,
        accepted_deposit_tokens: msg.accepted_deposit_tokens.unwrap_or(vec![]),
        contract_status: ContractStatus::ACTIVE,
    };

    CONFIG.save(deps.storage, &state)?;
    PORTFOLIO_LIST.save(deps.storage, &vec![])?;
    UNUPDATED_LIST.save(deps.storage, &vec![])?;
    REGISTERED_ASSETS.save(deps.storage, &vec![])?;
    VIEWING_KEY.save(deps.storage, &msg.viewing_key)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateConfig {
            admin,
            swap_factory,
            withdraw_fee,
            create_fee,
            snip20_code_id,
            accepted_deposit_tokens,
            contract_status,
        } => try_update_config(
            deps,
            env,
            info,
            admin,
            swap_factory,
            withdraw_fee,
            create_fee,
            snip20_code_id,
            accepted_deposit_tokens,
            contract_status,
        ),
        ExecuteMsg::RegisterAssets { assets } => try_register_assets(deps, env, info, assets),
        ExecuteMsg::Update { batch_amount } => try_update(deps, env, info, batch_amount),
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            msg,
            ..
        } => try_receive(deps, env, info, sender, from, amount, msg),
    }
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<Addr>,
    swap_factory: Option<ContractInfo>,
    withdraw_fee: Option<i32>,
    create_fee: Option<i32>,
    snip20_code_id: Option<i32>,
    accepted_deposit_tokens: Option<Vec<ContractInfo>>,
    contract_status: Option<ContractStatus>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(StdError::generic_err("Must be admin"));
    }
    Ok(Response::default())
}

pub fn try_register_assets(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    assets: Vec<ContractInfo>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(StdError::generic_err("Must be admin"));
    }
    Ok(Response::default())
}

pub fn try_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    batch_amount: Option<Uint128>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(StdError::generic_err("Must be admin"));
    }
    Ok(Response::default())
}

pub fn try_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    from: Addr,
    amount: Uint256,
    msg: Option<Binary>,
) -> StdResult<Response> {
    if let Some(x) = msg {
        match from_binary(&x)? {
            ReceiveMsg::CreatePortfolio { config, name } => {
                try_create_portfolio(deps, env, info, config, name)
            }
            ReceiveMsg::Deposit { portfolio_snip20 } => {
                try_deposit(deps, env, info, sender, from, portfolio_snip20)
            }
            ReceiveMsg::Withdraw {} => try_withdraw(deps, env, info, sender, from),
            _ => Err(StdError::generic_err("Snip20 msg not recognized")),
        }
    } else {
        Ok(Response::default())
    }
}

pub fn try_create_portfolio(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: Vec<PortfolioConfig>,
    name: String,
) -> StdResult<Response> {
    // check if is valid config
    // instantiate portfolio
    // instantiate snip20
    Ok(Response::default())
}

pub fn try_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    from: Addr,
    portfolio_snip20: Addr,
) -> StdResult<Response> {
    // Check if is valid deposit asset
    // Check if is valid Portfolio snip20
    Ok(Response::default())
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    from: Addr,
) -> StdResult<Response> {
    // check if is valid portfolio snip20 (sender)
    // withdraw from portfolio contract
    // burn snip20
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&get_config(deps)?),
        QueryMsg::GetState {} => to_binary(&get_config(deps)?),
        QueryMsg::GetUnupdated {} => to_binary(&get_config(deps)?),
    }
}

fn get_config(deps: Deps) -> StdResult<Config> {
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}
