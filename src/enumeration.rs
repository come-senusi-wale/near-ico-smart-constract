use crate::*;

#[near_bindgen]
impl Contract {

    //query for mimumu amount to invest at  particular time
    pub fn query_mininum_amount_to_invest(&self)->u128{
        self.minimun_amount_invest
    }

    //query for equivalent near token reward
    pub fn query_equivalent_token_reward(&self)->u128{
        self.equivalent_near_token_reward
    }

    //query for maximun token ivestor can mint or buy
    pub fn query_maximum_token_ivestor_can_buy(&self)->u128{
        self.maximun_token
    }

    //query for ico closing time in minisecond
    pub fn query_ico_closing_date(&self)->u64{
        self.ico_closing_date
    }

    //query timesstanp
    // pub fn query_timeStamp(&self)->u64{
    //     env::block_timestamp()
    // }

    //query for expected money by company from all investor
    pub fn query_expected_money(&self)->u128{
        self.expected_money
    }

    //query for all the money invested by investor 
    pub fn query_amoun_invested_so_far(&self)->u128{
        self.amount_invested_so_far
    }

    //query for investor data
    pub fn investor_details(&self, account: AccountId)-> InvestorMetadata{
        self.investor_lists.get(&account).unwrap()
    }

      //query for member data
      pub fn member_details(&self, account: AccountId)-> MemberMetadata{
        self.member_lists.get(&account).unwrap()
    }

}