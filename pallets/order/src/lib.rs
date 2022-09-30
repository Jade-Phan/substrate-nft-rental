use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Order {
	maker: Vec<u8>,
	taker: Vec<u8>,
	fee: u64,
	token: Vec<u8>,
	due_date: u64,
}

