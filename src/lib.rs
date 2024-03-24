use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, StorageUsage, BorshStorageKey, IntoStorageKey};

pub mod nearderthal_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;


use crate::metadata::*;
use crate::events::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    pub accounts: LookupMap<AccountId, u128>,
    pub total_supply: u128,
    pub metadata: LazyOption<FungibleTokenMetadata>,
    /// The bytes for the largest possible account ID that can be registered on the contract
    pub bytes_for_longest_account_id: StorageUsage,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                icon: Some(DATA_IMAGE_SVG.to_string()),
                reference: None,
                reference_hash: None,
                decimals: DECIMALS,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized.
        let mut this = Self {
            total_supply: total_supply.0,
            bytes_for_longest_account_id: 0,
            metadata: LazyOption::new(
                StorageKey::Metadata.into_storage_key(),
                Some(&metadata),
            ),
            accounts: LookupMap::new(StorageKey::Accounts.into_storage_key())
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());

        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
            .emit();

        this
    }
}