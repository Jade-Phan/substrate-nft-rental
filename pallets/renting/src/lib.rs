  #![cfg_attr(not(feature = "std"), no_std)]

  use std::io::Bytes;
  use sp_std::fmt::Write;
  use frame_support::{dispatch::{DispatchError, DispatchResult, result::Result}, ensure, log, pallet_prelude::*, traits::{Currency, Randomness}};
  use frame_support::traits::{UnixTime,ExistenceRequirement};
  use frame_system::{ensure_signed, pallet_prelude::*};
  use sp_core::sr25519;
  use scale_info::prelude::{string::String};
  use sp_runtime::{traits::{IdentifyAccount, Verify}, AnySignature,AccountId32,SaturatedConversion};
  pub use sp_std::{convert::Into,str};
  pub use sp_std::vec::Vec;
  pub use sp_std::vec;
  pub use pallet::*;
  use pallet_nft_currency::NonFungibleToken;
  use lite_json::{json_parser::parse_json,JsonValue,JsonObject};
  use bs58;
mod order;
  mod convert;
use convert::*;
  pub use order::Order;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Timestamp: UnixTime;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type TokenNFT: NonFungibleToken<Self::AccountId>;
		type Signature: Verify<Signer=Self::PublicKey> + Encode + Decode + Parameter;
		type PublicKey: IdentifyAccount<AccountId=Self::PublicKey> + Encode + Decode + Parameter;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn borrowers)]
	// AccountId => List of borrowing with hash id
	pub(super) type Borrowers<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cancel_order)]
	// AccountId => List of lending hash id
	pub(super) type CancelOrder<T: Config> =
	StorageMap<_, Blake2_128Concat, Vec<u8>, Order, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn rental)]
	// Hash Id -> Renting Info
	pub(super) type Rental<T: Config> =
	StorageMap<_, Blake2_128Concat, Vec<u8>, Order, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		MatchOrder(T::AccountId, T::AccountId, Vec<u8>),
		CancelOrder(Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T>{
		NotMatchToken,
		NotMatchLender,
		NotMatchBorrower,
		TimeOver,
		NotOwner,
		NotEnoughFee,
		NoneExist,
		SignatureVerifyError1,
		SignatureVerifyError2,
		NotCaller,
		AlreadyCanceled,
		NotOwnerOfOrder,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call ]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(35_678_000)]
		pub fn create_rental(origin: OriginFor<T>, lender: T::AccountId, borrower: T::AccountId,message_left:Vec<u8>, signature_left: Vec<u8>,message_right:Vec<u8>, signature_right: Vec<u8> ) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			if caller == lender {
				Self::verify_signature(message_right.clone(),signature_right.clone(),&borrower)?;
			} else if caller == borrower {
				Self::verify_signature(message_left.clone(), signature_left.clone(), &lender)?;
			} else {
				return Err(DispatchError::CannotLookup)
			}
			let lender_bytes = account_to_bytes(&lender).unwrap();
			let borrower_bytes = account_to_bytes(&borrower).unwrap();
			let order_left = Self::parse_to_order(lender_bytes.clone(),[0u8;32],&message_left).unwrap();
			let order_right = Self::parse_to_order(lender_bytes.clone(),borrower_bytes.clone(),&message_right).unwrap();
			ensure!(!CancelOrder::<T>::contains_key(order_left.clone().encode()) &&
				!CancelOrder::<T>::contains_key(order_right.clone().encode()),
				Error::<T>::AlreadyCanceled);
			let fulfilled_order = Self::match_order(order_left,order_right).unwrap();

			Self::transfer_asset(&lender,&borrower, fulfilled_order.clone());
			let hash_order = fulfilled_order.clone().encode();
			let token_id = fulfilled_order.clone().token;
			Rental::<T>::mutate(hash_order.clone(), |order| {
				*order = Some(fulfilled_order);
			});
			Borrowers::<T>::mutate(borrower.clone(), |orders| {
				orders.push(hash_order);
			});
			Self::deposit_event(Event::MatchOrder(lender, borrower,token_id));
			Ok(())
		}

		#[pallet::weight(35_678_000)]
		pub fn cancel_offer(origin:OriginFor<T>, message:Vec<u8>,is_lender:bool) -> DispatchResult{
			let caller = ensure_signed(origin)?;
			let account = account_to_bytes(&caller).unwrap();
			let order;
			if is_lender {
				order = Self::parse_to_order(account, [0u8;32], &message).unwrap();
				ensure!(account == order.lender, Error::<T>::NotOwnerOfOrder)
			} else {
				order = Self::parse_to_order([0u8;32], account, &message).unwrap();
				ensure!(account == order.borrower, Error::<T>::NotOwnerOfOrder)
			}
			CancelOrder::<T>::mutate(order.clone().encode(), |cancel_order|{
				*cancel_order = Some(order.clone());
			});
			Self::deposit_event(Event::CancelOrder(order.encode()));
			Ok(())
		}

		#[pallet::weight(35_678_000)]
		pub fn stop_renting(origin: OriginFor<T>, hash_order: Vec<u8>) -> DispatchResult{
			Ok(())
		}
	}
}

// helper functions
impl<T: Config> Pallet<T> {
	fn verify_signature(data: Vec<u8>,signature: Vec<u8>,who: &T::AccountId) -> Result<(), DispatchError> {
		// sr25519 always expects a 64 byte signature.
		let signature: AnySignature = sr25519::Signature::from_slice(signature.as_ref())
			.ok_or(Error::<T>::SignatureVerifyError1)?
			.into();

		// In Polkadot, the AccountId is always the same as the 32 byte public key.
		let account_bytes: [u8; 32] = account_to_bytes(who)?;
		let public_key = sr25519::Public::from_raw(account_bytes);

		// Check if everything is good or not.
		match signature.verify(data.as_slice(), &public_key) {
			true => Ok(()),
			false => Err(Error::<T>::SignatureVerifyError2)?,
		}
	}

	fn calculate_day_renting(due_date:u64) -> u64{
		let part = due_date-T::Timestamp::now().as_secs();
		part/24
	}

	/// Parse the json object to Order struct
	fn parse_to_order(lender:[u8;32],borrower:[u8;32],message: &Vec<u8>) -> Result<Order, DispatchError> {
		let data = str::from_utf8(message).unwrap();
		let order_data = parse_json(data).unwrap().to_object().unwrap();
		let mut order = Order {
			lender : [0u8;32],
			borrower : [0u8;32],
			fee: 0,
			token: vec![],
			due_date: 0
		};

		for data in order_data.into_iter(){
			let key = data.0;
			let k =  key.iter().map(|c| *c as u8).collect::<Vec<_>>();

			if k == "lender".as_bytes().to_vec(){
				let value = data.1.to_string().unwrap().iter().map(|c| *c as u8).collect::<Vec<_>>();
				let hex_account = convert_string_to_accountid(&String::from_utf8(value.clone()).unwrap());
				let account = convert_bytes_to_accountid(lender.clone());
				ensure!(hex_account == account, Error::<T>::NotMatchLender);
				order.lender = lender;
			} else if k == "borrower".as_bytes().to_vec(){
				let value = data.1.to_string().unwrap().iter().map(|c| *c as u8).collect::<Vec<_>>();
				let hex_account = convert_string_to_accountid(&String::from_utf8(value.clone()).unwrap());
				let account = convert_bytes_to_accountid(borrower.clone());
				log::info!("borrower {:?} {:?}", account, hex_account);
				ensure!(hex_account == account, Error::<T>::NotMatchBorrower);
				order.borrower = borrower;
			} else if k == "fee".as_bytes().to_vec(){
				let value = data.1.to_number().unwrap().integer;
				order.fee = value;
			} else if k == "token".as_bytes().to_vec() {
				let value = data.1.to_string().unwrap();
				log::info!("Token: {:?}", token);
				order.token = Vec::from(token);
			} else if k == "due_date".as_bytes().to_vec(){
				let value = data.1.to_number().unwrap().integer;
				ensure!(value >= T::Timestamp::now().as_secs(), Error::<T>::TimeOver);
				order.due_date = value;
			}
		}
		Ok(order)
	}

	fn match_order(order_left: Order, mut order_right: Order) -> Result<Order, DispatchError> {
		ensure!(order_left.token == order_right.token, Error::<T>::NotMatchToken);
		ensure!(order_left.lender == order_right.lender, Error::<T>::NotMatchLender);
		ensure!(order_left.due_date >= order_right.due_date, Error::<T>::TimeOver);
		ensure!(order_left.fee <= order_right.fee, Error::<T>::NotEnoughFee);
		let total_renting_days = Self::calculate_day_renting(order_right.due_date);
		let total_fee = order_right.fee * total_renting_days;
		order_right.fee = total_fee;

		Ok(order_right)
	}

	fn transfer_asset(lender:&T::AccountId, borrower:&T::AccountId,order:Order) {
		let token_id = String::from_utf8(order.token).unwrap();
		let _ = T::TokenNFT::transfer(lender.clone(), borrower.clone(), Vec::from(token_id.encode()));
		let _ = T::Currency::transfer(&lender,&borrower,order.fee.saturated_into(),ExistenceRequirement::KeepAlive);
	}


	// fn convert_bytes_to_hex(bytes: [u8;32])-> String{
	// 	let to_address = Self::convert_bytes_to_accountid(bytes);
	// 	let mut res = String::new();
	// 	write!(&mut res, "{:?}",to_address);
	// 	res
	// }
	//
	// fn convert_bytes_to_accountid(bytes: [u8;32])-> T::AccountId{
	// 	let account32: AccountId32 = bytes.into();
	// 	let mut to32:&[u8] = AccountId32::as_ref(&account32);
	// 	let to_address = T::AccountId::decode(&mut to32).unwrap();
	// 	to_address
	// }
	//
	// fn convert_string_to_accountid(account_str: &str)-> T::AccountId{
	// 	let mut output = vec![0xFF; 35];
	// 	bs58::decode(account_str).into(&mut output).unwrap();
	// 	let cut_address_vec:Vec<u8> = output.drain(1..33).collect();
	// 	let mut array = [0; 32];
	// 	let bytes = &cut_address_vec[..array.len()];
	// 	array.copy_from_slice(bytes);
	// 	let account32: AccountId32 = array.into();
	// 	let mut to32 = AccountId32::as_ref(&account32);
	// 	let to_address : T::AccountId = T::AccountId::decode(&mut to32).unwrap();
	// 	to_address
	// }
}



