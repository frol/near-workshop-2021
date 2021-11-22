Check that everything is compiling fine:

```
cargo check
```

Build Wasm binary:

```
cargo build --target wasm32-unknown-unknown --release
```

Call a read-only function (`get_message`) through JSON RPC via cURL:

```
curl https://rpc.testnet.near.org -H Content-Type:application/json -X POST --data '{"jsonrpc": "2.0", "id": "dontcare", "method": "query", "params": {"request_type": "call_function", "account_id": "frol-workshop-2021.testnet", "method_name": "get_message", "args_base64": "eyJ1c2VybmFtZSI6ICJmcm9sOS50ZXN0bmV0In0=", "finality": "final"}}'
```

Call a read-only function (`get_message`) through JSON RPC via near-cli:

```
./near-cli execute view-method network testnet contract frol-workshop-2021.testnet call get_message '{"username": "frol9.testnet"}' at-final-block
```

Call an update function (`set_message`) by submitting a transaction via near-cli:

```
./near-cli \
    execute change-method \
    network testnet \
    contract frol-workshop-2021.testnet \
    call set_message '{"username": "frol9.testnet", "message": "asd"}' \
        --attached-deposit '0 NEAR' \
        --prepaid-gas '100.000 TeraGas' \
    signer frol9.testnet \
    sign-with-keychain
```

NOTE: Update frol9.testnet with your own account id

WARNING: Based on our current implementation, `set_message` is allowed to set the message for any given `username`. We should use [`near_sdk::env::predecessor_account_id()`](https://docs.rs/near-sdk/3.1.0/near_sdk/env/fn.predecessor_account_id.html) to ensure the idendity rather than relying on the account id passed in the parameters. See the improved version [here](https://github.com/frol/near-workshop-2021/compare/improved-set-message)
