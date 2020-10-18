use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, MessageInfo, Querier,
    StdResult, Storage,
};
use drand_verify::{g1_from_fixed, verify};

use crate::msg::{HandleMsg, InitMsg, LatestResponse, QueryMsg};
use crate::state::{config, config_read, State};

// $ node
// > Uint8Array.from(Buffer.from("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31", "hex"))
const PK_LEO_MAINNET: [u8; 48] = [
    134, 143, 0, 94, 184, 230, 228, 202, 10, 71, 200, 167, 124, 234, 165, 48, 154, 71, 151, 138,
    124, 113, 188, 92, 206, 150, 54, 107, 93, 122, 86, 153, 55, 197, 41, 238, 218, 102, 199, 41,
    55, 132, 169, 64, 40, 1, 175, 49,
];

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State { round: msg.round };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Verify {
            round,
            previous_signature,
            signature,
        } => try_verify(round, previous_signature, signature),
    }
}

pub fn try_verify(
    round: u64,
    previous_signature: Binary,
    signature: Binary,
) -> StdResult<HandleResponse> {
    let pk = g1_from_fixed(PK_LEO_MAINNET).unwrap();
    let valid = verify(
        &pk,
        round,
        previous_signature.as_slice(),
        signature.as_slice(),
    )
    .unwrap();

    let mut response = HandleResponse::default();
    let mut data: Vec<u8> = Vec::new();
    data.push(if valid { 0x01 } else { 0x00 });
    response.data = Some(data.into());
    Ok(response)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Latest {} => to_binary(&query_latest(deps)?),
    }
}

fn query_latest<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<LatestResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(LatestResponse { round: state.round })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = InitMsg { round: 17 };

        let res = init(&mut deps, mock_env(), info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let res = query(&deps, mock_env(), QueryMsg::Latest {}).unwrap();
        let value: LatestResponse = from_binary(&res).unwrap();
        assert_eq!(value.round, 17);
    }

    #[test]
    fn verify() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let info = mock_info("creator", &coins(2, "token"));
        let msg = InitMsg { round: 17 };
        let _res = init(&mut deps, mock_env(), info, msg).unwrap();

        let info = mock_info("anyone", &coins(2, "token"));
        let msg = HandleMsg::Verify {
            // curl -sS https://drand.cloudflare.com/public/72785
            round: 72785,
            previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
            signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
        };
        let res = handle(&mut deps, mock_env(), info, msg).unwrap();
        assert_eq!(res.data.unwrap().as_slice(), [0x01]);
    }
}
