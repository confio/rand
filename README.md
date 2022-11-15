⚠️ This project was created as a proof of concept for an article in 2020. It served its<br>
⚠️ purpose. It was occasionally updated but does not belong to any actively maintained software<br>
⚠️ stack. It's archived now to highlight that no changes are done here anymore and no contributions<br>
⚠️ are accepted. It is likely insecure. Also please note that this code is licensed under AGPL which<br>
⚠️ might be a problem for when you want to use it. The story of drand verificartion in CosmWasm<br>
⚠️ is continued at [Nois](https://nois.network).

# Rand – A drand client as a CosmWasm smart contract

To learn more about this project, see this article: https://medium.com/confio/when-your-blockchain-needs-to-roll-the-dice-ed9da121f590

## Development build

Some fast checks

```sh
cargo fmt && cargo unit-test && cargo check --tests && cargo schema && cargo clippy -- -D warnings
```

Integratin tests

```sh
cargo wasm && cargo integration-test
```

### Run in singlepass

In order to measure gas consumption, singlepass tests need to be used. E.g.

```sh
cargo wasm
cargo integration-test --no-default-features verify_valid -- --nocapture
```

## Production build

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5
```

## License

```
A drand client in a smart contract for CosmWasm.
Copyright (C) 2020 Confio OÜ

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
