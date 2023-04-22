use crate::*;

impl Contract {

     // reture the owner account id
     pub fn owner_id(&self) -> AccountId {
        let acc = AccountId::from_str("akinyemisaheedwale2.testnet").unwrap();

        acc
    }

    // calculate for storage 

    pub fn pay_for_storage (&self, initial_storage: u64, attach_storage_cost: u128 ){

        // get the current storage 
        let current_storage = env::storage_usage();

        // get the storage used 
        let storage_used = current_storage - initial_storage;

        // get the storage cost per byts
        let storage_cost = env::storage_byte_cost();

        // get payable storage fees 
        if let Some(total_storage_cost) = storage_cost.checked_mul(storage_used as u128)  {

            // checker if user attach enough token to cater for storage 

            assert!(attach_storage_cost > total_storage_cost, "insufficient fund");

            //check for balance 
            let excess_balance = attach_storage_cost - total_storage_cost;

            if excess_balance > 0 {

                self.return_excess_token(excess_balance);
                
            }
            
        }


    }

    // return excess token back to signer

    pub fn return_excess_token(&self, excess_balance: u128){

        // get the addresss
        let signer = env::predecessor_account_id();

        //, Promise

        Promise::new(signer).transfer(excess_balance);
    }


    // calculate storage release
    pub fn refund_storage_cost(&self, initial_storage: u64) {

        // get the signer
        let _signer = env::predecessor_account_id();

        //get the current storage 
        let current_storage = env::storage_usage();

        //compute storage space release 
        let storage_release = initial_storage - current_storage;

        //get the storage unit price 
        let storage_unit_price = env::storage_byte_cost();

        //computing total refungible storage 
        if let Some(refundible_sorage_cost) = storage_unit_price.checked_mul(storage_release.into())  {
            
            //transfer to user wallet address 

            self.return_excess_token(refundible_sorage_cost);
        }else {
            panic!("Error in calculating storage cost");
        }
    }


    pub fn mint (&mut self, amount: u128){
        
        let sender = env::predecessor_account_id();

        if let Some(_user) = self.token.accounts.get(&sender) {

            self.token.internal_deposit(&sender, amount);

        }else{

            self.token.internal_register_account(&sender);

            self.token.internal_deposit(&sender, amount);
           
        }

    }

    pub fn mint_for_others (&mut self, amount: u128, account: AccountId){

        if let Some(_user) = self.token.accounts.get(&account) {

            self.token.internal_deposit(&account, amount);

        }else{

            self.token.internal_register_account(&account);

            self.token.internal_deposit(&account, amount);
     
        }

    }

    pub fn calculate_ico_time (&mut self,)-> bool{

        let current_time = env::block_timestamp();

        if self.ico_closing_date > current_time {
            
            true
        }else{
             
            false
        }
    }
}