#[cfg(test)]
mod mock {
    use candid::Principal;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<u64> = RefCell::new(0);
        static MOCK_CALLER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    }

    pub fn get_time() -> u64 {
        MOCK_TIME.with(|t| *t.borrow())
    }

    pub fn set_time(time: u64) {
        MOCK_TIME.with(|t| *t.borrow_mut() = time);
    }

    pub fn get_caller() -> Principal {
        MOCK_CALLER.with(|c| *c.borrow())
    }

    pub fn set_caller(caller: Principal) {
        MOCK_CALLER.with(|c| *c.borrow_mut() = caller);
    }
}

#[cfg(test)]
use mock::{get_time as time, get_caller as caller};

#[cfg(not(test))]
use ic_cdk::{api::time, caller};

use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, UserBalance>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Debug)]
pub enum LockPeriod {
    ThreeMonths,
    SixMonths,
    TwelveMonths,
}

impl LockPeriod {
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

    fn duration_nanos(&self) -> u64 {
        match self {
            LockPeriod::ThreeMonths => 90 * 24 * 60 * 60 * 1_000_000_000,  // 90 days
            LockPeriod::SixMonths => 180 * 24 * 60 * 60 * 1_000_000_000,   // 180 days
            LockPeriod::TwelveMonths => 365 * 24 * 60 * 60 * 1_000_000_000, // 365 days
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TokenDeposit {
    amount: u64,
    lock_period: LockPeriod,
    deposit_time: u64,
    interest_rate: f64,
    early_withdrawal_penalty: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserBalance {
    total_balance: u64,
    locked_balance: u64,
    available_balance: u64,
    deposits: Vec<TokenDeposit>,
    rewards_earned: u64,
}

const INR_TO_USD_RATE: f64 = 0.012; // 1 INR = 0.012 USD

#[derive(CandidType, Deserialize)]
pub enum MintResult {
    Ok(u64),
    Err(String),
}

#[ic_cdk::update]
pub fn mint_tokens(inr_amount: u64, lock_period: LockPeriod) -> MintResult {
    if inr_amount == 0 {
        return MintResult::Err("Amount must be greater than 0".to_string());
    }

    let caller = caller();
    let token_amount = (inr_amount as f64 * INR_TO_USD_RATE) as u64;
    
    let deposit = TokenDeposit {
        amount: token_amount,
        lock_period: lock_period.clone(),
        deposit_time: time(),
        interest_rate: lock_period.interest_rate(),
        early_withdrawal_penalty: lock_period.early_withdrawal_penalty(),
    };

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let user_balance = balances.entry(caller).or_insert_with(|| UserBalance {
            total_balance: 0,
            locked_balance: 0,
            available_balance: 0,
            deposits: Vec::new(),
            rewards_earned: 0,
        });

        user_balance.total_balance += token_amount;
        user_balance.locked_balance += token_amount;
        user_balance.deposits.push(deposit);
    });

    MintResult::Ok(token_amount)
}

#[ic_cdk::update]
pub fn burn_tokens(token_amount: u64) -> MintResult {
    if token_amount == 0 {
        return MintResult::Err("Amount must be greater than 0".to_string());
    }

    let caller = caller();
    
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        if let Some(user_balance) = balances.get_mut(&caller) {
            if user_balance.total_balance < token_amount {
                return MintResult::Err("Insufficient balance".to_string());
            }

            // Calculate early withdrawal penalty
            let mut total_penalty = 0;
            let mut remaining_amount = token_amount;
            
            for deposit in &mut user_balance.deposits {
                if remaining_amount == 0 {
                    break;
                }
                
                let time_elapsed = time() - deposit.deposit_time;
                let lock_duration = match deposit.lock_period {
                    LockPeriod::ThreeMonths => 3 * 30 * 24 * 60 * 60 * 1_000_000_000,
                    LockPeriod::SixMonths => 6 * 30 * 24 * 60 * 60 * 1_000_000_000,
                    LockPeriod::TwelveMonths => 12 * 30 * 24 * 60 * 60 * 1_000_000_000,
                };

                if time_elapsed < lock_duration {
                    let penalty = (deposit.amount as f64 * deposit.early_withdrawal_penalty) as u64;
                    total_penalty += penalty;
                }

                let amount_to_withdraw = std::cmp::min(remaining_amount, deposit.amount);
                deposit.amount -= amount_to_withdraw;
                remaining_amount -= amount_to_withdraw;
            }

            user_balance.total_balance -= token_amount;
            user_balance.locked_balance -= token_amount;
            user_balance.available_balance = user_balance.total_balance - user_balance.locked_balance;

            // Remove empty deposits
            user_balance.deposits.retain(|d| d.amount > 0);

            MintResult::Ok(token_amount - total_penalty)
        } else {
            MintResult::Err("User not found".to_string())
        }
    })
}

#[ic_cdk::query]
pub fn get_balance() -> Result<UserBalance, String> {
    let caller = caller();
    
    BALANCES.with(|balances| {
        balances
            .borrow()
            .get(&caller)
            .cloned()
            .ok_or("User balance not found".to_string())
    })
}

#[ic_cdk::update]
pub fn apply_rewards() -> Result<u64, String> {
    let caller = caller();
    let current_time = time();
    
    BALANCES.with(|balances| -> Result<u64, String> {
        let mut balances = balances.borrow_mut();
        let user_balance = balances.get_mut(&caller).ok_or("User balance not found")?;
        let mut total_rewards = 0;

        for deposit in &mut user_balance.deposits {
            let lock_end_time = deposit.deposit_time + deposit.lock_period.duration_nanos();
            if current_time >= lock_end_time {
                let time_elapsed = current_time - deposit.deposit_time;
                let years_elapsed = time_elapsed as f64 / (365.0 * 24.0 * 60.0 * 60.0 * 1_000_000_000.0);
                let rewards = (deposit.amount as f64 * deposit.interest_rate * years_elapsed) as u64;
                
                deposit.amount += rewards;
                total_rewards += rewards;
            }
        }

        user_balance.total_balance += total_rewards;
        user_balance.locked_balance += total_rewards;
        user_balance.rewards_earned += total_rewards;

        Ok(total_rewards)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock::{set_time, set_caller};

    const ONE_YEAR: u64 = 365 * 24 * 60 * 60 * 1_000_000_000;

    fn setup() {
        // Reset state before each test
        BALANCES.with(|balances| {
            balances.borrow_mut().clear();
        });
        set_time(0);
        set_caller(Principal::anonymous());
    }

    #[test]
    fn test_mint_tokens() {
        setup();
        
        // Test minting with different lock periods
        let test_cases = vec![
            (10000, LockPeriod::ThreeMonths),
            (20000, LockPeriod::SixMonths),
            (30000, LockPeriod::TwelveMonths),
        ];

        for (amount, lock_period) in test_cases {
            let result = mint_tokens(amount, lock_period.clone());
            assert!(result.is_ok());
            
            let token_amount = result.unwrap();
            let expected_tokens = (amount as f64 * INR_TO_USD_RATE) as u64;
            assert_eq!(token_amount, expected_tokens);

            // Verify balance was updated correctly
            let balance = get_balance().unwrap();
            assert_eq!(balance.total_balance, expected_tokens);
            assert_eq!(balance.locked_balance, expected_tokens);
            assert_eq!(balance.available_balance, 0);
            assert_eq!(balance.deposits.len(), 1);
            
            // Verify deposit details
            let deposit = &balance.deposits[0];
            assert_eq!(deposit.amount, expected_tokens);
            assert_eq!(deposit.lock_period, lock_period);
            assert_eq!(deposit.interest_rate, lock_period.interest_rate());
            assert_eq!(deposit.early_withdrawal_penalty, lock_period.early_withdrawal_penalty());

            setup(); // Reset for next test case
        }
    }

    #[test]
    fn test_burn_tokens() {
        setup();
        
        // Test cases for different lock periods and withdrawal scenarios
        let test_cases = vec![
            (LockPeriod::ThreeMonths, 10000, 5000), // Partial withdrawal
            (LockPeriod::SixMonths, 20000, 10000),  // Partial withdrawal
            (LockPeriod::TwelveMonths, 30000, 15000), // Partial withdrawal
        ];

        for (lock_period, mint_amount, burn_amount) in test_cases {
            // First mint tokens
            let mint_result = mint_tokens(mint_amount, lock_period.clone());
            assert!(mint_result.is_ok());
            let token_amount = mint_result.unwrap();
            let burn_token_amount = (burn_amount as f64 * INR_TO_USD_RATE) as u64;

            // Try to burn more than available
            let result = burn_tokens(token_amount + 1000);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Insufficient total balance");

            // Try early withdrawal (should incur penalty)
            let result = burn_tokens(burn_token_amount);
            assert!(result.is_ok());
            let burned_amount = result.unwrap();
            
            // Verify balance after withdrawal
            let balance = get_balance().unwrap();
            let expected_remaining = token_amount.saturating_sub(burned_amount + (burned_amount as f64 * lock_period.early_withdrawal_penalty()) as u64);
            assert_eq!(balance.total_balance, expected_remaining);
            assert_eq!(balance.locked_balance, expected_remaining);
            assert_eq!(balance.available_balance, 0);
            assert_eq!(balance.deposits.len(), 1);
            
            // Verify remaining deposit
            let deposit = &balance.deposits[0];
            assert_eq!(deposit.amount, token_amount - burned_amount);

            setup(); // Reset for next test case
        }
    }

    #[test]
    fn test_apply_rewards() {
        setup();
        
        // Test rewards for different lock periods
        let test_cases = vec![
            (10000, LockPeriod::ThreeMonths, 0.05),  // 5% APY
            (20000, LockPeriod::SixMonths, 0.07),    // 7% APY
            (30000, LockPeriod::TwelveMonths, 0.10), // 10% APY
        ];

        for (amount, lock_period, expected_rate) in test_cases {
            // Mint tokens
            let mint_result = mint_tokens(amount, lock_period.clone());
            assert!(mint_result.is_ok());
            let token_amount = mint_result.unwrap();

            // Advance time by one year
            set_time(ONE_YEAR);

            // Apply rewards after one year
            let result = apply_rewards();
            assert!(result.is_ok());
            let rewards = result.unwrap();

            // Calculate expected rewards (1 year)
            let expected_rewards = (token_amount as f64 * expected_rate) as u64;
            assert_eq!(rewards, expected_rewards);

            // Verify balance was updated
            let balance = get_balance().unwrap();
            assert_eq!(balance.total_balance, token_amount + rewards);
            assert_eq!(balance.locked_balance, token_amount + rewards);
            assert_eq!(balance.rewards_earned, rewards);
            
            // Verify deposit was updated
            let deposit = &balance.deposits[0];
            assert_eq!(deposit.amount, token_amount + rewards);

            setup(); // Reset for next test case
        }
    }

    #[test]
    fn test_get_balance() {
        setup();
        
        // Test initial balance
        let balance = get_balance();
        assert!(balance.is_err());
        assert_eq!(balance.unwrap_err(), "User balance not found");

        // Mint some tokens
        let mint_result = mint_tokens(10000, LockPeriod::ThreeMonths);
        assert!(mint_result.is_ok());
        let token_amount = mint_result.unwrap();

        // Check balance after minting
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, token_amount);
        assert_eq!(balance.locked_balance, token_amount);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.rewards_earned, 0);
        assert_eq!(balance.deposits.len(), 1);

        // Advance time by one year
        set_time(ONE_YEAR);

        // Apply rewards
        let reward_result = apply_rewards();
        assert!(reward_result.is_ok());
        let rewards = reward_result.unwrap();

        // Check balance after rewards
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, token_amount + rewards);
        assert_eq!(balance.locked_balance, token_amount + rewards);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.rewards_earned, rewards);
    }

    #[test]
    fn test_multiple_deposits() {
        setup();
        
        // Make multiple deposits with different lock periods
        let deposits = vec![
            (10000, LockPeriod::ThreeMonths),
            (20000, LockPeriod::SixMonths),
            (30000, LockPeriod::TwelveMonths),
        ];

        let mut total_tokens = 0;
        for (amount, lock_period) in deposits {
            let result = mint_tokens(amount, lock_period);
            assert!(result.is_ok());
            total_tokens += result.unwrap();
        }

        // Verify total balance
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, total_tokens);
        assert_eq!(balance.locked_balance, total_tokens);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.deposits.len(), 3);

        // Advance time by one year
        set_time(ONE_YEAR);

        // Apply rewards
        let reward_result = apply_rewards();
        assert!(reward_result.is_ok());
        let rewards = reward_result.unwrap();

        // Verify rewards were applied to all deposits
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, total_tokens + rewards);
        assert_eq!(balance.locked_balance, total_tokens + rewards);
        assert_eq!(balance.rewards_earned, rewards);
    }
}
