# mini-bitcoin 🦀

A simple implementation of Bitcoin protocol.

I follow [Programming Bitcoin](https://www.amazon.com/Programming-Bitcoin-Learn-Program-Scratch/dp/1492031496)
book as a point of reference.

## Build the project
```bash
git clone <REPO_URL>
cargo build --release
target/release/bitcoin-rs
```

## Run tests
```bash
cargo test
```

## Q/A
Q: What's your plan with this repo?
A: I just want to learn about Bitcoin and networking that's all
<br>
Q: Why is it so slow?
A: Because I haven't optimized BigNumber operations and I don't plan to optimize them unless I throughly understand the optimizations mathemetically.
