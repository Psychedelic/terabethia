sudo dfx canister --no-wallet create --all
cargo run > token.did
ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
sudo dfx build token
OWNER="principal \"$( \
   dfx identity get-principal
)\""

#  logo: String,
#  name: String,
#  symbol: String,
#  decimals: u8, 1e18
#  total_supply: u64, 
#  owner: Principal,
#  fee: u64,

sudo dfx canister --no-wallet install token --argument "(
   \"NA\", 
   \"Wrapped Ether\", 
   \"WETH\", 
   18:nat8, 
   0:nat64, 
   $OWNER, 
   0
)" -m=reinstall
