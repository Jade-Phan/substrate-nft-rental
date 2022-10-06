use frame_support::{pallet_prelude::*};
use sp_std::vec::Vec;
//use serde_bytes;

#[derive(Clone,Encode, Decode,PartialEq,TypeInfo,Debug)]
#[scale_info(skip_type_params(T))]
pub struct Order {
	//#[cfg_attr(feature = "std", serde(deserialize_with = "serde_bytes::deserialize"))]
	pub(crate) maker: Vec<u8>,
	//#[cfg_attr(feature = "std", serde(deserialize_with = "serde_bytes::deserialize"))]
	pub(crate) taker: Vec<u8>,
	//#[cfg_attr(feature = "std", serde(deserialize_with = "from_string"))]
	pub(crate) fee: u64,
	//#[cfg_attr(feature = "std", serde(deserialize_with = "serde_bytes::deserialize"))]
	pub(crate) token: Vec<u8>,
	//#[cfg_attr(feature = "std", serde(deserialize_with = "from_string"))]
	pub(crate) due_date: u64,
}


