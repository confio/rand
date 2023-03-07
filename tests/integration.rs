//! This integration test tries to run and call the generated wasm.
//! It depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.

use cosmwasm_std::{from_binary, Binary, ContractResult, Response};
use cosmwasm_vm::testing::{
    execute, instantiate, mock_env, mock_info, mock_instance, mock_instance_with_gas_limit, query,
};
use std::time::Instant;

use rand::msg::{ExecuteMsg, InstantiateMsg, LatestResponse, QueryMsg, ShuffleResponse};

static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/rand.wasm");
// static WASM: &[u8] = include_bytes!("../artifacts/rand.wasm");

fn pubkey_loe_mainnet() -> Binary {
    vec![
        134, 143, 0, 94, 184, 230, 228, 202, 10, 71, 200, 167, 124, 234, 165, 48, 154, 71, 151,
        138, 124, 113, 188, 92, 206, 150, 54, 107, 93, 122, 86, 153, 55, 197, 41, 238, 218, 102,
        199, 41, 55, 132, 169, 64, 40, 1, 175, 49,
    ]
    .into()
}

const BOUNTY_DENOM: &str = "ucosm";

#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = InstantiateMsg {
        pubkey: pubkey_loe_mainnet(),
        bounty_denom: BOUNTY_DENOM.into(),
    };
    let info = mock_info("creator", &[]);
    // we can just call .unwrap() to assert this was a success
    let res: Response = instantiate(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(res.messages.len(), 0);
}

#[test]
fn verify_valid() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000_000_000);

    let msg = InstantiateMsg {
        pubkey: pubkey_loe_mainnet(),
        bounty_denom: BOUNTY_DENOM.into(),
    };
    let info = mock_info("creator", &[]);
    let _res: Response = instantiate(&mut deps, mock_env(), info.clone(), msg).unwrap();

    let time_before = Instant::now();
    let gas_before = deps.get_gas_left();

    let msg = ExecuteMsg::Add {
        round: 72785,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };

    let _res: Response = execute(&mut deps, mock_env(), info, msg).unwrap();
    let gas_used = gas_before - deps.get_gas_left();
    println!("Gas used: {}", gas_used);
    println!("Time elapsed: {:.2?}", time_before.elapsed());

    let latest: LatestResponse =
        from_binary(&query(&mut deps, mock_env(), QueryMsg::Latest {}).unwrap()).unwrap();
    assert_eq!(latest.round, 72785);

    assert_eq!(
        latest.randomness,
        hex::decode("8b676484b5fb1f37f9ec5c413d7d29883504e5b669f604a1ce68b3388e9ae3d9").unwrap()
    );
}

#[test]
fn verify_invalid() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000_000_000);

    let msg = InstantiateMsg {
        pubkey: pubkey_loe_mainnet(),
        bounty_denom: BOUNTY_DENOM.into(),
    };
    let info = mock_info("creator", &[]);
    let _res: Response = instantiate(&mut deps, mock_env(), info, msg).unwrap();

    let gas_before = deps.get_gas_left();
    let info = mock_info("anyone", &[]);
    let msg = ExecuteMsg::Add {
        // curl -sS https://drand.cloudflare.com/public/72785
        round: 42,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };

    let res: ContractResult<Response> = execute(&mut deps, mock_env(), info, msg);
    let err = res.unwrap_err();

    assert_eq!(err, "Signature verification failed");
    let gas_used = gas_before - deps.get_gas_left();
    println!("Gas used: {}", gas_used);
}

#[test]
fn query_shuffle() {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000_000_000);

    let msg = InstantiateMsg {
        pubkey: pubkey_loe_mainnet(),
        bounty_denom: BOUNTY_DENOM.into(),
    };
    let info = mock_info("creator", &[]);
    let _res: Response = instantiate(&mut deps, mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Add {
        round: 72785,
        previous_signature: hex::decode("a609e19a03c2fcc559e8dae14900aaefe517cb55c840f6e69bc8e4f66c8d18e8a609685d9917efbfb0c37f058c2de88f13d297c7e19e0ab24813079efe57a182554ff054c7638153f9b26a60e7111f71a0ff63d9571704905d3ca6df0b031747").unwrap().into(),
        signature: hex::decode("82f5d3d2de4db19d40a6980e8aa37842a0e55d1df06bd68bddc8d60002e8e959eb9cfa368b3c1b77d18f02a54fe047b80f0989315f83b12a74fd8679c4f12aae86eaf6ab5690b34f1fddd50ee3cc6f6cdf59e95526d5a5d82aaa84fa6f181e42").unwrap().into(),
    };
    let _res: Response = execute(&mut deps, mock_env(), info, msg).unwrap();

    let response: ShuffleResponse = from_binary(
        &query(
            &mut deps,
            mock_env(),
            QueryMsg::Shuffle {
                round: 72785,
                from: 1,
                to: 65,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(response.list.len(), 65);
    assert_eq!(
        response.list,
        [
            41, 53, 24, 43, 18, 47, 45, 11, 51, 39, 5, 32, 7, 33, 56, 30, 27, 55, 15, 21, 31, 9, 2,
            61, 17, 28, 48, 19, 46, 37, 4, 34, 29, 8, 10, 13, 65, 38, 64, 16, 26, 50, 35, 6, 49,
            59, 58, 63, 12, 42, 62, 23, 52, 60, 44, 20, 57, 40, 22, 3, 25, 14, 54, 1, 36
        ]
    );

    // Test various list sizes

    for count in [1, 5, 10, 25, 50, 100, 250, 500, 750, 900, 1000] {
        let time_before = Instant::now();
        let gas_before = deps.get_gas_left();
        let response: ShuffleResponse = from_binary(
            &query(
                &mut deps,
                mock_env(),
                QueryMsg::Shuffle {
                    round: 72785,
                    from: 1,
                    to: count,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(response.list.len(), count as usize);
        let gas_used = gas_before - deps.get_gas_left();
        let mega_gas_per_element = (gas_used as f32 / count as f32) / 1_000_000f32;
        println!(
            "{} | {} | {} | {:.2?}",
            count,
            gas_used,
            mega_gas_per_element,
            time_before.elapsed()
        );
    }
}
