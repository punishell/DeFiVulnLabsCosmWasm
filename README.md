# DeFiVulnLabs

![Logo](https://raw.githubusercontent.com/punishell/DeFiVulnLabsCosmWasm/master/DeFiVulnLabsLogo.png)

This is  an Web3 cosmwasm security training which is contribution to [DeFiVulnLabs](https://github.com/SunWeb3Sec/DeFiVulnLabs). I want to share these materials with everyone interested in Web3 security and how to find vulnerabilities in code and exploit them. 

  
##### Education only! Please do not use it in production.

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed. Currently, we are testing on 1.44.1+.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## Compiling and running tests

Now that you created your custom contract, make sure you can compile and run it before
making any changes. Go into the

```sh
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test

# this runs integration tests with cranelift backend (uses rust stable)
cargo integration-test

# this runs integration tests with singlepass backend (needs rust nightly)
cargo integration-test --no-default-features --features singlepass

# auto-generate json schema
cargo schema
```

The wasmer engine, embedded in `cosmwasm-vm` supports multiple backends:
singlepass and cranelift. Singlepass has fast compile times and slower run times,
and supportes gas metering. It also requires rust `nightly`. This is used as default
when embedding `cosmwasm-vm` in `go-cosmwasm` and is needed to use if you want to
check the gas usage.

However, when just building contacts, if you don't want to worry about installing
two rust toolchains, you can run all tests with cranelift. The integration tests
may take a small bit longer, but the results will be the same. The only difference
is that you can not check gas usage here, so if you wish to optimize gas, you must
switch to nightly and run with cranelift.

## List of vulnerabilities
* [Insufficient Access Control](contracts/access/tests/access_control.rs) 
* [Insufficient Token Address Validation](contracts/receive/tests/receive.rs) 
* [Lack of address normalization](contracts/normalization/tests/normalize.rs) 
* [Default Values](contracts/default_values/tests/default_values.rs) 
* Rounding Issues 
* Loss of Precision
* Denial of Service


## Link reference

* [Comparison with Solidity Contracts](https://docs.cosmwasm.com/docs/0.14/architecture/smart-contracts/)
* [Top 10 DeFi Security Best Practices](https://blog.chain.link/defi-security-best-practices/)
* [Cw-Contracts](https://github.com/deus-labs/cw-contracts) 
* [Cw20 Allowance](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md#allowances)

