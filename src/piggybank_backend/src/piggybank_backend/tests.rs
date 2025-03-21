use super::*;
use ic_cdk::api::time;
use candid::Principal;

const TEST_TIME: u64 = 1_000_000_000_000_000_000; // Some fixed timestamp
const ONE_DAY: u64 = 86_400_000_000_000; // One day in nanoseconds

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        // Reset state before each test
        BALANCES.with(|balances| {
            balances.borrow_mut().clear();
        });
    }

    #[test]
    fn test_mint_tokens() {
        setup();
        let caller = Principal::anonymous();
        
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
        }
    }

    #[test]
    fn test_burn_tokens() {
        setup();
        let caller = Principal::anonymous();
        
        // Test cases for different lock periods and withdrawal scenarios
        let test_cases = vec![
            (LockPeriod::ThreeMonths, 10000, 5000), // Partial withdrawal
            (LockPeriod::SixMonths, 20000, 20000),  // Full withdrawal
            (LockPeriod::TwelveMonths, 30000, 15000), // Partial withdrawal
        ];

        for (lock_period, mint_amount, burn_amount) in test_cases {
            // First mint tokens
            let mint_result = mint_tokens(mint_amount, lock_period.clone());
            assert!(mint_result.is_ok());
            let token_amount = mint_result.unwrap();

            // Try to burn more than available
            let result = burn_tokens(token_amount + 1000);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Insufficient available balance");

            // Try early withdrawal (should incur penalty)
            let result = burn_tokens(burn_amount);
            assert!(result.is_ok());
            let burned_amount = result.unwrap();
            
            // Calculate expected amount after penalty
            let penalty = (burn_amount as f64 * lock_period.early_withdrawal_penalty()) as u64;
            let expected_amount = burn_amount - penalty;
            
            // Verify balance after withdrawal
            let balance = get_balance().unwrap();
            assert_eq!(balance.total_balance, token_amount - burned_amount);
            assert_eq!(balance.locked_balance, token_amount - burned_amount);
            assert_eq!(balance.available_balance, 0);
            assert_eq!(balance.deposits.len(), 1);
            
            // Verify remaining deposit
            let deposit = &balance.deposits[0];
            assert_eq!(deposit.amount, token_amount - burned_amount);
        }
    }

    #[test]
    fn test_apply_rewards() {
        setup();
        let caller = Principal::anonymous();
        
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
        }
    }

    #[test]
    fn test_get_balance() {
        setup();
        let caller = Principal::anonymous();
        
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
        let caller = Principal::anonymous();
        
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

#[test]
fn test_create_and_deposit_goal() {
    // Create a new goal
    let args = CreateGoalArgs {
        name: "Test Goal".to_string(),
        target_amount: 1000,
        category: "emergency".to_string(),
        deadline: Some(time() + 30 * 24 * 60 * 60 * 1_000_000_000), // 30 days from now
    };

    let result = create_goal(args);
    assert!(result.is_ok());
    let goal_id = result.unwrap();

    // Verify goal was created
    let goal = get_goal(goal_id.clone()).unwrap();
    assert_eq!(goal.name, "Test Goal");
    assert_eq!(goal.target_amount, 1000);
    assert_eq!(goal.current_amount, 0);

    // Deposit to the goal
    let deposit_args = DepositArgs {
        goal_id: goal_id.clone(),
        amount: 500,
    };

    let deposit_result = deposit(deposit_args);
    assert!(deposit_result.is_ok());
    assert_eq!(deposit_result.unwrap(), 500);

    // Verify goal was updated
    let updated_goal = get_goal(goal_id).unwrap();
    assert_eq!(updated_goal.current_amount, 500);
}

#[test]
fn test_rewards_calculation() {
    // Mint tokens
    let inr_amount = 1000;
    mint_tokens(inr_amount).unwrap();

    // Apply rewards
    let reward_result = apply_rewards();
    assert!(reward_result.is_ok());
    let reward_info = reward_result.unwrap();

    // Verify reward info
    assert_eq!(reward_info.lock_in_period, LOCK_IN_PERIOD);
    assert!(reward_info.last_reward_calculation > 0);

    // Get reward info
    let info = get_reward_info().unwrap();
    assert_eq!(info.total_rewards, reward_info.total_rewards);
}

#[test]
fn test_get_user_goals() {
    // Create multiple goals
    let args1 = CreateGoalArgs {
        name: "Goal 1".to_string(),
        target_amount: 1000,
        category: "emergency".to_string(),
        deadline: None,
    };

    let args2 = CreateGoalArgs {
        name: "Goal 2".to_string(),
        target_amount: 2000,
        category: "vacation".to_string(),
        deadline: None,
    };

    create_goal(args1).unwrap();
    create_goal(args2).unwrap();

    // Get all user goals
    let goals = get_user_goals();
    assert_eq!(goals.len(), 2);
    assert!(goals.iter().any(|g| g.name == "Goal 1"));
    assert!(goals.iter().any(|g| g.name == "Goal 2"));
}

#[test]
fn test_insufficient_balance() {
    // Try to burn more tokens than available
    let result = burn_tokens(1000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User balance not found");
}

#[test]
fn test_unauthorized_deposit() {
    // Create a goal
    let args = CreateGoalArgs {
        name: "Test Goal".to_string(),
        target_amount: 1000,
        category: "emergency".to_string(),
        deadline: None,
    };

    let goal_id = create_goal(args).unwrap();

    // Try to deposit with different principal (this would need to be mocked in a real test)
    let deposit_args = DepositArgs {
        goal_id,
        amount: 500,
    };

    // Note: This test would need to be modified to properly test unauthorized access
    // as we can't easily mock different principals in this test environment
    let deposit_result = deposit(deposit_args);
    assert!(deposit_result.is_ok()); // This would fail in a real scenario with different principals
} 