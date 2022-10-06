  #![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::{DispatchError, DispatchResult, result::Result}, ensure, log, pallet_prelude::*, traits::{Currency, Randomness}};
use frame_support::traits::UnixTime;
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_core::sr25519;
  use scale_info::prelude::string::String;
use sp_runtime::{traits::{IdentifyAccount, Verify},AnySignature};
pub use sp_std::{convert::Into,str};
pub use sp_std::vec::Vec;
pub use sp_std::vec;
pub use pallet::*;
use pallet_nft_currency::NonFungibleToken;
use lite_json::{json_parser::parse_json};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod order;
pub use order::Order;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::ExistenceRequirement;
	use sp_runtime::SaturatedConversion;
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

	#[pallet::storage]
	#[pallet::getter(fn total_order)]
	pub type TotalOrder<T> = StorageValue<_, u32, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn borrowers)]
	// AccountId => List of borrowing with hash id
	pub(super) type Borrowers<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn lenders)]
	// AccountId => List of lending hash id
	pub(super) type Lenders<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

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
		Created(T::AccountId, T::AccountId, Vec<u8>, u64, u64),
		Terminated(Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T>{
		NotMatchToken,
		NotMatchMaker,
		TimeOver,
		NotOwner,
		NotEnoughFee,
		NoneExist,
		SignatureVerifyError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call ]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(35_678_000)]
		pub fn create_rental(origin: OriginFor<T>, lender: T::AccountId, borrower: T::AccountId,message_left:Vec<u8>, signature_left: Vec<u8>,message_right:Vec<u8>, signature_right: Vec<u8> ) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			if caller == lender  {
				Self::verify_signature(message_right.clone(),signature_right.clone(),&borrower)?;
			} else if caller == borrower {
				Self::verify_signature(message_left.clone(), signature_left.clone(), &lender)?;
			}
			let order_left = Self::parse_to_order(&message_left);
			log::info!("data order {:?}", order_left);
			let order_right = Self::parse_to_order(&message_right);
			log::info!("data order {:?}", order_right);
			let fullfilled_order = Self::match_order(order_left, order_right).unwrap();
			let rent_fee = fullfilled_order.fee;
			Self::transfer_asset(fullfilled_order);
			//let _ = T::Currency::transfer(&borrower,&lender,rent_fee.saturated_into(),ExistenceRequirement::KeepAlive);
			Ok(())
		}
	}
}

// helper functions
impl<T: Config> Pallet<T> {
	fn gen_hash_id() -> Vec<u8> {
		let nonce = TotalOrder::<T>::get();
		let n = nonce.encode();
		let (rand, _) = T::Randomness::random(&n);
		rand.encode()
	}

	fn verify_signature(data: Vec<u8>,signature: Vec<u8>,who: &T::AccountId) -> Result<(), DispatchError> {
		// sr25519 always expects a 64 byte signature.
		let signature: AnySignature = sr25519::Signature::from_slice(signature.as_ref())
			.ok_or(Error::<T>::SignatureVerifyError)?
			.into();

		// In Polkadot, the AccountId is always the same as the 32 byte public key.
		let account_bytes: [u8; 32] = account_to_bytes(who)?;
		let public_key = sr25519::Public::from_raw(account_bytes);

		// Check if everything is good or not.
		match signature.verify(data.as_slice(), &public_key) {
			true => Ok(()),
			false => Err(Error::<T>::SignatureVerifyError)?,
		}
	}

	fn calculate_day_renting(due_date:u64) -> u64{
		let part = due_date-T::Timestamp::now().as_secs();
		part/24
	}

	/// Parse the json object to Order struct
	fn parse_to_order(message: &Vec<u8>) -> Order{
		let data = str::from_utf8(message).unwrap();
		let order_data = parse_json(data).unwrap().to_object().unwrap();
		let mut order : Order = Order {
			maker: vec![],
			taker: vec![],
			fee: 0,
			token: vec![],
			due_date: 0
		};

		for data in order_data.into_iter(){
			let key = data.0;
			let k =  key.iter().map(|c| *c as u8).collect::<Vec<_>>();

			if k =="maker".as_bytes().to_vec() {
				let value = data.1.to_string().unwrap().iter().map(|c| *c as u8).collect::<Vec<_>>();
				order.maker = value;
			} else if k == "taker".as_bytes().to_vec() {
				let value = data.1.to_string().unwrap().iter().map(|c| *c as u8).collect::<Vec<_>>();
				order.taker = value;
			} else if k == "fee".as_bytes().to_vec(){
				let value = data.1.to_number().unwrap().integer;
				log::info!("fee {:?}", value);
				order.fee = value;
			} else if k == "token".as_bytes().to_vec() {
				let value = data.1.to_string().unwrap().iter().map(|c| *c as u8).collect::<Vec<_>>();
				order.token = value;
			} else if k == "due_date".as_bytes().to_vec(){
				let value = data.1.to_number().unwrap().integer;
				log::info!("fee {:?}", value);
				order.due_date = value;
			}
		}
		order
	}

	fn match_order(order_left: Order, mut order_right: Order) -> Result<Order, DispatchError> {
		ensure!(order_left.token == order_right.token, Error::<T>::NotMatchToken);
		ensure!(order_left.maker == order_right.maker, Error::<T>::NotMatchMaker);
		ensure!(order_left.due_date >= order_right.due_date, Error::<T>::TimeOver);
		let total_renting_days = Self::calculate_day_renting(order_right.due_date);
		let total_fee = order_left.fee * total_renting_days;
		log::info!("Total fee: {}", total_fee);
		order_right.fee = total_fee;

		Ok(order_right)
	}

	fn transfer_asset(order:Order) {
		let token_id = String::from_utf8(order.token).unwrap();
		log::info!("Transfer asset: {}", token_id);
	}
}

// This function converts a 32 byte AccountId to its byte-array equivalent form.
fn account_to_bytes<AccountId>(account: &AccountId) -> Result<[u8; 32], DispatchError>
	where
		AccountId: Encode,
{
	let account_vec = account.encode();
	ensure!(account_vec.len() == 32, "AccountId must be 32 bytes.");
	let mut bytes = [0u8; 32];
	bytes.copy_from_slice(&account_vec);
	Ok(bytes)
}
