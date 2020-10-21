## Production build

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.4
```

## Run in singlepass

In order to measure gas consumption, singlepass tests need to be used. E.g.

```sh
cargo wasm
cargo +nightly integration-test --no-default-features --features singlepass verify_valid -- --nocapture
```
