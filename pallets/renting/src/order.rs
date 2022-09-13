use frame_support::{dispatch::{result::Result, DispatchError, DispatchResult}};
pub use sp_std::*;
use sp_std::vec::Vec;
pub enum TypeOrder {
	ForRent,
	Borrow
}
pub struct Order{
	maker: AccountId,
	type_order: TypeOrder,
	fee_per_week:u4,
	timestamp: u64,
}
