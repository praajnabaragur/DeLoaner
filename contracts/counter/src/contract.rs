// Helped by Ian from PFC-Validator for much of the functiions.
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, SubMsg, WasmMsg, Order
};

use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, GenericBalance, State, Wager, WAGERS, LoanToken, LOAN_TOKENS};

// version info for migration info
const CONTRACT_NAME: &str = "duel-dojo:wager";
const CONTRACT_VERSION: &str = "0.1";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State {
        creator: info.sender.clone(),
        owner: info.sender,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        //DUEL DOJO FUNCTIONS
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::CreateWagerNative { wager_id } => {
            execute_create_wager(deps, env, info.sender, Balance::from(info.funds), wager_id)
        }
        ExecuteMsg::AddFundsNative { wager_id } => {
            execute_add_funds(deps, env, info.sender, Balance::from(info.funds), wager_id)
        }
        ExecuteMsg::Cancel { wager_id } => execute_cancel(deps, env, info, wager_id),
        ExecuteMsg::SendFunds {
            wager_id,
            winner_address,
        } => execute_send_funds(deps, env, info, wager_id, winner_address),
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // Note: info.sender is the address of the token contract as the contract makes this call and
    //       cw20_msg.sender is the user who initiated the send of tokens call.
    //TODO: Add validation of allowed CW20s here by checking info.sender.
    let coin = Cw20CoinVerified {
        address: info.sender,
        amount: cw20_msg.amount,
    };
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::CreateWager { wager_id }) => {
            let api = deps.api;
            execute_create_wager(
                deps,
                env,
                api.addr_validate(&cw20_msg.sender)?,
                Balance::from(coin),
                wager_id,
            )
        }
        Ok(Cw20HookMsg::AddFunds { wager_id }) => {
            let api = deps.api;
            execute_add_funds(
                deps,
                env,
                api.addr_validate(&cw20_msg.sender)?,
                Balance::from(coin),
                wager_id,
            )
        }
        Err(_) => Err(ContractError::DataShouldBeGiven {}),
    }
}

pub fn execute_create_wager(
    deps: DepsMut,
    _env: Env,
    sender: Addr,
    balance: Balance,
    wager_id: String,
) -> Result<Response, ContractError> {
    let user1_balance = match balance {
        Balance::Native(balance) => GenericBalance {
            native: balance.0,
            cw20: vec![],
        },
        Balance::Cw20(token) => GenericBalance {
            native: vec![],
            cw20: vec![token],
        },
    };

    let state = config(deps.storage).load()?;

    let wager = Wager {
        arbiter: state.owner,
        user1: sender,
        user2: Addr::unchecked("empty"),
        user1_balance,
        user2_balance: GenericBalance::new(),
    };

    WAGERS.update(deps.storage, &wager_id, |existing| match existing {
        None => Ok(wager),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("id", wager_id.as_str())]);
    Ok(res)
}

pub fn execute_add_funds(
    deps: DepsMut,
    _env: Env,
    sender: Addr,
    balance: Balance,
    wager_id: String,
) -> Result<Response, ContractError> {
    let mut wager = get_wager(&deps, &wager_id)?;

    if wager.user2 != "empty" || wager.user1 == sender {
        return Err(ContractError::AlreadyInUse {});
    }

    wager.user2_balance.add_tokens(balance);
    wager.user2 = sender;

    if wager.user2_balance != wager.user1_balance {
        return Err(ContractError::UnequalBalance {});
    }

    WAGERS.update(deps.storage, &wager_id, |existing| match existing {
        None => Err(ContractError::WagerDoesNotExist {}),
        Some(_) => Ok(wager),
    })?;

    let res =
        Response::new().add_attributes(vec![("action", "add_funds"), ("id", wager_id.as_str())]);

    Ok(res)
}

pub fn execute_cancel(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wager_id: String,
) -> Result<Response, ContractError> {
    let wager = get_wager(&deps, &wager_id)?;

    if info.sender != wager.user1 && info.sender != wager.arbiter || wager.user2 != "empty" {
        Err(ContractError::Unauthorized {})
    } else {
        WAGERS.remove(deps.storage, &wager_id);

        let messages: Vec<SubMsg> = send_tokens(&wager.user1, &wager.user1_balance, 100)?;

        Ok(Response::new()
            .add_attribute("action", "cancel")
            .add_attribute("id", wager_id)
            .add_attribute("to", wager.user1)
            .add_submessages(messages))
    }
}

pub fn execute_send_funds(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wager_id: String,
    winner_address: Addr,
) -> Result<Response, ContractError> {
    let wager = get_wager(&deps, &wager_id)?;
    let state = config(deps.storage).load()?;

    if info.sender != state.owner {
        Err(ContractError::Unauthorized {})
    } else if winner_address != wager.user1 && winner_address != wager.user2 {
        Err(ContractError::UserDoesNotExist {})
    } else {
        // we delete the wager
        WAGERS.remove(deps.storage, &wager_id);

        // send user1 tokens to winner
        let user1_messages: Vec<SubMsg> = send_tokens(&winner_address, &wager.user1_balance, 100)?;
        // send user2 tokens to winner
        let user2_messages: Vec<SubMsg> = send_tokens(&winner_address, &wager.user2_balance, 100)?;

        Ok(Response::new()
            .add_attribute("action", "send_tokens_to_winner")
            .add_attribute("id", wager_id)
            .add_attribute("to", winner_address)
            .add_submessages(user1_messages)
            .add_submessages(user2_messages))
    }
}

fn get_wager(deps: &DepsMut, wager_id: &str) -> Result<Wager, ContractError> {
    match WAGERS.load(deps.storage, wager_id) {
        Ok(wager) => Ok(wager),
        Err(_) => Err(ContractError::WagerDoesNotExist {}),
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Wager { id } => to_binary(&query_wager_for_id(id, deps)?),
        //QueryMsg::WagerList {} => to_binary(&query_wager_list(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<State> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

fn query_wager_for_id(id: String, deps: Deps) -> StdResult<Wager> {
    let wager = WAGERS.load(deps.storage, &id)?;
    Ok(wager)
}
// fn query_wager_list(deps: Deps) -> StdResult<Vec<Wager>> {
//     //let wagers = WAGERS.load_all(deps.storage)?;
//     //Ok(wagers)
// }
// funds: anchor_pools()
//             .range(deps.storage, start, None, Order::Ascending)
//             .filter(|f| match f {
//                 Ok((_, ap)) => ap.open,
//                 Err(_) => false,
//             })
//             .take(limit)
//             .map(|item| item.map(|(_, v)| v))
//             .collect::<StdResult<Vec<AnchorPool>>>()?,
pub fn execute_send_loans(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    borrower: Addr,
    loan_amount: GenericBalance,
    date: i32,
) -> Result<Response, ContractError> {
    let state = config(deps.storage).load()?;

    if info.sender != state.owner {
        Err(ContractError::Unauthorized {})
    } else {
        let loan_messages: Vec<SubMsg> = send_tokens(&borrower, &loan_amount, 100)?;

        let loan_amount_paid = GenericBalance {
            native: vec![],
            cw20: vec![],
        };

        let loan_token = LoanToken {
            lender: info.sender,
            borrower: borrower,
            loan_amount: loan_amount,
            loan_amount_paid: loan_amount_paid,
            date: date, // date as numeric value
        };

        LOAN_TOKENS.push(loan_token);

        Ok(Response::new()
            .add_attribute("action", "send_tokens_to_borrower"))
    }
}

pub fn execute_send_payment(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    receiver: Addr,
    payment_amount: GenericBalance,
) -> Result<Response, ContractError> {
    let commission = 7;
    let state = config(deps.storage).load()?;

    if info.sender != state.owner {
        Err(ContractError::Unauthorized {})
    } else {
        let loan_messages1: Vec<SubMsg> = send_tokens(&receiver, &payment_amount, 100 - commission)?;

        let mut idx = 0;
        for loan_token in LOAN_TOKENS.iter() {
            if loan_token.borrower == receiver {
                break;
            }
            idx += 1;
        } 

        let loan_messages2: Vec<SubMsg> = send_tokens(&LOAN_TOKENS[idx].lender, &payment_amount, commission)?;

        Ok(Response::new()
            .add_attribute("action", "send_tokens_to_receiver_and_lender"))
    }
}

fn send_tokens(to: &Addr, balance: &GenericBalance, percentage: i32) -> StdResult<Vec<SubMsg>> {
    let native_balance = &balance.native;
    let mut msgs: Vec<SubMsg> = if native_balance.is_empty() {
        vec![]
    } else {
        vec![SubMsg::new(BankMsg::Send {
            to_address: to.into(),
            amount: native_balance.to_vec(),
        })]
    };

    let cw20_balance = &balance.cw20;
    let cw20_msgs: StdResult<Vec<_>> = cw20_balance
        .iter()
        .map(|c| {
            let amount: u128 = c.amount.u128();
            
            let msg = Cw20ExecuteMsg::Transfer {
                recipient: to.into(),
                amount: c.amount,
            };
            let exec = SubMsg::new(WasmMsg::Execute {
                contract_addr: c.address.to_string(),
                msg: to_binary(&msg)?,
                funds: vec![],
            });
            Ok(exec)
        })
        .collect();
    msgs.append(&mut cw20_msgs?);
    Ok(msgs)
}