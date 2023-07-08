use crate::*;
// use near_sdk::ext_contract;

#[near_bindgen]
impl Contract {
    //Requests from affiliates
    #[payable]
    pub fn affiliate_request(&mut self, id: U64, affiliate_id: AccountId) {
        // Measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let check_existing = self
            .affiliate_requests
            .iter()
            .position(|a| {
                a.account_id == affiliate_id
                    && a.series_id == id
            })
            .unwrap_or_else(|| 11111111);

        match self.affiliate_requests.get(check_existing as u64) {
            Some(a) => panic!("Already applied to become an affiliate: {:?}", a),
            None => {
                // Get the series and how many tokens currently exist (edition number = cur_len + 1)
                let series = self.series_by_id.get(&id.0).expect("Not a series");

                assert!(series.affiliate.is_some(), "This series does not accept affiliate");

                assert!(env::attached_deposit() > ONE_YOCTO, "Must attach upto 0.1 near to this call");

                //specify the token struct that contains the owner ID
                let request = AffiliatesRequests {
                    // Affiliate account ID
                    account_id: affiliate_id,
                    // SERIES ID of product
                    series_id: id.clone(),
                    // Status of request
                    approved: false,
                };

                self.affiliate_requests.push(&request);

                //calculate the required storage which was the used - initial
                let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

                // refund storage used
                refund_deposit(required_storage_in_bytes);
            }
        }
    }

    pub fn approve_affiliate(&mut self, id: U64, affiliate_id: AccountId, percentage: u32) -> Option<AffiliatesRequests> {
        self.assert_contract_owner();

        let index = self
            .affiliate_requests
            .iter()
            .position(|a| {
                a.account_id == affiliate_id
                    && a.series_id == id
                && a.approved == false
            })
            .unwrap_or_else(|| 11111111);

        match self.affiliate_requests.get(index as u64) {
            Some(a) => {
                let series = self.series_by_id.get(&id.0).expect("Not a series");

                if let Some(mut affix) = series.affiliate {
                    assert!(
                        affix.contains_key(&affiliate_id),
                        "Affiliateer is already approved for this product"
                    );
                    affix.insert(affiliate_id, percentage);
                }

                {
                    self.affiliate_requests.replace(
                        index as u64,
                        &AffiliatesRequests {
                            account_id: a.account_id,
                            series_id: a.series_id,
                            approved: true,
                        },
                    );
                }
                let affiliate = self.affiliate_requests.get(index.clone() as u64);

                return affiliate
            }
            None => panic!("Couldn't find affiliate or affiliate already exists"),
        }
    }

    pub fn get_affiliates(&self) -> Vec<AffiliatesRequests> {
        let affiliates: Vec<AffiliatesRequests> = self.affiliate_requests.iter().map(|x| x).collect();

        affiliates
    }
}