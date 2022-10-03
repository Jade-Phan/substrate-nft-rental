use frame_support::{ dispatch::{DispatchError, DispatchResult, fmt, result::Result}, pallet_prelude::*};
#[cfg(feature = "std")]
use serde::Deserialize;
#[cfg(feature = "std")]
use serde::Deserializer;
use sp_std::vec::Vec;


#[derive(Clone, Encode, Decode,PartialEq,TypeInfo,Debug)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct Order {
	#[serde(deserialize_with = "serde_bytes::deserialize")]
	maker: Vec<u8>,
	#[serde(deserialize_with = "serde_bytes::deserialize")]
	taker: Vec<u8>,
	#[serde(deserialize_with = "from_string")]
	fee: u64,
	#[serde(deserialize_with = "serde_bytes::deserialize")]
	token: Vec<u8>,
	#[serde(deserialize_with = "from_string")]
	due_date: u64,
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

fn from_string<'de, D>(d: D) -> Result<u64, D::Error>
	where
		D: Deserializer<'de>,
{
	let data = <&str>::deserialize(d)?;
	data.parse::<u64>()
}


