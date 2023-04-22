#!/bin/bash


###########first default
#near call ico.akinyemisaheedwale2.testnet new_default_meta '{"owner_id": "akinyemisaheedwale2.testnet", "total_supply": "30"}' --accountId akinyemisaheedwale2.testnet 

########view account balance
#near view ico.akinyemisaheedwale2.testnet ft_balance_of '{"account_id": "akinyemisaheedwale2.testnet"}'

########### call change equivalent token reward
#near call ico.akinyemisaheedwale2.testnet change_equivalent_near_token_reward '{"amount": 200}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

########## view change equivalent token reward
#near view ico.akinyemisaheedwale2.testnet query_equivalent_token_reward 

########### call change minimun amount to invest
#near call ico.akinyemisaheedwale2.testnet change_minimun_amount_to_invest '{"amount": 1000}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

############ view change minimun amount to invest
#near view ico.akinyemisaheedwale2.testnet query_mininum_amount_to_invest

########### call change maximun token an investor call invest
#near call ico.akinyemisaheedwale2.testnet change_maximun_token '{"amount": 9000000000000000000}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

############ view change maximun token an investor call invest
#near view ico.akinyemisaheedwale2.testnet query_maximum_token_ivestor_can_buy

########### call add member to list
#near call ico.akinyemisaheedwale2.testnet add_member_to_role '{"member_id": "akinyemisaheedwale4.testnet", "role": "secretary"}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

############ view member detail
#near view ico.akinyemisaheedwale2.testnet member_details '{"account": "akinyemisaheedwale4.testnet"}'

########### call mint token
#near call ico.akinyemisaheedwale2.testnet mint_token '{"amount": 600}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

########### call mint token  for other member
#near call ico.akinyemisaheedwale2.testnet mint_token_for_others '{"amount": 8000, "account": "akinyemisaheedwale4.testnet"}' --accountId akinyemisaheedwale2.testnet  --amount 0.006


########### call change ico closing date
#near call ico.akinyemisaheedwale2.testnet change_ico_closing_date '{"duration": 9}' --accountId akinyemisaheedwale2.testnet  --amount 0.006

############ view ico closing date
#near view ico.akinyemisaheedwale2.testnet query_ico_closing_date 

############ view timeStap
#near view ico.akinyemisaheedwale2.testnet query_timeStamp

########### call change add expected money by the company for ico
#near call ico.akinyemisaheedwale2.testnet add_expected_from_all_investor '{"amount": 98000000000000000000}' --accountId akinyemisaheedwale2.testnet  

############ view ico expectec money
#near view ico.akinyemisaheedwale2.testnet query_expected_money

############ view amount invested so far
near view ico.akinyemisaheedwale2.testnet query_amoun_invested_so_far



############## invest
near call ico.akinyemisaheedwale2.testnet invest '{"amount": 2000}' --accountId akinyemisaheedwale3.testnet  --amount 0.04

############ view investor detail
near view ico.akinyemisaheedwale2.testnet investor_details '{"account": "akinyemisaheedwale3.testnet"}'