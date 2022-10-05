use frame_support::{pallet_prelude::*};
use scale_info::prelude::string::String;
use sp_std::vec::Vec;
//use serde_bytes;

#[derive(Clone, Encode, Decode,PartialEq,TypeInfo,Debug)]
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

// fn deserialize_maker<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
// 	where
// 		D: Deserializer<'de>,
// {
// 	let data = <&str>::deserialize(d)?;
// 	data.as_bytes().unwrap()
// }
//
// fn deserialize_taker<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
// 	where
// 		D: Deserializer<'de>,
// {
// 	let data = <&str>::deserialize(d)?;
// 	data.as_bytes().unwrap()
// }
//
// fn deserialize_fee<'de, D>(d: D) -> Result<u64, D::Error>
// 	where
// 		D: Deserializer<'de>,
// {
// 	let data = uint64::deserialize(d)?;
//
// }
//
// fn deserialize_token<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
// 	where
// 		D: Deserializer<'de>,
// {
// 	let data = <&str>::deserialize(d)?;
// }
//
// fn deserialize_due_date<'de, D>(d: D) -> Result<u64, D::Error>
// 	where
// 		D: Deserializer<'de>,
// {
// 	let data = <&str>::deserialize(d)?;
// }

// fn from_string<'de, D>(d: D) -> Result<u64, D::Error>
// 	where
// 		D: Deserializer<'de>
// {
// 	let data: &str= serde::Deserialize::deserialize(d)?;
// 	let str = String::from(data);
// 	let res = str.parse::<u64>();
// 	res
// }


