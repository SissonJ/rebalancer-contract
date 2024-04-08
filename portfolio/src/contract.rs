use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, ContractInfo, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, Uint256,
};
use rebalancer_factory::state::RouteKey;
use secret_toolkit::snip20;

use crate::msg::{
    BalanceItem, ExecuteAnswer, ExecuteMsg, Fee, InstantiateMsg, PositionCorrection,
    PositionDetails, QueryAnswer, QueryMsg, RouterMsg, SwapTokensForExact, UpdateAction,
    WithdrawAction,
};
use crate::state::{Config, Portfolio, PortfolioConfig, CONFIG, FEES, VIEWING_KEY};
use rebalancer_factory::msg::{query_prices, query_route};

pub const BLOCK_SIZE: usize = 256;
pub const NORMALIZATION_FACTOR: u32 = 18;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let mut state = Config {
        factory: msg.factory,
        accepted_deposit_tokens: msg.accepted_deposit_tokens.clone(),
        portfolio: msg.portfolio.clone(),
        admin: msg.admin,
    };

    let mut messages = vec![];

    for deposit_token in msg.accepted_deposit_tokens {
        messages.push(snip20::register_receive_msg(
            env.contract.code_hash.clone(),
            None,
            BLOCK_SIZE,
            deposit_token.code_hash.clone(),
            deposit_token.address.clone().into_string(),
        ));
        state.portfolio.config.push(PortfolioConfig {
            percent: 0,
            asset: deposit_token,
        })
    }

    for asset in msg.portfolio.config {
        messages.push(snip20::set_viewing_key_msg(
            msg.viewing_key.clone(),
            None,
            BLOCK_SIZE,
            asset.asset.code_hash,
            asset.asset.address.into_string(),
        ));
    }

    CONFIG.save(deps.storage, &state)?;
    VIEWING_KEY.save(deps.storage, &msg.viewing_key)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Update { tolerance_percent } => try_update(deps, env, info, tolerance_percent),
        ExecuteMsg::UpdateKey { viewing_key } => try_update_key(deps, info, viewing_key),
        ExecuteMsg::Withdraw {
            share,
            receiver,
            fee,
        } => try_withdraw(deps, env, info, share, receiver, fee),
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            msg,
            ..
        } => try_receive(deps, env, info, sender, from, amount, msg),
    }
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    share: Uint128,
    receiver: Addr,
    fee: u128,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.factory.address || info.sender != config.admin {
        return Err(StdError::generic_err("Must be factory contract"));
    }
    let viewing_key = VIEWING_KEY.load(deps.storage)?;

    let mut messages = vec![];
    let mut actions = vec![];

    for position in config.portfolio.config.clone() {
        let balance = snip20::balance_query(
            deps.querier,
            env.contract.address.clone().into_string(),
            viewing_key.clone(),
            BLOCK_SIZE,
            position.asset.code_hash.clone(),
            position.asset.address.clone().into_string(),
        )?;
        if balance.amount.is_zero() {
            continue;
        }
        let full_withdraw_amount = balance
            .amount
            .multiply_ratio(share, Uint128::new(10).pow(NORMALIZATION_FACTOR));
        if full_withdraw_amount.is_zero() {
            continue;
        }
        let fee_amount = full_withdraw_amount.multiply_ratio(Uint128::new(fee), Uint128::new(100)); //TODO get rid of constant
        let withdraw_amount = full_withdraw_amount.saturating_sub(fee_amount);
        messages.append(&mut vec![
            snip20::transfer_msg(
                receiver.clone().into_string(),
                withdraw_amount,
                None,
                None,
                BLOCK_SIZE,
                position.asset.code_hash.clone(),
                position.asset.address.clone().into_string(),
            )?,
            snip20::transfer_msg(
                config.admin.clone().into_string(),
                fee_amount,
                None,
                None,
                BLOCK_SIZE,
                position.asset.code_hash,
                position.asset.address.clone().into_string(),
            )?,
        ]);
        actions.push(WithdrawAction {
            snip20_addr: position.asset.address.clone(),
            amount: withdraw_amount,
        });
        let asset_fee = FEES
            .get(deps.storage, &position.asset.address)
            .unwrap_or(Uint128::zero());
        FEES.insert(
            deps.storage,
            &position.asset.address,
            &asset_fee.saturating_add(fee_amount),
        )?;
    }

    Ok(Response::new()
        .add_messages(messages)
        .set_data(to_binary(&ExecuteAnswer::Withdraw {
            withdraw_assets: actions,
        })?))
}

pub fn try_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    tolerance_percent: u128,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.factory.address || info.sender != config.admin {
        return Err(StdError::generic_err("Must be factory contract"));
    }
    let viewing_key = VIEWING_KEY.load(deps.storage)?;

    let mut portfolio_total_value = Uint128::zero();
    let mut percentage_tally = Uint128::zero();
    let mut messages = vec![];
    let mut actions = vec![];

    let mut imbalanced_positions = vec![];

    let mut temp_config = config.portfolio.config.clone();
    let price_query_assets = temp_config
        .into_iter()
        .map(|x| x.asset.address)
        .rev()
        .collect();
    let price_query_vec = query_prices(
        &deps,
        config.factory.clone(),
        price_query_assets,
        viewing_key.clone(),
    )?;

    for asset_position in config.portfolio.config {
        //percentage_tally = percentage_tally.saturating_add(Uint128::new(asset_position.percent));
        let price_query = price_query_vec
            .iter()
            .find(|&x| x.asset == asset_position.asset.address);
        if let Some(price) = price_query {
            let balance = snip20::balance_query(
                deps.querier,
                env.contract.address.clone().into_string(),
                viewing_key.clone(),
                BLOCK_SIZE,
                asset_position.asset.code_hash.clone(),
                asset_position.asset.address.clone().into_string(),
            )?;
            imbalanced_positions.push(PositionDetails {
                position: asset_position,
                value: Uint256::from_uint128(balance.amount)
                    .saturating_mul(Uint256::from_uint128(price.price)),
                price: price.price,
            })
        }
    }

    let mut over_target = vec![];
    let mut under_target = vec![];

    for imbalanced_position in imbalanced_positions {
        //TODO remove constants
        let target_asset_value =
            portfolio_total_value.multiply_ratio(imbalanced_position.position.percent, 100u128);
        let tolerance_amount = target_asset_value.multiply_ratio(tolerance_percent, 100u128);
        if imbalanced_position.value.gt(&Uint256::from_uint128(
            target_asset_value.saturating_add(tolerance_amount),
        )) {
            over_target.push(PositionCorrection {
                position: imbalanced_position.position.clone(),
                correction: imbalanced_position
                    .value
                    .saturating_sub(Uint256::from_uint128(target_asset_value)),
                price: imbalanced_position.price,
            });
        }
        if imbalanced_position.value.lt(&Uint256::from_uint128(
            target_asset_value.saturating_sub(tolerance_amount),
        )) {
            under_target.push(PositionCorrection {
                position: imbalanced_position.position,
                correction: Uint256::from_uint128(target_asset_value)
                    .saturating_sub(imbalanced_position.value),
                price: imbalanced_position.price,
            });
        }
    }

    for over_target_position in over_target {
        let mut over_target_correction = over_target_position.correction;
        if !over_target_correction.is_zero() {
            for under_target_position in &mut under_target {
                let (sell_amount, expected_return) = {
                    if over_target_correction.eq(&under_target_position.correction) {
                        let sell_amount = over_target_correction
                            .checked_div(Uint256::from_uint128(over_target_position.price))?;
                        let expected_return = under_target_position
                            .correction
                            .checked_div(Uint256::from_uint128(under_target_position.price))?;
                        under_target_position.correction = Uint256::zero();
                        over_target_correction = Uint256::zero();
                        (sell_amount, expected_return)
                    } else if over_target_correction.gt(&under_target_position.correction) {
                        let sell_amount = (over_target_correction
                            .saturating_sub(under_target_position.correction))
                        .checked_div(Uint256::from_uint128(over_target_position.price))?;
                        let expected_return = under_target_position
                            .correction
                            .checked_div(Uint256::from_uint128(under_target_position.price))?;
                        over_target_correction =
                            over_target_correction.saturating_sub(under_target_position.correction);
                        under_target_position.correction = Uint256::zero();
                        (sell_amount, expected_return)
                    } else if over_target_correction.lt(&under_target_position.correction) {
                        let sell_amount = over_target_correction
                            .checked_div(Uint256::from_uint128(over_target_position.price))?;
                        let expected_return = over_target_correction
                            .checked_div(Uint256::from_uint128(under_target_position.price))?;
                        over_target_correction = Uint256::zero();
                        under_target_position.correction = under_target_position
                            .correction
                            .saturating_sub(over_target_correction);
                        (Uint256::zero(), Uint256::zero())
                    } else {
                        (Uint256::zero(), Uint256::zero())
                    }
                };
                if !sell_amount.is_zero() {
                    let min_expected_return = expected_return.multiply_ratio(7u128, 10u128); // TODO
                    let route = query_route(
                        &deps,
                        config.factory.clone(),
                        RouteKey(
                            over_target_position.position.asset.address.clone(),
                            under_target_position.position.asset.address.clone(),
                        ),
                        viewing_key.clone(),
                    )?;
                    let msg = RouterMsg {
                        swap_tokens_for_exact: SwapTokensForExact {
                            expected_return: Uint128::try_from(min_expected_return)?,
                            path: route.route,
                        },
                    };
                    messages.push(snip20::send_msg_with_code_hash(
                        route.router_contract.address.into_string(),
                        Some(route.router_contract.code_hash),
                        Uint128::try_from(sell_amount)?,
                        Some(to_binary(&msg)?), // TODO
                        None,
                        None,
                        BLOCK_SIZE,
                        over_target_position.position.asset.code_hash.clone(),
                        over_target_position
                            .position
                            .asset
                            .address
                            .clone()
                            .into_string(),
                    )?);
                    actions.push(UpdateAction {
                        from_asset: over_target_position.position.asset.clone(),
                        to_asset: under_target_position.position.asset.clone(),
                        sell_amount,
                        expected_return,
                    });
                }
            }
        }
    }

    Ok(Response::new()
        .add_messages(messages)
        .set_data(to_binary(&ExecuteAnswer::Update { actions })?))
}

pub fn try_update_key(
    deps: DepsMut,
    info: MessageInfo,
    viewing_key: String,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.factory.address || info.sender != config.admin {
        return Err(StdError::generic_err("Must be factory contract"));
    }
    VIEWING_KEY.save(deps.storage, &viewing_key);
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
    let config = CONFIG.load(deps.storage)?;
    if config
        .accepted_deposit_tokens
        .iter()
        .find(|x| x.address == sender)
        == None
    {
        return Err(StdError::generic_err("Must be a valid deposit token"));
    }
    if info.sender != config.factory.address || info.sender != config.admin {
        return Err(StdError::generic_err("Must be factory contract"));
    }
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&get_config(deps)?),
        QueryMsg::GetFees {} => to_binary(&get_config(deps)?),
        QueryMsg::GetBalances {} => to_binary(&get_config(deps)?),
    }
}

fn get_config(deps: Deps) -> StdResult<QueryAnswer> {
    let state = CONFIG.load(deps.storage)?;
    Ok(QueryAnswer::GetConfig { config: state })
}

fn get_balances(deps: Deps, env: Env) -> StdResult<QueryAnswer> {
    let state = CONFIG.load(deps.storage)?;
    let viewing_key = VIEWING_KEY.load(deps.storage)?;
    let mut balances = vec![];
    for position in state.portfolio.config {
        let balance = snip20::balance_query(
            deps.querier,
            env.contract.address.clone().into_string(),
            viewing_key.clone(),
            BLOCK_SIZE,
            position.asset.code_hash,
            position.asset.address.clone().into_string(),
        )?;
        balances.push(BalanceItem {
            asset: position.asset.address,
            amount: balance.amount,
        })
    }
    Ok(QueryAnswer::GetBalances { balances })
}

fn get_fees(deps: Deps, env: Env) -> StdResult<QueryAnswer> {
    let state = CONFIG.load(deps.storage)?;
    let mut fees = vec![];
    for position in state.portfolio.config {
        fees.push(Fee {
            asset: position.asset.address.clone(),
            amount: FEES
                .get(deps.storage, &position.asset.address)
                .unwrap_or(Uint128::zero()),
        });
    }
    Ok(QueryAnswer::GetFees { fees })
}
