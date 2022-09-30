use frame_support::{dispatch::{DispatchError, DispatchResult,fmt, result::Result}, pallet_prelude::*};
use sp_std::vec::Vec;
use crate::{Config, pallet};

// #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub enum Status {
// 	None,
// 	// if lender puts on for rent or borrower makes an offer
// 	Created,
// 	// if both side accepts offer
// 	Started,
// 	// if due_date renting is over
// 	Terminated,
// }
//
#[derive(Clone, Encode, Decode,PartialEq,TypeInfo,Debug)]
#[scale_info(skip_type_params(T))]
pub struct Order {
	maker: Vec<u8>,
	taker: Vec<u8>,
	fee: u64,
	token: Vec<u8>,
	due_date: u64,
}



