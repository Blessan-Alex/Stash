import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type LockPeriod = { 'TwelveMonths' : null } |
  { 'ThreeMonths' : null } |
  { 'SixMonths' : null };
export interface TokenDeposit {
  'deposit_time' : bigint,
  'early_withdrawal_penalty' : number,
  'interest_rate' : number,
  'amount' : bigint,
  'lock_period' : LockPeriod,
}
export interface UserBalance {
  'available_balance' : bigint,
  'rewards_earned' : bigint,
  'locked_balance' : bigint,
  'total_balance' : bigint,
  'deposits' : Array<TokenDeposit>,
}
export interface _SERVICE {
  'apply_rewards' : ActorMethod<[], { 'Ok' : bigint } | { 'Err' : string }>,
  'burn_tokens' : ActorMethod<[bigint], { 'Ok' : bigint } | { 'Err' : string }>,
  'get_balance' : ActorMethod<[], { 'Ok' : UserBalance } | { 'Err' : string }>,
  'mint_tokens' : ActorMethod<
    [bigint, LockPeriod],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
