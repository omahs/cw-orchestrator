# Juno

Juno stands as the central hub for CosmWasm smart contracts, underpinned by the InterWasm DAO. As a global, open-source, and permission-less platform, Juno champions the forefront of CosmWasm development, enabling developers to seamlessly deploy inter-chain smart contracts crafted in Rust. The network's inclusive design ensures that anyone can innovate and engage with inter-chain applications.

[Visit Juno's Website](https://junonetwork.io/)

## Usage

See how to setup your main function in the [main function](../single_contract/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::JUNO_1`.

```rust,ignore
{{#include ../../../cw-orch/src/daemon/networks/juno.rs:juno}}
```

## References

- [Juno Documentation](https://docs.junonetwork.io/juno/readme)
- [Juno Discord](https://discord.junonetwork.io/)
