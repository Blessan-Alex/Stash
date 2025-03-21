export const idlFactory = ({ IDL }) => {
  const LockPeriod = IDL.Variant({
    'ThreeMonths': IDL.Null,
    'SixMonths': IDL.Null,
    'TwelveMonths': IDL.Null,
  });

  const TokenDeposit = IDL.Record({
    'amount': IDL.Nat64,
    'lock_period': LockPeriod,
    'deposit_time': IDL.Nat64,
    'interest_rate': IDL.Float64,
    'early_withdrawal_penalty': IDL.Float64,
  });

  const UserBalance = IDL.Record({
    'total_balance': IDL.Nat64,
    'locked_balance': IDL.Nat64,
    'available_balance': IDL.Nat64,
    'deposits': IDL.Vec(TokenDeposit),
    'rewards_earned': IDL.Nat64,
  });

  return IDL.Service({
    'mint_tokens': IDL.Func([IDL.Nat64, LockPeriod], [IDL.Variant({ 'Ok': IDL.Nat64, 'Err': IDL.Text })], []),
    'burn_tokens': IDL.Func([IDL.Nat64], [IDL.Variant({ 'Ok': IDL.Nat64, 'Err': IDL.Text })], []),
    'get_balance': IDL.Func([], [IDL.Variant({ 'Ok': UserBalance, 'Err': IDL.Text })], ['query']),
    'apply_rewards': IDL.Func([], [IDL.Variant({ 'Ok': IDL.Nat64, 'Err': IDL.Text })], []),
  });
};

export const init = ({ IDL }) => { return []; };
