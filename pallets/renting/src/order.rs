use frame_support::{dispatch::{DispatchError, DispatchResult,fmt, result::Result}, pallet_prelude::*};
#[no_std]
use serde::{Serialize,Deserialize,Serializer,Deserializer};
#[no_std]
use serde::ser::SerializeStruct;
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

#[derive(Clone, Encode, Decode,PartialEq, Serialize, Deserialize,TypeInfo,Debug)]
#[scale_info(skip_type_params(T))]
pub struct Order<T: Config> {
	maker: T::AccountId,
	taker: T::AccountId,
	fee: u64, // total fee that borrower must pay
	token: Vec<u8>,
	due_date: u64,
}



impl<T: Config> Serialize for Order<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
	{
		let mut s = serializer.serialize_struct("Order", 5)?;
		s.serialize_field("maker", &self.maker)?;
		s.serialize_field("taker", &self.taker)?;
		s.serialize_field("fee", &self.fee)?;
		s.serialize_field("token", &self.token)?;
		s.serialize_field("due_date", &self.due_date)?;
		s.end()
	}
}


