# NEARderthal

Meme coin on the NEAR Blockchain.
Token aligned to the NIP-141 Standard.

Learn along Nearderthal to create your own token on NEAR.
Contract account: https://testnet.nearblocks.io/es/token/transfer.mainjungle.testnet#
![NEARderthal](/docs/Nearderthal.webp)

## Launch your token
The objective of this project is to give a template for those who want to launch their own token on the NEAR blockchain.
Alongside the template I'll also give you the updated scripts on the near-cli-rs

This projects assume the development will be in RUST not in javascript.

## Prerequisites
To launch your own token successfully, you'll need:

Rust
A NEAR TestNet account
NEAR-CLI-rs

## Set up
Install NEAR-CLI-rs

```
npm i -g near-cli-rs cargo-near
```

Create the project
```
cargo near new <project_name>
```

Create a new testnet account

Replace <created-account> with a custom name
```
cargo-near near create-dev-account use-specific-account-id <created-account> autogenerate-new-keypair save-to-keychain network-config testnet create
```

```
near login
```

Save account name as environment variable for easier use
```
export NEARID=YOUR_ACCOUNT_NAME
```

### lib.rs
In lib.rs resides the definition of the Contract struct.
And the two functions for deploying the contract.

### events.rs
Implement the Events standard and specifically for the NIP-141 implements the Mint and Transfer events.

### interal.rs
Internal functions for the smart contract

### metadata.rs
The metadata associated to the TOKEN

### nearderthal_core.rs
The core of the contract where the core standard is defined and implemented.

### storage.rs
Storage standard implementation.
Accounts must register first in order to pay for the storage on the contract.
In this case the min and max storage utilized will be the maximum size of an account.


## Deploy and Test your TOKEN

### Create a sub account:

```
near account create-account fund-myself transfer.$NEARID '25 NEAR' autogenerate-new-keypair save-to-keychain sign-as $NEARID network-config testnet sign-with-keychain send
```

### Add to variable for convenience
```
export TRANSFER_ACC_ID=transfer.$NEAR_ID
```

### Deploy contract to testnet
```
near contract deploy $TRANSFER_ACC_ID use-file out/contract.wasm without-init-call network-config testnet sign-with-keychain send
```
### Initialize the contract
```
near contract call-function as-transaction $TRANSFER_ACC_ID new_default_meta json-args '{"owner_id": "$TRANSFER_ACC_ID", "total_supply": "1000000000000000000000000000"}' prepaid-gas '30 TeraGas' attached-deposit '0 NEAR' sign-as $TRANSFER_ACC_ID network-config testnet sign-with-keychain send
```
### Check balance:
```
near contract call-function as-read-only $TRANSFER_ACC_ID ft_balance_of text-args '{"account_id": "$TRANSFER_ACC_ID"}’ network-config testnet now
```
### Transfer to your main or other account

#### First register it
```
near contract call-function as-transaction $TRANSFER_ACC_ID storage_deposit json-args '{"account_id": "$NEAR_ID"}' prepaid-gas '30 TeraGas' attached-deposit '1 NEAR' sign-as transfer.mainjungle.testnet network-config testnet sign-with-keychain send
```

#### Once the account is registered, you can transfer the FTs by running the following command. Take note that you're also attaching exactly 1 yoctoNEAR.
```
near contract call-function as-transaction $TRANSFER_ACC_ID ft_transfer json-args '{"receiver_id": "$NEAR_ID", "amount": "1000000000000000000000000", "memo": "Genesis tx, or the birth of the first nearderthal?"}' prepaid-gas '30 TeraGas' attached-deposit '1 YNEAR' sign-as $TRANSFER_ACC_ID network-config testnet sign-with-keychain send
```


## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Iteract with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
