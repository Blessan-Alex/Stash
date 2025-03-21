export const idlFactory = ({ IDL }) => {
  const LockPeriod = IDL.Variant({
    'TwelveMonths' : IDL.Null,
    'ThreeMonths' : IDL.Null,
    'SixMonths' : IDL.Null,
  });
  const TokenDeposit = IDL.Record({
    'deposit_time' : IDL.Nat64,
    'early_withdrawal_penalty' : IDL.Float64,
    'interest_rate' : IDL.Float64,
    'amount' : IDL.Nat64,
    'lock_period' : LockPeriod,
  });
  const UserBalance = IDL.Record({
    'available_balance' : IDL.Nat64,
    'rewards_earned' : IDL.Nat64,
    'locked_balance' : IDL.Nat64,
    'total_balance' : IDL.Nat64,
    'deposits' : IDL.Vec(TokenDeposit),
  });
  return IDL.Service({
    'apply_rewards' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'burn_tokens' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'get_balance' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : UserBalance, 'Err' : IDL.Text })],
        ['query'],
      ),
    'mint_tokens' : IDL.Func(
        [IDL.Nat64, LockPeriod],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
