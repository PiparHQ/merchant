use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault,
    Promise, PromiseOrValue, is_promise_success, Gas,
};
use std::collections::HashMap;

pub use crate::approval::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;
pub use crate::owner::*;
pub use crate::royalty::*;
pub use crate::series::*;
pub use crate::factory::*;
pub use crate::reward::*;
pub use crate::affiliate::*;

mod approval;
mod enumeration;
mod events;
mod internal;
mod metadata;
mod nft_core;
mod owner;
mod royalty;
mod series;
mod factory;
mod reward;
mod affiliate;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

// cost of deploying FT token
pub const TOKEN_BALANCE: u128 = 4_000_000_000_000_000_000_000_000;

// 0.1 near in yocto
pub const ONE_YOCTO: u128 = 10_000_000_000_000_000_000_000;

// Attach 0 near token
pub const NO_DEPOSIT: Balance = 0;

// gas calculation
pub const fn tgas(n: u64) -> Gas {
    Gas(n * 10u64.pow(12))
}

// Genereal gas to use for cross contract calls
pub const GAGAS: Gas = tgas(35 + 5);

// Represents the series type. All tokens will derive this data.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Series {
    // Metadata including title, num copies etc.. that all tokens will derive from
    metadata: TokenMetadata,
    // colors of this product
    colors: HashMap<String, u32>,
    // Royalty used for all tokens in the collection
    royalty: Option<HashMap<AccountId, u32>>,
    // List of affiliates for all the tokens in this series collection
    affiliate: Option<HashMap<AccountId, u32>>,
    // Set of tokens in the collection
    tokens: UnorderedSet<TokenId>,
    // What is the price of each token in this series? If this is specified, when minting,
    // Users will need to attach enough $NEAR to cover the price.
    price: Option<Balance>,
    // Owner of the collection
    owner_id: AccountId,
}

pub type SeriesId = u64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //marketplace contract id
    pub marketplace_contract_id: AccountId,

    //store token boolean
    pub token: bool,

    //cost of deploying a token
    pub token_cost: U128,

    //affiliate requests
    pub affiliate_requests: Vector<AffiliatesRequests>,

    //approved minters
    pub approved_minters: LookupSet<AccountId>,

    //approved users that can create series
    pub approved_creators: LookupSet<AccountId>,

    //Map the collection ID (stored in Token obj) to the collection data
    pub series_by_id: UnorderedMap<SeriesId, Series>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: UnorderedMap<TokenId, Token>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of tokens currently locked pending transaction completion
    pub tokens_locked: LookupSet<String>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    ApprovedMinters,
    ApprovedCreators,
    ApprovedAffiliates,
    PendingAffiliates,
    SeriesById,
    SeriesByIdInner { account_id_hash: CryptoHash },
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokensLocked,
    NFTContractMetadata,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId, marketplace_contract_id: AccountId, name: String, symbol: String, icon: Option<String>, bg_icon: Option<String>, category: Option<String>, description: Option<String>, facebook: Option<String>, twitter: Option<String>, instagram: Option<String>, tiktok: Option<String>, youtube: Option<String>, zip: Option<String>, city: Option<String>, state: Option<String>, country: Option<String>) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            marketplace_contract_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: name,
                symbol: symbol,
                icon: icon,
                bg_icon: bg_icon,
                category: category,
                description: description,
                facebook: facebook,
                twitter: twitter,
                instagram: instagram,
                tiktok: tiktok,
                youtube: youtube,
                zip: zip,
                city: city,
                state: state,
                country: country,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, marketplace_contract_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // Create the approved minters set and insert the owner
        let mut approved_minters =
            LookupSet::new(StorageKey::ApprovedMinters.try_to_vec().unwrap());
        approved_minters.insert(&owner_id);
        approved_minters.insert(&marketplace_contract_id);

        // Create the approved creators set and insert the owner
        let mut approved_creators =
            LookupSet::new(StorageKey::ApprovedCreators.try_to_vec().unwrap());
        approved_creators.insert(&owner_id);
        
        // Create a variable of type Self with all the fields initialized.
        let this = Self {
            approved_minters,
            approved_creators,
            affiliate_requests: Vector::new(StorageKey::PendingAffiliates.try_to_vec().unwrap()),
            series_by_id: UnorderedMap::new(StorageKey::SeriesById.try_to_vec().unwrap()),
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: UnorderedMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            tokens_locked: LookupSet::new(StorageKey::TokensLocked.try_to_vec().unwrap()),
            //set the &owner_id field equal to the passed in owner_id.
            owner_id,
            //set the &marketplace_id field equal to the passed in marketplace_id.
            marketplace_contract_id,
            token: false,
            token_cost: U128::from(TOKEN_BALANCE),
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}