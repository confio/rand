use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{
    prefixed, singleton, singleton_read, PrefixedStorage, ReadonlySingleton, Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";
const BEACONS_KEY: &[u8] = b"beacons";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub round: u64,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn beacons_storage<S: Storage>(storage: &mut S) -> PrefixedStorage<S> {
    prefixed(storage, BEACONS_KEY)
}
