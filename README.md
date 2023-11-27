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
- What's your plan with this repo? <br>
  A: I just want to learn about Bitcoin and networking, that's all
- Why is it so slow? <br>
  A: Because I haven't optimized BigNumber operations and I don't plan to optimize them unless I throughly understand the optimizations mathemetically.
