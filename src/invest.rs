use crate::*;

pub trait InvestCore {

    fn invest(
        &mut self,
        amount: u128,
    );
}

#[near_bindgen]
impl InvestCore for Contract {

    #[payable]
    fn  invest(&mut self, amount: u128,){

        //gettin investor account id
        let signers = env::predecessor_account_id();
        
        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        //checking the attach deposit is greater than amount due for storage cost
        assert!(
            deposite > amount,
            "the attach deposite must be greater than amount invest",
        );

        let remaining_deposite_for_storage = deposite - amount;

        //checking the amount investing is greater than or equal to minimun amount investor can invest
        assert!(
            amount >= self.minimun_amount_invest,
            "amount invest must be greater than or equal to minimun amount to be invested which is {}",
            self.minimun_amount_invest,
        );

        // //checking if the ico date has not pass
        assert!(
            self.calculate_ico_time() == true,
            "the crow sale time has finised",
        );

        //checking that amount must greater than zero
        assert!(
            self.expected_money > 0,
            "the expected money  has not be set",
        );

        let incase_over_flow = amount + self.amount_invested_so_far;

        //checking the investor can not invest more than expected money by companey
        assert!(
            self.expected_money >= incase_over_flow,
            "you have invested more than money expected by company",
        );

        //checking whether expected money from company has reach
        assert!(
            self.expected_money > self.amount_invested_so_far,
            "the money needed by company has reach no more investing",
        );

        //checking if a particular investor has already invest
        if let Some(mut investor_list) = self.investor_lists.get(&signers) {
            
            let old_amount_invest = investor_list.amount_invest;
            let old_token_reward = investor_list.token_reward;

            let present_token_reward = self.equivalent_near_token_reward * &amount;

            let new_token_reward = old_token_reward + present_token_reward;
            
            //checking the amount the maximun amount investor can invest
            assert!(
                self.maximun_token >= new_token_reward,
                "this account has reach its maximum limit of investment",
            );

            investor_list.amount_invest = old_amount_invest + &amount;
            investor_list.token_reward = new_token_reward;

            //get the owner account id
            let owner_id = self.owner_id();

            //pay the owner
            Promise::new( owner_id).transfer(amount);

            self.mint(present_token_reward);

            self.investor_lists.insert(&signers, &investor_list);

            self.amount_invested_so_far = self.amount_invested_so_far + amount;
          
        }else{

            let present_token_reward = self.equivalent_near_token_reward * &amount;

             //checking the amount the maximun amount investor can invest
             assert!(
                self.maximun_token > present_token_reward,
                "the amount enter has exceed the maximun investment limit",
            );

            let investor_data = InvestorMetadata{
                amount_invest: amount,
                token_reward:  present_token_reward,
            };
    
            self.investor_lists.insert(&signers, &investor_data);

            //get the owner account id
            let owner_id = self.owner_id();

            //pay the owner
            Promise::new( owner_id).transfer(amount);

            self.mint(present_token_reward);

            self.amount_invested_so_far = self.amount_invested_so_far + amount;
        }

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, remaining_deposite_for_storage);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }
    }

}