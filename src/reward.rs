use crate::*;
// use near_sdk::ext_contract;

#[near_bindgen]
impl Contract {
    // pub fn reward_with_token(
    //     &mut self,
    //     product_id: U128,
    //     quantity: U128,
    //     buyer_account_id: AccountId,
    // ) -> Promise {
    //     self.assert_marketplace_contract();
    //
    //     let product_index = self
    //         .products
    //         .iter()
    //         .position(|p| p.product_id == product_id)
    //         .unwrap_or_else(|| 11111111);
    //
    //     match self.products.get(product_index as u64) {
    //         Some(product) => {
    //             assert!(
    //                 &product.is_reward,
    //                 "Product cannot be rewarded with tokens"
    //             );
    //             let reward = product.reward_amount.clone();
    //             let t: u128 = reward.into();
    //             let q: u128 = quantity.into();
    //             let token_quantity = &t * &q;
    //             let mem = format!("Thank You for Shopping at {:?}!", env::current_account_id());
    //             let current_account = env::current_account_id().to_string();
    //             let token_account: AccountId = format!("ft.{current_account}").parse().unwrap();
    //
    //             let storage_args = serde_json::to_vec(&StorageData {
    //                 account_id: buyer_account_id.clone(),
    //                 registration_only: false,
    //             })
    //                 .unwrap();
    //
    //             let token_args = serde_json::to_vec(&TokenData {
    //                 receiver_id: buyer_account_id.clone(),
    //                 amount: token_quantity.into(),
    //                 memo: mem,
    //             })
    //                 .unwrap();
    //
    //             Promise::new(token_account.clone())
    //                 .function_call(
    //                     "storage_deposit".to_owned(),
    //                     storage_args,
    //                     ONE_YOCTO,
    //                     GAGAS,
    //                 )
    //                 .function_call(
    //                     "ft_transfer".to_owned(),
    //                     token_args,
    //                     NO_DEPOSIT,
    //                     GAGAS,
    //                 )
    //                 .then(
    //                     Self::ext(env::current_account_id())
    //                         .reward_with_token_callback(token_quantity.clone()),
    //                 )
    //         }
    //         None => panic!("Couldn't find product"),
    //     }
    // }
    //
    // #[private]
    // pub fn reward_with_token_callback(&self, token_quantity: u128) -> String {
    //     if is_promise_success() {
    //         let res = format!("Sent {token_quantity} token successfully!");
    //
    //         res
    //     } else {
    //         let res = format!("failed sending token");
    //
    //         res
    //     }
    // }
}