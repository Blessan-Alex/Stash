use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

// Constants
const INR_TO_USD_RATE: f64 = 0.012; // 1 INR = 0.012 USD

#[derive(CandidType, Deserialize, Clone, PartialEq)]
pub enum LockPeriod {
    ThreeMonths,
    SixMonths,
    TwelveMonths,
}

impl LockPeriod {
    fn to_nanoseconds(&self) -> u64 {
        match self {
            LockPeriod::ThreeMonths => 3 * 30 * 24 * 60 * 60 * 1_000_000_000,
            LockPeriod::SixMonths => 6 * 30 * 24 * 60 * 60 * 1_000_000_000,
            LockPeriod::TwelveMonths => 12 * 30 * 24 * 60 * 60 * 1_000_000_000,
        }
    }

    fn interest_rate(&self) -> f64 {
        match self {
            LockPeriod::ThreeMonths => 0.05,  // 5% APY
            LockPeriod::SixMonths => 0.07,    // 7% APY
            LockPeriod::TwelveMonths => 0.10, // 10% APY
        }
    }

    fn early_withdrawal_penalty(&self) -> f64 {
        match self {
            LockPeriod::ThreeMonths => 0.02,  // 2% penalty
            LockPeriod::SixMonths => 0.05,    // 5% penalty
            LockPeriod::TwelveMonths => 0.10, // 10% penalty
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TokenDeposit {
    amount: u64,
    lock_period: LockPeriod,
    deposit_time: u64,
    interest_rate: f64,
    early_withdrawal_penalty: f64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserBalance {
    total_balance: u64,
    locked_balance: u64,
    available_balance: u64,
    deposits: Vec<TokenDeposit>,
    rewards_earned: u64,
}

// State
thread_local! {
    static BALANCES: std::cell::RefCell<HashMap<Principal, UserBalance>> = std::cell::RefCell::new(HashMap::new());
}

#[init]
fn init() {
    // Initialize canister
}

// Token methods
#[update]
fn mint_tokens(inr_amount: u64, lock_period: LockPeriod) -> Result<u64, String> {
    let caller = ic_cdk::caller();
    
    // Convert INR to USD (token amount)
    let token_amount = (inr_amount as f64 * INR_TO_USD_RATE) as u64;
    
    // Create deposit record
    let deposit = TokenDeposit {
        amount: token_amount,
        lock_period: lock_period.clone(),
        deposit_time: ic_cdk::api::time(),
        interest_rate: lock_period.interest_rate(),
        early_withdrawal_penalty: lock_period.early_withdrawal_penalty(),
    };

    // Update user balance
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(caller).or_insert_with(|| UserBalance {
            total_balance: 0,
            locked_balance: 0,
            available_balance: 0,
            deposits: Vec::new(),
            rewards_earned: 0,
        });

        balance.total_balance += token_amount;
        balance.locked_balance += token_amount;
        balance.deposits.push(deposit);
        
        Ok(token_amount)
    })
}

#[update]
fn burn_tokens(token_amount: u64) -> Result<u64, String> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();
    
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.get_mut(&caller)
            .ok_or_else(|| "User balance not found".to_string())?;

        if balance.available_balance < token_amount {
            return Err("Insufficient available balance".to_string());
        }

        // Find deposits that can be withdrawn
        let mut remaining_amount = token_amount;
        let mut deposits_to_remove = Vec::new();
        
        for (index, deposit) in balance.deposits.iter_mut().enumerate() {
            let lock_end_time = deposit.deposit_time + deposit.lock_period.to_nanoseconds();
            
            if current_time >= lock_end_time {
                // Lock period has ended, can withdraw without penalty
                if remaining_amount <= deposit.amount {
                    deposits_to_remove.push(index);
                    remaining_amount = 0;
                    break;
                } else {
                    remaining_amount -= deposit.amount;
                    deposits_to_remove.push(index);
                }
            } else {
                // Early withdrawal with penalty
                let penalty = (deposit.amount as f64 * deposit.early_withdrawal_penalty()) as u64;
                let available = deposit.amount - penalty;
                
                if remaining_amount <= available {
                    deposit.amount -= remaining_amount;
                    remaining_amount = 0;
                    break;
                } else {
                    remaining_amount -= available;
                    deposits_to_remove.push(index);
                }
            }
        }

        if remaining_amount > 0 {
            return Err("Insufficient available balance after penalties".to_string());
        }

        // Remove processed deposits
        for index in deposits_to_remove.iter().rev() {
            balance.deposits.remove(*index);
        }

        balance.total_balance -= token_amount;
        balance.available_balance -= token_amount;
        
        Ok(token_amount)
    })
}

#[query]
fn get_balance() -> Result<UserBalance, String> {
    let caller = ic_cdk::caller();
    
    BALANCES.with(|balances| {
        balances.borrow()
            .get(&caller)
            .cloned()
            .ok_or_else(|| "User balance not found".to_string())
    })
}

#[update]
fn apply_rewards() -> Result<u64, String> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();
    
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.get_mut(&caller)
            .ok_or_else(|| "User balance not found".to_string())?;

        let mut total_rewards = 0u64;

        // Calculate interest rewards for each deposit
        for deposit in &mut balance.deposits {
            let time_diff = current_time - deposit.deposit_time;
            let years = time_diff as f64 / (365.0 * 24.0 * 60.0 * 60.0 * 1_000_000_000.0);
            let reward = (deposit.amount as f64 * deposit.interest_rate * years) as u64;
            deposit.amount += reward;
            total_rewards += reward;
        }

        balance.total_balance += total_rewards;
        balance.locked_balance += total_rewards;
        balance.rewards_earned += total_rewards;
        
        Ok(total_rewards)
    })
} 