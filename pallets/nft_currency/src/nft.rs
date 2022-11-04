use frame_support::{dispatch::{result::Result, DispatchError, DispatchResult}};
use frame_support::dispatch::DispatchErrorWithPostInfo;
pub use sp_std::*;
use sp_std::vec::Vec;

pub trait NonFungibleToken<AccountId>{
	fn token_uri(token_id: Vec<u8>) -> Vec<u8>;
	fn total() -> u32;
	fn owner_of_token(token_id: Vec<u8>) -> AccountId;
	fn create(owner:AccountId, name: Vec<u8>, symbol: Vec<u8>) -> DispatchResult;
	fn mint(to:AccountId,token_id:Vec<u8>, totalSupply: u32) -> DispatchResult;
	fn transfer(from: AccountId, to: AccountId, token_id: Vec<u8>, serial:u64) -> DispatchResult;
	fn set_token_uri(token_id: Vec<u8>, serial:u64, token_uri: Vec<u8>) -> DispatchResult;
	fn set_admin_account(admin_account: AccountId) -> DispatchResult;
	fn remove_admin_account(admin_account: AccountId) -> DispatchResult;

	fn approve(from: AccountId, to: AccountId,token_id: Vec<u8>, serial: u64) -> DispatchResult;
	fn is_approve_for_all(account_approve:(AccountId,AccountId), token_id: Vec<u8>) -> bool;
	fn set_approve_for_all(from: AccountId, to: AccountId, token_id:Vec<u8>) -> DispatchResult;
	fn delete_approval(from: AccountId, to: AccountId, token_id: Vec<u8>) -> DispatchResult;

	fn burn_single_token(token_id:Vec<u8>, serial: u64) -> DispatchResult;
	fn burn_all_tokens(token_id:Vec<u8>) -> DispatchResult;
}
