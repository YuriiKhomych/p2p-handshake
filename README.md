# p2p-handshake
p2p-handshake is a Command Line Interface (CLI) tool designed for making P2P handshakes with blockchain nodes. It currently supports the following blockchain networks:
* [Bitcoin network handshake](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).
* [Ethereum handshake](https://github.com/ethereum/devp2p/blob/master/rlpx.md#the-rlpx-transport-protocol)

You can obtain a list of nodes from the following sources:
* [Bitcoin nodes](https://bitnodes.io/nodes/?q=Satoshi:25.0.0)
* [Ethereum nodes](https://etherscan.io/nodetracker/nodes)

## Example Usage and Output
##### Bitcoin handshake
```bash
$ p2p-handshake btc 178.238.233.75:8333 108.208.224.203:8333
2023-11-01T12:42:47.286142Z  INFO p2p_handshake::p2p::btc: [178.238.233.75:8333] Perform a P2P handshake took 211.88ms
2023-11-01T12:42:47.286222Z  INFO p2p_handshake::p2p: [successful] [178.238.233.75]
2023-11-01T12:42:47.576376Z  INFO p2p_handshake::p2p::btc: [108.208.224.203:8333] Perform a P2P handshake took 502.12ms
2023-11-01T12:42:47.576480Z ERROR p2p_handshake::p2p: [failed] [108.208.224.203:8333] error: deadline has elapsed: Tokio elapsed error: P2P handshake error
```

##### Ethereum handshake
```bash
$ p2p-handshake eth enode://7723cea4576dd5b4b92dad365da58604329866e84ad0689d86892566c087fce6f87836467dc9c9ab59fc03eeae3eede68e01b4984c4bba60ec20fc25063a3ecc@3.239.83.130:30303 enode://5a73de9456c7eda38d28fb47a8561250f6ca49a640b0f3628a9df71cc90ee1aa7c2007f1d88412f4fe2f6cfae671b5fa023340b19c0d8d905b650ddd8b4c615e@135.181.1.189:30304 enode://7723cea4576dd5b4b92dad365da58604329866e84ad0689d86892566c087fce6f87836467dc9c9ab59fc03eeae3eede68e01b4984c4bba60ec20fc25063a3ecc@3.239.83.130:30303 enode://7723cea4576dd5b4b92dad365da58604329866e84ad0689d86892566c087fce6f87836467dc9c9ab59fc03eeae3eede68e01b4984c4bba60ec20fc25063a3ecc@3.239.82.130:30303

2023-10-28T13:25:33.498171Z  INFO p2p_handshake_eth::p2p::eth: [135.181.1.189] Perform a P2P handshake took 118.42ms
2023-10-28T13:25:33.648393Z  INFO p2p_handshake_eth::p2p::eth: [3.239.82.130] Perform a P2P handshake took 268.64ms
2023-10-28T13:25:33.882262Z  INFO p2p_handshake_eth::p2p::eth: [3.239.83.130] Perform a P2P handshake took 502.51ms
2023-10-28T13:25:33.882263Z  INFO p2p_handshake_eth::p2p::eth: [3.239.83.130] Perform a P2P handshake took 502.51ms
2023-10-28T13:25:33.882335Z ERROR p2p_handshake_eth::p2p: [failed] [3.239.83.130] error: deadline has elapsed: Tokio elapsed error: P2P handshake error
2023-10-28T13:25:33.882349Z  INFO p2p_handshake_eth::p2p: [successful] [135.181.1.189]
2023-10-28T13:25:33.882355Z ERROR p2p_handshake_eth::p2p: [failed] [3.239.83.130] error: deadline has elapsed: Tokio elapsed error: P2P handshake error
2023-10-28T13:25:33.882361Z  INFO p2p_handshake_eth::p2p: [successful] [3.239.82.130]
```
For each node provided, the CLI will attempt to perform a P2P handshake and display the time taken to complete it, as well as the result of the handshake.

## Architecture Decision Record

For detailed information on the architecture decisions made for this project, please refer to the [ADR document](ADR.md).

## How to run

To run this tool, you'll need to have Rust installed, which you can get from [rust installation](https://rustup.rs/).

There are two ways to run the tool:
### Using cargo

```bash
$ cargo run --release -- btc <ip_address:port> <ip_address:port>
$ cargo run --release -- eth enode://<node_id@ip_address:port> enode://<node_id@ip_address:port>
```
### Install the CLI binary in your system

```bash
$ cargo install --path .
$ p2p-handshake btc <ip_address:port> <ip_address:port>
$ p2p-handshake eth enode://<node_id@ip_address:port> enode://<node_id@ip_address:port>
```

To view all available options and commands, use the following command:

```bash
$ p2p-handshake --help
Usage: p2p-handshake [OPTIONS] <COMMAND>

Commands:
  eth   Perform a P2P handshake with the ethereum network nodes
  btc   Perform a P2P handshake with the bitcoin network nodes
  help  Print this message or the help of the given subcommand(s)

Options:
  -t, --timeout <TIMEOUT>  handshake operation maximum time (in ms) [default: 500]
  -h, --help               Print help
  -V, --version            Print version
```


You can set the log level to `debug` or `trace` by using the `RUST_LOG` environment variable to get more detailed information about the handshake process.
```bash
$ RUST_LOG=trace p2p-handshake btc <ip_address:port> <ip_address:port>
$ RUST_LOG=debug p2p-handshake eth enode://<node_id@ip_address:port> enode://<node_id@ip_address:port>
```

## How to contribute

### Development workflow
For development workflow for this project we use:
1. [Cargo make workflow](https://github.com/sagiegurari/cargo-make).
To install it, use the next steps described in [Installation](https://github.com/sagiegurari/cargo-make#installation) section. Workflow is defined in [workflow.toml](workflow-dev.toml) file.
Then you can run the following command to setup the make workflow development:

```bash
cargo make --makefile workflow-dev.toml workflow-dev
```

2. [Pre-commit](https://pre-commit.com), a framework for managing and maintaining multi-language pre-commit hooks. To install it, use the next steps described in [Installation](https://pre-commit.com/#installation) section.
Pre commit hooks are defined in [.pre-commit-config.yaml](.pre-commit-config.yaml) file.
Then you can run the following command to setup the pre-commit hooks and run them:
```bash
pre-commit install && pre-commit run --all-files
```
Pre commit hooks are run automatically on every commit.

### How to run the tests
Tests are also run automatically on every commit but you can run them manually.
To run unit tests which located with the library source code, use the following command:
```bash
cargo test --lib
```
We also have integration tests which located in [tests](tests) folder.
```bash
cargo test --test test_eth_handshake
```
