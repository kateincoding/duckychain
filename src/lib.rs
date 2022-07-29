use near_sdk::borsh::{self,BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    phone: u32,
    address: String,
    website: String
}

#[derive(Copy, Clone, BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Restaurant,
    Agriculture,
    Business,
    Education
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Contract {
    phone_book: LookupMap<Category, HashMap<String, Entry> >
}

impl Default for Contract {
    fn default() ->Self {
        let mut contract = Self { phone_book: LookupMap::new(b"a") };
        contract.phone_book.insert(&Category::Restaurant, &HashMap::new());
        contract.phone_book.insert(&Category::Agriculture, &HashMap::new());
        contract.phone_book.insert(&Category::Business, &HashMap::new());
        contract.phone_book.insert(&Category::Education, &HashMap::new());
        contract
    }
}

fn get_category(category: String) ->Category {
    if category == "Restaurant" {
        Category::Restaurant
    }
    else if category == "Agriculture" {
        Category::Agriculture
    }
    else if category == "Business" {
        Category::Business
    }
    else if category == "Education" {
        Category::Education
    }
    else {
        env::panic_str("Category does not exist");
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() ->Self {
        let mut contract = Self { phone_book: LookupMap::new(b"a") };
        contract.phone_book.insert(&Category::Restaurant, &HashMap::new());
        contract.phone_book.insert(&Category::Agriculture, &HashMap::new());
        contract.phone_book.insert(&Category::Business, &HashMap::new());
        contract.phone_book.insert(&Category::Education, &HashMap::new());
        contract
    }

    pub fn add_entry(&mut self, category: String, phone: u32, name: String, address: String, website: String) {
        let cat = get_category(category.clone());
        if self.phone_book.get(&cat.clone()) == None {
            env::panic_str("Category error");
        }
        let mut entry = self.phone_book.get(&cat.clone()).unwrap();
        entry.insert(name.clone(), Entry{phone: phone.clone(), address: address.clone(), website: website.clone()});
        self.phone_book.insert(&cat, &entry);
    }

    pub fn get_entry(&self, category: String, name: String) ->Entry {
        let cat = get_category(category.clone());
        if self.phone_book.get(&cat.clone()) == None {
            env::panic_str("No such category");
        }
        let tmp = self.phone_book.get(&cat.clone()).unwrap();
        if tmp.get(&name.clone()) == None {
            env::panic_str("No such entry");
        }
        tmp.get(&name.clone()).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    fn get_context(wallet: &str, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "jane.testnet".parse().unwrap(),
            signer_account_id: wallet.parse().unwrap(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob.testnet".parse().unwrap(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_one() {
        let context = get_context("test.testnet", 0);
        let mut contract = Contract::new();
        testing_env!(context.clone());
        contract.add_entry(
            "Restaurant".to_string(),
            1234567,
            "pizza hut".to_string(),
            "lima, Peru".to_string(),
            "pizzahut.com".to_string()
        );
        let entry = contract.get_entry("Restaurant".to_string(), "pizza hut".to_string());
        println!("Entry = {:?}", entry);
    }
}