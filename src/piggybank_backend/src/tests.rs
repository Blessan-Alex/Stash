use super::*;
use candid::Principal;

const TEST_TIME: u64 = 1_000_000_000_000_000_000; // Some fixed timestamp
const ONE_DAY: u64 = 86_400_000_000_000;

// Mock environment for testing
#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;
    
    thread_local! {
        static MOCK_CALLER: RefCell<Principal> = RefCell::new(Principal::anonymous());
        static MOCK_TIME: RefCell<u64> = RefCell::new(TEST_TIME);
    }

    pub fn caller() -> Principal {
        MOCK_CALLER.with(|c| *c.borrow())
    }

    pub fn time() -> u64 {
        MOCK_TIME.with(|t| *t.borrow())
    }

    pub fn set_caller(caller: Principal) {
        MOCK_CALLER.with(|c| {
            *c.borrow_mut() = caller;
        });
    }

    pub fn set_time(time: u64) {
        MOCK_TIME.with(|t| {
            *t.borrow_mut() = time;
        });
    }

    pub fn advance_time(duration: u64) {
        MOCK_TIME.with(|t| {
            *t.borrow_mut() += duration;
        });
    }
}

#[cfg(test)]
use mock::*;

fn setup() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.savings_goals.clear();
        state.users.clear();
        state.username_to_id.clear();
        state.transactions.clear();
        state.notifications.clear();
        state.notified_deadlines.clear();
        state.user_balances.clear();
        state.next_goal_id = 0;
        state.next_transaction_id = 0;
        state.next_notification_id = 0;
    });
    
    // Reset mock environment
    set_caller(Principal::anonymous());
    set_time(TEST_TIME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_management() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);

        // Test user registration
        let user = register_user("testuser".to_string(), "test@example.com".to_string()).unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.id, test_principal);

        // Test duplicate registration
        let result = register_user("testuser".to_string(), "another@example.com".to_string());
        assert!(result.is_err());

        // Test user retrieval
        let retrieved = get_user(user.id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().username, "testuser");

        // Test non-existent user
        let retrieved = get_user(Principal::anonymous());
        assert!(retrieved.is_none());

        // Test user update
        let result = update_user("new@example.com".to_string());
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.email, "new@example.com");

        // Test update non-existent user
        set_caller(Principal::anonymous());
        let result = update_user("new@example.com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_savings_goals() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);

        // Register a user
        let _user = register_user("testuser".to_string(), "test@example.com".to_string()).unwrap();

        // Test goal creation
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal = create_savings_goal(goal_args).unwrap();
        assert_eq!(goal.name, "Test Goal");
        assert_eq!(goal.target_amount, 1000);
        assert_eq!(goal.current_amount, 0);
        assert_eq!(goal.status, GoalStatus::Active);
        assert_eq!(goal.user_id, test_principal);

        // Test goal with past deadline
        let goal_args = CreateGoalArgs {
            name: "Past Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME - ONE_DAY),
        };
        let result = create_savings_goal(goal_args);
        assert!(result.is_err());

        // Test goal creation for non-existent user
        set_caller(Principal::anonymous());
        let goal_args = CreateGoalArgs {
            name: "Invalid Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let result = create_savings_goal(goal_args);
        assert!(result.is_err());

        // Test goal retrieval
        set_caller(test_principal);
        let retrieved = get_savings_goal(goal.id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Goal");

        // Test non-existent goal
        let retrieved = get_savings_goal(999);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_transactions() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);

        // Register a user and create a goal
        let _user = register_user("testuser".to_string(), "test@example.com".to_string()).unwrap();
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal = create_savings_goal(goal_args).unwrap();

        // Test adding transactions
        let result = add_transaction(
            goal.id,
            500,
            TransactionType::Contribution,
            Some("Initial contribution".to_string())
        );
        assert!(result.is_ok());

        let result = add_transaction(
            goal.id,
            300,
            TransactionType::Contribution,
            Some("Additional contribution".to_string())
        );
        assert!(result.is_ok());

        // Test goal amount update
        let updated_goal = get_savings_goal(goal.id).unwrap();
        assert_eq!(updated_goal.current_amount, 800);

        // Test withdrawal
        let result = add_transaction(
            goal.id,
            200,
            TransactionType::Withdrawal,
            Some("Emergency withdrawal".to_string())
        );
        assert!(result.is_ok());

        // Test goal amount after withdrawal
        let updated_goal = get_savings_goal(goal.id).unwrap();
        assert_eq!(updated_goal.current_amount, 600);

        // Test invalid withdrawal amount
        let result = add_transaction(
            goal.id,
            1000,
            TransactionType::Withdrawal,
            Some("Invalid withdrawal".to_string())
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_retrieval() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);

        // Register a user and create a goal
        let _user = register_user("testuser".to_string(), "test@example.com".to_string()).unwrap();
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal = create_savings_goal(goal_args).unwrap();

        // Add a transaction
        let transaction = add_transaction(
            goal.id,
            500,
            TransactionType::Contribution,
            Some("Test transaction".to_string())
        ).unwrap();

        // Test transaction retrieval
        let retrieved = get_transaction(transaction.id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().amount, 500);

        // Test non-existent transaction
        let retrieved = get_transaction(999);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_transaction_listing() {
        setup();
        let test_principal1 = Principal::from_slice(&[1, 2, 3, 4]);
        let test_principal2 = Principal::from_slice(&[5, 6, 7, 8]);

        // Register two users and create goals
        set_caller(test_principal1);
        let _user1 = register_user("user1".to_string(), "user1@example.com".to_string()).unwrap();
        
        set_caller(test_principal2);
        let _user2 = register_user("user2".to_string(), "user2@example.com".to_string()).unwrap();

        set_caller(test_principal1);
        let goal_args1 = CreateGoalArgs {
            name: "User1 Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal1 = create_savings_goal(goal_args1).unwrap();

        set_caller(test_principal2);
        let goal_args2 = CreateGoalArgs {
            name: "User2 Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal2 = create_savings_goal(goal_args2).unwrap();

        // Add transactions for both users
        set_caller(test_principal1);
        add_transaction(goal1.id, 500, TransactionType::Contribution, None).unwrap();
        
        set_caller(test_principal2);
        add_transaction(goal2.id, 300, TransactionType::Contribution, None).unwrap();

        // Test listing all transactions
        let all_transactions = list_transactions(None);
        assert_eq!(all_transactions.len(), 2);

        // Test listing transactions for specific goal
        let goal1_transactions = list_transactions(Some(goal1.id));
        assert_eq!(goal1_transactions.len(), 1);
        assert_eq!(goal1_transactions[0].goal_id, goal1.id);
    }

    #[test]
    fn test_transaction_summary() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);

        // Register a user and create a goal
        let _user = register_user("testuser".to_string(), "test@example.com".to_string()).unwrap();
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + ONE_DAY),
        };
        let goal = create_savings_goal(goal_args).unwrap();

        // Add various transactions
        add_transaction(goal.id, 500, TransactionType::Contribution, None).unwrap();
        add_transaction(goal.id, 200, TransactionType::Withdrawal, None).unwrap();
        add_transaction(goal.id, 300, TransactionType::Contribution, None).unwrap();

        // Test transaction summary
        let result = get_transaction_summary(goal.id);
        assert!(result.is_ok());
        let (total_contributions, total_withdrawals) = result.unwrap();
        assert_eq!(total_contributions, 800);
        assert_eq!(total_withdrawals, 200);

        // Test summary for non-existent goal
        let result = get_transaction_summary(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_notifications() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);
        
        // Create a goal
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 10000,
            category: "Savings".to_string(),
            deadline: None,
        };
        let goal_id = create_savings_goal(goal_args).unwrap().id;

        // Add contribution to reach 75% milestone
        let deposit_args = DepositArgs {
            goal_id,
            amount: 7500, // 75% milestone
        };
        deposit(deposit_args).unwrap();

        // Check notifications
        let notifications = get_notifications(true);
        assert_eq!(notifications.len(), 1);
        let milestone_notification = notifications[0].clone();
        assert_eq!(milestone_notification.notification_type, NotificationType::MilestoneReached);

        // Complete the goal
        let deposit_args = DepositArgs {
            goal_id,
            amount: 2500,
        };
        deposit(deposit_args).unwrap();

        // Check notifications again
        let notifications = get_notifications(true);
        assert_eq!(notifications.len(), 2);
        let completion_notification = notifications.iter()
            .find(|n| n.notification_type == NotificationType::GoalCompleted)
            .unwrap()
            .clone();

        // Mark notification as read
        let result = mark_notification_as_read(milestone_notification.id);
        assert!(result.is_ok());

        // Check unread notifications
        let unread = get_notifications(false);
        assert_eq!(unread.len(), 1);
        assert_eq!(unread[0].id, completion_notification.id);

        // Try to mark another user's notification as read
        let other_principal = Principal::from_slice(&[9, 8, 7, 6]);
        set_caller(other_principal);
        let result = mark_notification_as_read(milestone_notification.id);
        assert!(result.is_err());
    }

    #[test]
    fn test_deadline_notifications() {
        setup();
        let test_principal = Principal::from_slice(&[1, 2, 3, 4]);
        set_caller(test_principal);
        
        let two_days = 2 * ONE_DAY;

        // Create a goal with deadline in 2 days
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 1000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + two_days),
        };
        let goal_id = create_savings_goal(goal_args).unwrap().id;

        // Check deadlines (should create notification)
        check_deadlines();

        // Check notifications
        let notifications = get_notifications(true);
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].notification_type, NotificationType::DeadlineApproaching);

        // Check deadlines again (should not create duplicate notification)
        check_deadlines();
        let notifications = get_notifications(true);
        assert_eq!(notifications.len(), 1);

        // Create a completed goal
        let goal_args = CreateGoalArgs {
            name: "Completed Goal".to_string(),
            target_amount: 500,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + two_days),
        };
        let completed_goal_id = create_savings_goal(goal_args).unwrap().id;

        // Add transaction to complete the goal
        let deposit_args = DepositArgs {
            goal_id: completed_goal_id,
            amount: 500,
        };
        deposit(deposit_args).unwrap();

        // Check deadlines (should not create notification for completed goal)
        check_deadlines();
        let notifications = get_notifications(true);
        assert_eq!(notifications.len(), 1); // Still only one notification
    }

    #[test]
    fn test_token_minting() {
        setup();
        let _caller = Principal::anonymous();
        
        // Test minting with different lock periods
        let result = mint_tokens(10000, LockPeriod::ThreeMonths);
        assert!(result.is_ok());
        let token_amount = result.unwrap();
        assert_eq!(token_amount, (10000 as f64 * INR_TO_USD_RATE) as u64);

        // Check balance
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, token_amount);
        assert_eq!(balance.locked_balance, token_amount);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.deposits.len(), 1);

        // Test minting with 12-month lock period
        let result = mint_tokens(20000, LockPeriod::TwelveMonths);
        assert!(result.is_ok());
        let token_amount2 = result.unwrap();
        
        // Check updated balance
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, token_amount + token_amount2);
        assert_eq!(balance.locked_balance, token_amount + token_amount2);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.deposits.len(), 2);
    }

    #[test]
    fn test_token_burning() {
        setup();
        let _caller = Principal::anonymous();
        
        // Mint some tokens
        let mint_result = mint_tokens(10000, LockPeriod::ThreeMonths);
        assert!(mint_result.is_ok());
        let token_amount = mint_result.unwrap();

        // Try to burn more than available
        let result = burn_tokens(token_amount + 1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient available balance");

        // Try early withdrawal (should incur penalty)
        let result = burn_tokens(token_amount / 2);
        assert!(result.is_ok());
        let burned_amount = result.unwrap();
        
        // Check balance after early withdrawal
        let balance = get_balance().unwrap();
        assert_eq!(balance.total_balance, token_amount - burned_amount);
        assert_eq!(balance.locked_balance, token_amount - burned_amount);
        assert_eq!(balance.available_balance, 0);
        assert_eq!(balance.deposits.len(), 1);
    }

    #[test]
    fn test_reward_calculation() {
        setup();
        let _caller = Principal::anonymous();
        
        // Mint tokens with different lock periods
        let amount1 = 10000;
        let amount2 = 20000;
        
        mint_tokens(amount1, LockPeriod::ThreeMonths).unwrap();
        mint_tokens(amount2, LockPeriod::TwelveMonths).unwrap();

        // Apply rewards
        let result = apply_rewards();
        assert!(result.is_ok());
        let reward_info = result.unwrap();

        // Check that rewards were calculated
        assert!(reward_info.total_rewards > 0);
        assert!(reward_info.last_reward_calculation > 0);
    }

    #[test]
    fn test_milestone_rewards() {
        setup();
        let _caller = Principal::anonymous();
        
        // Create a goal
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 10000,
            category: "Savings".to_string(),
            deadline: None,
        };
        let goal_id = create_savings_goal(goal_args).unwrap().id;

        // Add contributions to reach different milestones
        let deposit_args = DepositArgs {
            goal_id,
            amount: 2500, // 25% milestone
        };
        deposit(deposit_args).unwrap();

        // Check milestone rewards
        let result = check_milestone_rewards(goal_id);
        assert!(result.is_ok());
        let rewards = result.unwrap();
        assert_eq!(rewards.len(), 1);
        assert_eq!(rewards[0].milestone_percentage, 25);

        // Add more to reach 50% milestone
        let deposit_args = DepositArgs {
            goal_id,
            amount: 2500,
        };
        deposit(deposit_args).unwrap();

        // Check updated milestone rewards
        let result = check_milestone_rewards(goal_id);
        assert!(result.is_ok());
        let rewards = result.unwrap();
        assert_eq!(rewards.len(), 1); // Only new milestone
        assert_eq!(rewards[0].milestone_percentage, 50);
    }

    #[test]
    fn test_early_completion_rewards() {
        setup();
        let _caller = Principal::anonymous();
        
        // Create a goal with deadline
        let goal_args = CreateGoalArgs {
            name: "Test Goal".to_string(),
            target_amount: 10000,
            category: "Savings".to_string(),
            deadline: Some(TEST_TIME + 30 * ONE_DAY), // 30 days deadline
        };
        let goal_id = create_savings_goal(goal_args).unwrap().id;

        // Complete the goal before deadline
        let deposit_args = DepositArgs {
            goal_id,
            amount: 10000,
        };
        deposit(deposit_args).unwrap();

        // Check early completion reward
        let result = check_early_completion_reward(goal_id);
        assert!(result.is_ok());
        let reward = result.unwrap();
        assert!(reward.is_some());
        assert_eq!(reward.unwrap().amount, (10000 as f64 * EARLY_COMPLETION_REWARD_RATE) as u64);

        // Check that reward is not given twice
        let result = check_early_completion_reward(goal_id);
        assert!(result.is_ok());
        let reward = result.unwrap();
        assert!(reward.is_none());
    }

    #[test]
    fn test_lock_period_penalties() {
        setup();
        let _caller = Principal::anonymous();
        
        // Test different lock periods and their penalties
        let test_cases = vec![
            (LockPeriod::ThreeMonths, THREE_MONTHS_PENALTY),
            (LockPeriod::SixMonths, SIX_MONTHS_PENALTY),
            (LockPeriod::TwelveMonths, TWELVE_MONTHS_PENALTY),
        ];

        for (lock_period, expected_penalty) in test_cases {
            let amount = 10000;
            mint_tokens(amount, lock_period.clone()).unwrap();

            // Try early withdrawal
            let result = burn_tokens(amount);
            assert!(result.is_ok());
            let burned_amount = result.unwrap();
            
            // Check that penalty was applied
            let expected_amount = (amount as f64 * (1.0 - expected_penalty)) as u64;
            assert_eq!(burned_amount, expected_amount);
        }
    }
} 