# defi kit

### Before use
1. Do not provide your `PRIVATE_KEY` until you are ready
2. Approve to Zerion router must be submitted manually (through interface, for
   example)
3. `INPUT_TOKEN`, `OUTPUT_TOKEN` are zerion asset ids
4. `CHAIN` is used for building transaction but it will be submitted to testnet
   if `NODE_URL` points there

### How to use
1. Clone repo
2. Create `.env` file with contents:
```
RUST_LOG=info

NODE_URL=https://optimism-mainnet.infura.io/...

WALLET_ADDRESS=0x...
CHAIN=optimism
INPUT_TOKEN=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
OUTPUT_TOKEN=eth
AMOUNT=10000000
SLIPPAGE=1

PRIVATE_KEY=...
```
3. `cargo run`
