## Wasm Cache Rebuilder

To bump wasmer version from 2.0.0 to 2.1.1, we need to recreate serialized wasm cache due to [this issue](https://github.com/wasmerio/wasmer/issues/2781). The wasm cache recreation is not mandatory because CosmWasm will detect and recreate wasm cache if the cache version is old. The query nodes, however, need the recreation step because wasm cache compilation would takes some delay and make node irresponsible.

### How to build
```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# register cargo to PATH env
source $HOME/.cargo/env

# build
cargo build
```
### How to use

```shell
cargo run -- $TERRA_HOME/data/wasm
```
