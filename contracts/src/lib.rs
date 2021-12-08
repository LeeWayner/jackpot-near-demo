use borsh::{ BorshDeserialize, BorshSerialize };
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PublicKey, Promise,
    collections::{ UnorderedMap },
    json_types::{ U128, Base58PublicKey },
};
use serde::Serialize;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const PROB:u8 = 128;

#[near_bindgen]

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Jackpot {
    pub owner_id: AccountId,
    pub balance: Balance,
    pub playCount : u8
}

// impl Default for Jackpot {
//     fn default() -> Self {
//         panic!("Should be initialized before usage")
//     }
// }

#[near_bindgen]
impl Jackpot {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            balance: 0,
            playCount : 0
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        self.balance = self.balance +  deposit;
        // let mut credits = self.credits.get(&account_id).unwrap_or(0);
        // credits = credits + deposit;
        // self.credits.insert(&account_id, &credits);
    }
    
    #[payable]
    pub fn play(&mut self) -> u8 {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();

        self.playCount= self.playCount + 1;
        let mut win = 0;
        self.balance = self.balance +  deposit;
        if self.playCount > 5
        {
            self.playCount = 0;
            Promise::new(account_id).transfer(self.balance);
            self.balance = 0;
            win = 1;
        }
        win
    }

    pub fn get_balance(&self) -> U128 {
        self.balance.into()
    }
}


