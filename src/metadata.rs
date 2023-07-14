use crate::*;
pub type TokenId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "PUMA"
    pub symbol: String,            // required, ex. "PMA"
    pub icon: Option<String>,      // logo image for store
    pub bg_icon: Option<String>,   // background image for store
    pub category: Option<String>,  // category of store
    pub description: Option<String>, // description of store
    pub facebook: Option<String>,   // facebook page social media link
    pub twitter: Option<String>,    // twitter page social media link
    pub instagram: Option<String>,  // instagram page social media link
    pub tiktok: Option<String>,     // tiktok page social media link
    pub youtube: Option<String>,    // youtube page social media link
    pub zip: Option<String>,        // store location zip
    pub city: Option<String>,       // city location of store
    pub state: Option<String>,      // state location of store
    pub country: Option<String>,    // country location of store
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: String, // free-form description
    pub media: String, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub buy_timeout: u64, // time seller agrees to fulfil an order to a buyer else the buyer gets refunded
    pub is_discount: bool, // does the seller want to give the buyer a discount on this purchase
    pub discount_percent: u64, // upto to what percentage %?
    pub token_amount_per_unit: U128, // number of tokens to give up by the buyer before accessing this discount
    pub is_reward: bool, // does the seller want to reward a buyer with a it's store tokens after successfully purchasing this product
    pub reward_amount_per_unit: U128, // amount of token to give the buyer after purchasing this product
    pub is_custom_user: bool, // is this series collection made for a particular user?
    pub user: Option<String>, // if yes, what is the ID of that user
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // Series that the token belongs to
    pub series_id: u64,
    //owner of the token
    pub owner_id: AccountId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //the next approval ID to give out.
    pub next_approval_id: u64,
}

//The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    // Series that the token belongs to
    pub series_id: u64,
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    // colors of this product
    pub colors: HashMap<String, u32>,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: Option<HashMap<AccountId, u32>>,
    //keep track of the price for the token
    pub price: Option<Balance>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageData {
    pub(crate) account_id: AccountId,
    pub(crate) registration_only: bool,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenData {
    pub(crate) receiver_id: AccountId,
    pub(crate) amount: U128,
    pub(crate) memo: String,
}

// Represents the series type. All tokens will derive this data.
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AffiliatesRequests {
    // Affiliate account ID
    pub(crate) account_id: AccountId,
    // SERIES ID of product
    pub(crate) series_id: U64,
    // Status of request
    pub(crate) approved: bool,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketplaceData {
    pub(crate) price: Balance,
    pub(crate) affiliate: bool,
    pub(crate) affiliate_id: Option<AccountId>,
    pub(crate) affiliate_percentage: Option<u32>,
    pub(crate) token_id: String,
    pub(crate) token_owner: AccountId,
    pub(crate) store_owner: AccountId,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
