use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtData {
    owner_id: AccountId,
    total_supply: U128,
    name: String,
    symbol: String,
    icon: String,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn deploy_token(
        &mut self,
        total_supply: U128,
        name: String,
        symbol: String,
        icon: String,
    ) -> Promise {
        assert!(
            env::attached_deposit() > TOKEN_BALANCE,
            "To cover the storage required for your ft contract, you need to attach at least {} yoctoNEAR to this transaction.",
            TOKEN_BALANCE
        );
        self.assert_contract_token_false();
        self.assert_contract_owner();
        let current_account = env::current_account_id().to_string();
        let subaccount: AccountId = format!("ft.{current_account}").parse().unwrap();
        assert!(
            env::is_valid_account_id(subaccount.as_bytes()),
            "Invalid subaccount"
        );
        let init_args = serde_json::to_vec(&FtData {
            owner_id: env::current_account_id(),
            total_supply,
            name,
            symbol,
            icon,
        })
            .unwrap();

        Promise::new(subaccount.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(TOKEN_BALANCE.into())
            .deploy_contract(include_bytes!("../wasm/pipar_fungible_token.wasm").to_vec())
            .function_call(
                "new_default_meta".to_owned(),
                init_args,
                NO_DEPOSIT,
                GAGAS,
            )
            .then(Self::ext(env::current_account_id()).deploy_token_callback(
                env::predecessor_account_id(),
                env::attached_deposit().into(),
            ))
    }

    #[private]
    pub fn deploy_token_callback(&mut self, token_creator_id: AccountId, attached_deposit: U128) {
        let attached_deposit: u128 = attached_deposit.into();
        if is_promise_success() {
            self.token = true;
            env::log_str("Successful token deployment")
        } else {
            Promise::new(token_creator_id).transfer(attached_deposit);
            env::log_str("failed token deployment")
        }
    }

}