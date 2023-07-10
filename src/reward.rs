use crate::*;
// use near_sdk::ext_contract;

#[near_bindgen]
impl Contract {
    pub fn reward_with_token(
        &mut self,
        id: U64, receiver_id: AccountId,
    ) -> Promise {
        //ensure smart contract is only called by pipar marketplace
        self.assert_marketplace_contract();
        //confirm seller has a token deployed
        assert_eq!(
            true, self.token,
            "Store owner has not deployed a token yet"
        );

        // Get the series and how many tokens currently exist (edition number = cur_len + 1)
        let mut series = self.series_by_id.get(&id.0).expect("Not a series");

        assert_eq!(series.metadata.is_reward, true, "There's no token reward for this product");

        let token_quantity = series.metadata.reward_amount_per_unit;
        let mem = format!("Thank You for Shopping at {:?}!", env::current_account_id());
        let current_account = env::current_account_id().to_string();
        let token_account: AccountId = format!("ft.{current_account}").parse().unwrap();
        let storage_args = serde_json::to_vec(&StorageData {
            account_id: receiver_id.clone(),
            registration_only: false,
        })
            .unwrap();

        let token_args = serde_json::to_vec(&TokenData {
            receiver_id: receiver_id.clone(),
            amount: token_quantity,
            memo: mem,
        })
            .unwrap();

        Promise::new(token_account.clone())
            .function_call(
                "storage_deposit".to_owned(),
                storage_args,
                ONE_YOCTO,
                GAGAS,
            )
            .function_call(
                "ft_transfer".to_owned(),
                token_args,
                NO_DEPOSIT,
                GAGAS,
            )
            .then(
                Self::ext(env::current_account_id())
                    .reward_with_token_callback(token_quantity.clone()),
            )
    }

    #[private]
    pub fn reward_with_token_callback(&self, token_quantity: U128) -> String {
        let token: u128 = token_quantity.into();
        if is_promise_success() {
            let res = format!("Sent {token} token successfully!");

            res
        } else {
            let res = format!("failed sending token");

            res
        }
    }
}