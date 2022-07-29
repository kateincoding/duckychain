cargo build --target wasm32-unknown-unknown --release ;
cat neardev/dev-account >> neardev/history ;
rm ./neardev/dev-account.env ;
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/yellow_pages.wasm ;
WALLET=`cat ./neardev/dev-account` ;
near call $WALLET add_entry --args '{"category": "Restaurant", "phone": 1234567, "name": "pizza hut", "address": "Lima, Peru", "website": "pz.com"}' --accountId $WALLET ;
near view $WALLET get_entry --args '{"category": "Restaurant", "name": "pizza hut"}'
