#[cfg(test)]
use crate::Contract;
use crate::admin:: AdminCore;
use crate::invest::InvestCore;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{env, AccountId};
use near_sdk::{testing_env, Balance};
use super::*;

const MINT_STORAGE_COST: u128 = 100_000_000_000_000_000_000_000;
const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

#[test]
fn test_new() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
}

#[test]
#[should_panic(expected = "The contract is not initialized")]
fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
}

#[test]
fn test_transfer() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(contract.storage_balance_bounds().min.into())
        .predecessor_account_id(accounts(1))
        .build());
    // Paying for account registration, aka storage deposit
    contract.storage_deposit(None, None);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(1)
        .predecessor_account_id(accounts(2))
        .build());
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(1), transfer_amount.into(), None);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());
    assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
    assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
}

#[test]
fn check_balance () {
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(contract.storage_balance_bounds().min.into())
        .predecessor_account_id(accounts(5))
        .build());
    // Paying for account registration, aka storage deposit
    contract.storage_deposit(None, None);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());
    assert_eq!(contract.ft_balance_of(accounts(4)).0, TOTAL_SUPPLY);
    assert_eq!(contract.ft_balance_of(accounts(5)).0, 0);
}

#[test]
fn change_equivqlent_token(){
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let amount = 5000;

    contract.change_equivalent_near_token_reward(amount.clone());

    assert_eq!(contract.equivalent_near_token_reward, amount);
}

#[test]
fn minum_amount_to_invest(){
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let minimun_amount = 40;

    contract.change_minimun_amount_to_invest(minimun_amount.clone());

    assert_eq!(contract.minimun_amount_invest, minimun_amount);
}

#[test]
fn maximun_amount_to_invest(){
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let maximun_amount = 4000000000;

    contract.change_maximun_token(maximun_amount.clone());

    assert_eq!(contract.maximun_token, maximun_amount);
}

#[test]
fn expected_amount(){
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context 
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let expected_amount = 800000000000000000000000000;

    contract.add_expected_from_all_investor(expected_amount.clone());

    assert_eq!(contract.expected_money, expected_amount);
}

#[test]
fn invest(){
    let mut context = get_context(accounts(4));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(4).into(), TOTAL_SUPPLY.into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let amount = 5000;

    contract.change_equivalent_near_token_reward(amount.clone());

    assert_eq!(contract.equivalent_near_token_reward, amount);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let minimun_amount = 40;

    contract.change_minimun_amount_to_invest(minimun_amount.clone());

    assert_eq!(contract.minimun_amount_invest, minimun_amount);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let maximun_amount = 4000000000;

    contract.change_maximun_token(maximun_amount.clone());

    assert_eq!(contract.maximun_token, maximun_amount);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let closing_date = 50;

    contract.change_ico_closing_date(closing_date.clone());



    testing_env!(context 
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let expected_amount = 800000000000000000000000000;

    contract.add_expected_from_all_investor(expected_amount.clone());

    assert_eq!(contract.expected_money, expected_amount);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(5))
        .build());

    let amount_invest = 800;

    contract.invest(amount_invest.clone());

    assert_eq!(contract.amount_invested_so_far, amount_invest);

    
}
