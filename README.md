# Simplicity CTF

The first-ever [Simplicity](https://github.com/BlockstreamResearch/simplicity) capture-the-flag challenge.

A locked 0.01 L-BTC reward sits behind two cooperating contracts. Your task is to find how to unlock these contracts and claim the reward!

> Special thanks to [@Hrom131](https://github.com/Hrom131) for coding the challenge.

## The Challenge

Here is the funding [transaction](https://blockstream.info/liquid/tx/aa52a138a0e193c8530e1195b201c7139de194decc0ff3bb01489adbe814095c).

Here are the contracts [source code](./simf).

The rest is up to you to figure out.

## Prerequisites

If you wish to use this repository to solve the CTF, please make sure to have the following available:

- [Rust](https://rustup.rs/) 1.91.0 (see `rust-version` in `Cargo.toml`).
- [simplexup](https://github.com/BlockstreamResearch/smplx/blob/master/simplexup/README.md).

Then run `simplexup --install v0.0.8` to install the Simplex framework.

### Setup

```bash
# Generate Simplicity artifacts (required before Rust build)
simplex build

# Build the project
cargo build

# Run the tests (your solution)
simplex test -v
```

Check out the [Simplex](https://github.com/BlockstreamResearch/smplx/tree/master/examples/basic) framework example to learn more.

## Disclaimer

GLHF!
