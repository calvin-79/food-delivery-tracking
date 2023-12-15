use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::collections::HashMap;
use std::borrow::Cow;
use validator::Validate;

// Define type aliases for convenience
pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

// Define a struct for the 'Client'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Client {
    pub id: u64,
    pub client_principal: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String
}

// Define a struct for the 'Order'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    pub id: u64,
    pub client_id: u64,
    pub items: HashMap<u64, u64>,
    pub total: u64,
    pub status: String,
    pub delivered: bool,
}

// Define a struct for the 'Review'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Review {
    pub id: u64,
    pub client_id: u64,
    pub item_id: u64,
    pub rating: u64,
    pub comment: String,
}

// Define a struct for the 'Item'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub id: u64,
    pub item_owner: String,
    // pub order_ids: Vec<u64>,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub category: String,
}

// Implement the 'Storable' trait for the 'Client', 'Order', 'Review' and 'Item' structs
impl Storable for Client {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Order {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Review {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Item {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement the 'BoundedStorable' trait for the 'Client', 'Order', 'Review' and 'Item' structs
impl BoundedStorable for Client {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Review {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Item {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define structs for payload data (used in update calls)
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Validate)]
pub struct ClientPayload {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub phone: String,
    pub email: String
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct OrderPayload {
    pub client_id: u64,
    pub items: Vec<OrderItem>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub item_id: u64,
    pub quantity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct ReviewPayload {
    pub item_id: u64,
    pub rating: u64,
    pub comment: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Validate)]
pub struct ItemPayload {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(length(min = 4))]
    pub description: String,
    pub price: u64,
    pub category: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct ConfirmDeliveryPayload {
    pub order_id: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DeleteReviewPayload {
    pub review_id: u64,
}

// Define an Error enum for handling errors
#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    AlreadyDelivered { msg: String },
    InvalidPayload { msg: String },
    Unauthorized { msg: String }
}