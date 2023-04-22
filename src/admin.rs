use crate::*;

pub trait AdminCore {

    // to change near equivalent token reward
    fn change_equivalent_near_token_reward
    (
        &mut self,
        amount: u128, 
    );

    //to change minimun amount in vest
    fn change_minimun_amount_to_invest
    (
        &mut self,
        amount: u128, 
    );

    //to change maximun token
    fn change_maximun_token
    (
        &mut self,
        amount: u128, 
    );

    //to add membet to role
    fn add_member_to_role
    (
        &mut self,
        member_type: AccountId,
        role: String,
    );

    //to mint token
    fn mint_token
    (
        &mut self, 
        amount: u128, 
    )->u128;

    //to mint token for others
    fn mint_token_for_others
    (
        &mut self, 
        amount: u128, 
        account: AccountId,
    )->u128;

    //to change ico closing date
    fn change_ico_closing_date
    (
        &mut self, 
        duration: u64, 
    );

    fn add_expected_from_all_investor(
        &mut self,
        amount: u128,
    );
    
}

#[near_bindgen]
impl AdminCore for Contract {

    #[payable]
    fn change_equivalent_near_token_reward(&mut self, amount: u128, ){

        //get the admin account id
        let admin = self.owner_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        //get the signer account id
        let signer = env::predecessor_account_id();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );
        

        self.equivalent_near_token_reward = amount;

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

    }

    #[payable]
    fn change_minimun_amount_to_invest(&mut self, amount: u128, ){

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        self.minimun_amount_invest = amount;

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

    }

    #[payable]
    fn change_maximun_token(&mut self, amount: u128, ){

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        self.maximun_token = amount;

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

    }

    #[payable]
    fn add_member_to_role(&mut self, member_id: AccountId, role: String,){

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        let memberdata = MemberMetadata{
            role: role,
            token_reward_status: 0,
            amount_of_token_rewarded: 0,
        };

        self.member_lists.insert(&member_id, &memberdata);

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

    }

    #[payable]
    fn mint_token(&mut self, amount: u128)-> u128{

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        // //checking if the ico date has not pass
        assert!(
            self.calculate_ico_time() == false,
            "the crow sale time has not finised",
        );

        self.mint(amount);

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

        amount
    }

    #[payable]
    fn mint_token_for_others(&mut self, amount: u128, account: AccountId,)->u128{

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        let mut memberdata = self.member_lists.get(&account).unwrap(); 

        //check if particular has token minted already
        assert_eq!(
            memberdata.token_reward_status, 0,
            "this account {} already has token minted",
            account,
        );

        //checking if the ico date has not pass
        assert!(
            self.calculate_ico_time() == false,
            "the crow sale time has finised",
        );

        memberdata.token_reward_status = 1;
        memberdata.amount_of_token_rewarded = amount;

        self.member_lists.insert(&account, &memberdata);

        self.mint_for_others(amount, account);

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }

        amount

    }

    #[payable]
    fn change_ico_closing_date(&mut self, duration: u64, ){

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        let one_day: u64 = 86400 * 1000000000;

        let next_closing_day = one_day * duration;

        let closing_date = env::block_timestamp() + next_closing_day;
        
        self.ico_closing_date = closing_date;

        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }
        
    }

    fn add_expected_from_all_investor(&mut self, amount: u128,){

        //get the admin account id
        let admin = self.owner_id();

        //get the signer account id
        let signer = env::predecessor_account_id();

        assert_eq!(
            admin, signer,
            "you are not the admin please",
        );

        //checking that amount must greater than zero
        assert!(
            amount > 0,
            "the amount must be greater than zero",
        );

        self.expected_money = amount;
    }

}