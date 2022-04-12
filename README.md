## CosmWasm Cache Rebuilder

To bump wasmer version from 2.0.0 to 2.1.1, we need to recreate serialized wasm cache due to [this issue](https://github.com/wasmerio/wasmer/issues/2781). The wasm cache recreation is not mandatory because CosmWasm will detect and recreate wasm cache if the cache version is old. The query nodes, however, would require this recreation step to decrease response latency.

### Installation
```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# register cargo to PATH env
source $HOME/.cargo/env

# clone the repository
git clone https://github.com/terra-money/cosmwasm-cache-rebuilder.git; cd cosmwasm-cache-rebuilder
```

### Run
TERRA_HOME variable should be changed according to your local environment (Default: $HOME/.terra/data/wasm)
```shell
cargo run --release -- $TERRA_HOME/data/wasm
```
