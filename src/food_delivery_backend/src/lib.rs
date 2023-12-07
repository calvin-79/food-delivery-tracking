#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};
use validator::Validate;

// Define type aliases for convenience
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define a struct for the 'Client'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Client {
    id: u64,
    name: String,
    address: String,
    phone: String,
    email: String,
    password: String,
    order_ids: Vec<u64>,
}

// Define a struct for the 'Order'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Order {
    id: u64,
    client_id: u64,
    items: HashMap<u64, u64>,
    total: u64,
    status: String,
    delivered: bool,
}

// Define a struct for the 'Review'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Review {
    id: u64,
    client_id: u64,
    item_id: u64,
    rating: u64,
    comment: String,
}

// Define a struct for the 'Item'
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Item {
    id: u64,
    name: String,
    description: String,
    price: u64,
    category: String,
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

// Define thread-local static variables for memory management and storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static CLIENT_STORAGE: RefCell<StableBTreeMap<u64, Client, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ORDER_STORAGE: RefCell<StableBTreeMap<u64, Order, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static REVIEW_STORAGE: RefCell<StableBTreeMap<u64, Review, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static ITEM_STORAGE: RefCell<StableBTreeMap<u64, Item, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// Define structs for payload data (used in update calls)
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Validate)]
struct ClientPayload {
    #[validate(length(min = 2))]
    name: String,
    #[validate(length(min = 4))]
    address: String,
    phone: String,
    email: String,
    password: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct OrderPayload {
    client_id: u64,
    items: Vec<OrderItem>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct OrderItem {
    item_id: u64,
    quantity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ReviewPayload {
    item_id: u64,
    rating: u64,
    comment: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Validate)]
struct ItemPayload {
    #[validate(length(min = 2))]
    name: String,
    #[validate(length(min = 4))]
    description: String,
    price: u64,
    category: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ClientResponse {
    id: u64,
    name: String,
    address: String,
    phone: String,
    email: String,
    order_ids: Vec<u64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ConfirmDeliveryPayload {
    order_id: u64,
    password: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct DeleteReviewPayload {
    review_id: u64,
    password: String,
}

// Define query functions to get all Food Items
#[ic_cdk::query]
fn get_all_food_items() -> Result<Vec<Item>, Error> {
    // Retrieve all items from the storage
    let items_vec: Vec<(u64, Item)> = ITEM_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the items from the tuple and create a vector
    let items: Vec<Item> = items_vec.into_iter().map(|(_, item)| item).collect();

    // Check if any items are found
    match items.len() {
        0 => Err(Error::NotFound {
            msg: format!("no Food items for order could be found"),
        }),
        _ => Ok(items),
    }
}

// Define query functions to get a specific Food Item by id
#[ic_cdk::query]
fn get_food_item_by_id(id: u64) -> Result<Item, Error> {
    // Retrieve the item from the storage
    let item: Option<Item> = ITEM_STORAGE.with(|s| s.borrow().get(&id));

    // Check if the item is found
    match item {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("no Food item could be found for id: {}", id),
        }),
    }
}

// Define update functions to create a new Food item
#[ic_cdk::update]
fn create_food_item(payload: ItemPayload) -> Result<Item, Error> {
    // Validate the payload
    let validate_payload = payload.validate();
    if validate_payload.is_err() {
        return Err(Error::InvalidPayload {
            msg: validate_payload.unwrap_err().to_string(),
        });
    }

    // Retrieve the next id from the storage
    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    // Create a new Food item
    let item: Item = Item {
        id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        category: payload.category,
    };

    // Store the new Food item in the storage
    ITEM_STORAGE.with(|s| s.borrow_mut().insert(id, item.clone()));

    // Return the new Food item
    Ok(item)
}

// Define query functions to delete a specific Food Item by id
#[ic_cdk::update]
fn delete_food_item_by_id(id: u64) -> Result<String, Error> {
    //    check if the item is exists
    match ITEM_STORAGE.with(|s| s.borrow().get(&id)) {
        Some(_) => (),
        None => {
            return Err(Error::NotFound {
                msg: format!("Food item id: {} could not be found", id),
            })
        }
    }

    // delete item reviews
    let reviews_vec: Vec<(u64, Review)> = REVIEW_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the reviews from the tuple and create a vector
    let reviews: Vec<Review> = reviews_vec.into_iter().map(|(_, review)| review).collect();

    // Filter the reviews by item_id
    let reviews_by_item_id: Vec<Review> = reviews
        .into_iter()
        .filter(|review| review.item_id == id)
        .collect();

    // Check if any reviews are found
    match reviews_by_item_id.len() {
        0 => (),
        _ => {
            for review in reviews_by_item_id {
                REVIEW_STORAGE.with(|s| s.borrow_mut().remove(&review.id));
            }
        }
    }

    // Delete the Food item from the storage
    match ITEM_STORAGE.with(|s| s.borrow_mut().remove(&id)) {
        Some(_) => Ok(format!("Food item id: {} deleted", id)),
        None => Err(Error::NotFound {
            msg: format!("Food item id: {} could not be deleted", id),
        }),
    }
}

//  get food items by category
#[ic_cdk::query]
fn get_food_items_by_category(category: String) -> Result<Vec<Item>, Error> {
    // Retrieve all items from the storage
    let items_vec: Vec<(u64, Item)> = ITEM_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the items from the tuple and create a vector
    let items: Vec<Item> = items_vec.into_iter().map(|(_, item)| item).collect();

    // Filter the items by category
    let items_by_category: Vec<Item> = items
        .into_iter()
        .filter(|item| (item.category).contains(&category) || (item.description).contains(&category))
        .collect();

    // Check if any items are found
    match items_by_category.len() {
        0 => Err(Error::NotFound {
            msg: format!("no Food items for category: {} could be found", category),
        }),
        _ => Ok(items_by_category),
    }
}

// Define query functions to get all Orders
#[ic_cdk::query]
fn get_all_orders() -> Result<Vec<Order>, Error> {
    // Retrieve all orders from the storage
    let orders_vec: Vec<(u64, Order)> = ORDER_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the orders from the tuple and create a vector
    let orders: Vec<Order> = orders_vec.into_iter().map(|(_, order)| order).collect();

    // Check if any orders are found
    match orders.len() {
        0 => Err(Error::NotFound {
            msg: format!("no orders could be found"),
        }),
        _ => Ok(orders),
    }
}

// Define query functions to get a specific Order by id
#[ic_cdk::query]
fn get_order_by_id(id: u64) -> Result<Order, Error> {
    // Retrieve the order from the storage
    let order: Option<Order> = ORDER_STORAGE.with(|s| s.borrow().get(&id));

    // Check if the order is found
    match order {
        Some(order) => Ok(order),
        None => Err(Error::NotFound {
            msg: format!("no order could be found for id: {}", id),
        }),
    }
}

// Define query functions to get all Orders for a specific Client
#[ic_cdk::query]
fn get_orders_by_client_id(client_id: u64) -> Result<Vec<Order>, Error> {
    // Retrieve all orders from the storage
    let orders_vec: Vec<(u64, Order)> = ORDER_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the orders from the tuple and create a vector
    let orders: Vec<Order> = orders_vec.into_iter().map(|(_, order)| order).collect();

    // Filter the orders by client_id
    let orders_by_client_id: Vec<Order> = orders
        .into_iter()
        .filter(|order| order.client_id == client_id)
        .collect();

    // add item quantity to Item
    // Check if any orders are found
    match orders_by_client_id.len() {
        0 => Err(Error::NotFound {
            msg: format!("no orders could be found for client_id: {}", client_id),
        }),
        _ => Ok(orders_by_client_id),
    }
}

// Define query functions to confirm order delivery
#[ic_cdk::update]
fn confirm_delivery(payload: ConfirmDeliveryPayload) -> Result<String, Error> {
    // Retrieve the order from the storage
    let order: Option<Order> = ORDER_STORAGE.with(|s| s.borrow().get(&payload.order_id));

    // Check if the order is found
    match order {
        Some(order) => {
            // Retrieve the client from the storage
            let client: Option<Client> = CLIENT_STORAGE.with(|s| s.borrow().get(&order.client_id));

            // Check if the client is found
            match client {
                Some(client) => {
                    // Check if the password matches
                    if client.password == payload.password {
                        // Check if the order is already delivered
                        if order.delivered {
                            return Err(Error::AlreadyDelivered {
                                msg: format!("order id: {} is already delivered", order.id),
                            });
                        }

                        // Update the order status
                        ORDER_STORAGE.with(|s| {
                            s.borrow_mut().insert(
                                order.id,
                                Order {
                                    delivered: true,
                                    status: "order delivered".to_string(),
                                    ..order
                                },
                            )
                        });

                        Ok(format!("order id: {} is delivered", order.id))
                    } else {
                        Err(Error::Unauthorized {
                            msg: format!("password is incorrect"),
                        })
                    }
                }
                None => Err(Error::NotFound {
                    msg: format!("no client could be found for id: {}", order.client_id),
                }),
            }
        }
        None => Err(Error::NotFound {
            msg: format!("no order could be found for id: {}", payload.order_id),
        }),
    }
}

// Define update functions to update order status
#[ic_cdk::update]
fn update_order_status(order_id: u64, status: String) -> Result<String, Error> {
    // Retrieve the order from the storage
    let order: Option<Order> = ORDER_STORAGE.with(|s| s.borrow().get(&order_id));

    // Check if the order is found
    match order {
        Some(order) => {
            // Update the order status
            ORDER_STORAGE.with(|s| {
                s.borrow_mut().insert(
                    order.id,
                    Order {
                        status: status.clone(),
                        ..order
                    },
                )
            });

            Ok(format!(
                "order id: {} status updated to {}",
                order.id, status
            ))
        }
        None => Err(Error::NotFound {
            msg: format!("no order could be found for id: {}", order_id),
        }),
    }
}

// Define update functions to create a new Order
#[ic_cdk::update]
fn create_order(payload: OrderPayload) -> Result<Order, Error> {
    // Retrieve the next id from the storage
    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    let payload_items: HashMap<u64, u64> = payload
        .items
        .into_iter()
        .map(|item| (item.item_id, item.quantity))
        .collect();

    let items_vec: Vec<(u64, Item)> = ITEM_STORAGE.with(|s| s.borrow().iter().collect());
    let items: Vec<Item> = items_vec.into_iter().map(|(_, item)| item).collect();
    // get order items from payload
    let order_items: Vec<Item> = items
        .into_iter()
        .filter(|item| payload_items.contains_key(&item.id))
        .collect();

    // calculate total price by multiplying item price by quantity
    let total: u64 = order_items
        .iter()
        .map(|item| {
            let item = ITEM_STORAGE.with(|s| s.borrow().get(&item.id).unwrap());
            let quantity = payload_items.get(&item.id).unwrap();
            item.price * quantity
        })
        .sum();

    // Create a new Order
    let order: Order = Order {
        id,
        client_id: payload.client_id,
        items: payload_items,
        total,
        status: "order placed".to_string(),
        delivered: false,
    };

    // Store the new Order in the storage
    ORDER_STORAGE.with(|s| s.borrow_mut().insert(id, order.clone()));

    // Return the new Order
    Ok(order)
}

// Define query functions to get all Reviews
#[ic_cdk::query]
fn get_all_reviews() -> Result<Vec<Review>, Error> {
    // Retrieve all reviews from the storage
    let reviews_vec: Vec<(u64, Review)> = REVIEW_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the reviews from the tuple and create a vector
    let reviews: Vec<Review> = reviews_vec.into_iter().map(|(_, review)| review).collect();

    // Check if any reviews are found
    match reviews.len() {
        0 => Err(Error::NotFound {
            msg: format!("no reviews could be found"),
        }),
        _ => Ok(reviews),
    }
}

// Define query functions to get all Reviews for a specific Item
#[ic_cdk::query]
fn get_reviews_by_item_id(item_id: u64) -> Result<Vec<Review>, Error> {
    // Retrieve all reviews from the storage
    let reviews_vec: Vec<(u64, Review)> = REVIEW_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the reviews from the tuple and create a vector
    let reviews: Vec<Review> = reviews_vec.into_iter().map(|(_, review)| review).collect();

    // Filter the reviews by item_id
    let reviews_by_item_id: Vec<Review> = reviews
        .into_iter()
        .filter(|review| review.item_id == item_id)
        .collect();

    // Check if any reviews are found
    match reviews_by_item_id.len() {
        0 => Err(Error::NotFound {
            msg: format!("no reviews could be found for item_id: {}", item_id),
        }),
        _ => Ok(reviews_by_item_id),
    }
}

// Define update functions to create a new Review
#[ic_cdk::update]
fn create_review(payload: ReviewPayload) -> Result<Review, Error> {
    // Retrieve the next id from the storage
    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    // Create a new Review
    let review: Review = Review {
        id,
        client_id: 0,
        item_id: payload.item_id,
        rating: payload.rating,
        comment: payload.comment,
    };

    // Store the new Review in the storage
    REVIEW_STORAGE.with(|s| s.borrow_mut().insert(id, review.clone()));

    // Return the new Review
    Ok(review)
}

// Define query functions to delete a specific Review by id
#[ic_cdk::update]
fn delete_review_by_id(payload: DeleteReviewPayload) -> Result<String, Error> {
    // Retrieve the review from the storage
    let review: Option<Review> = REVIEW_STORAGE.with(|s| s.borrow().get(&payload.review_id));

    // Check if the review is found
    match review {
        Some(review) => {
            // Retrieve the client from the storage
            let client: Option<Client> = CLIENT_STORAGE.with(|s| s.borrow().get(&review.client_id));

            // Check if the client is found
            match client {
                Some(client) => {
                    // Check if the password matches
                    if client.password == payload.password {
                        // Delete the Review from the storage
                        match REVIEW_STORAGE.with(|s| s.borrow_mut().remove(&review.id)) {
                            Some(_) => Ok(format!("Review id: {} deleted", review.id)),
                            None => Err(Error::NotFound {
                                msg: format!("Review id: {} could not be deleted", review.id),
                            }),
                        }
                    } else {
                        Err(Error::Unauthorized {
                            msg: format!("password is incorrect"),
                        })
                    }
                }
                None => Err(Error::NotFound {
                    msg: format!("no client could be found for id: {}", review.client_id),
                }),
            }
        }
        None => Err(Error::NotFound {
            msg: format!("no review could be found for id: {}", payload.review_id),
        }),
    }
}

// Define query functions to get all Clients
#[ic_cdk::query]
fn get_all_clients() -> Result<Vec<ClientResponse>, Error> {
    // Retrieve all clients from the storage
    let clients_vec: Vec<(u64, Client)> = CLIENT_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the clients from the tuple and create a vector
    let clients: Vec<Client> = clients_vec.into_iter().map(|(_, client)| client).collect();

    // Check if any clients are found
    match clients.len() {
        0 => Err(Error::NotFound {
            msg: format!("no clients could be found"),
        }),
        _ => {
            // Create a vector of ClientResponse structs
            let mut client_responses: Vec<ClientResponse> = Vec::new();

            // Iterate over the clients and create a ClientResponse struct for each client
            for client in clients {
                let client_response: ClientResponse = ClientResponse {
                    id: client.id,
                    name: client.name,
                    address: client.address,
                    phone: client.phone,
                    email: client.email,
                    order_ids: client.order_ids,
                };
                client_responses.push(client_response);
            }

            Ok(client_responses)
        }
    }
}

// Define query functions to get a specific Client by id
#[ic_cdk::query]
fn get_client_by_id(id: u64) -> Result<ClientResponse, Error> {
    // Retrieve the client from the storage
    let client: Option<Client> = CLIENT_STORAGE.with(|s| s.borrow().get(&id));

    // Check if the client is found
    match client {
        Some(client) => {
            // Create a ClientResponse struct
            let client_response: ClientResponse = ClientResponse {
                id: client.id,
                name: client.name,
                address: client.address,
                phone: client.phone,
                email: client.email,
                order_ids: client.order_ids,
            };

            Ok(client_response)
        }
        None => Err(Error::NotFound {
            msg: format!("no client could be found for id: {}", id),
        }),
    }
}

// Define update functions to create a new Client
#[ic_cdk::update]
fn create_client(payload: ClientPayload) -> Result<Client, Error> {
    // Validate the payload
    let validate_payload = payload.validate();
    if validate_payload.is_err() {
        return Err(Error::InvalidPayload {
            msg: validate_payload.unwrap_err().to_string(),
        });
    }

    // Retrieve the next id from the storage
    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    // Create a new Client
    let client: Client = Client {
        id,
        name: payload.name,
        address: payload.address,
        phone: payload.phone,
        email: payload.email,
        password: payload.password,
        order_ids: Vec::new(),
    };

    // Store the new Client in the storage
    CLIENT_STORAGE.with(|s| s.borrow_mut().insert(id, client.clone()));

    Ok(client)
}

// Define an Error enum for handling errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    AlreadyDelivered { msg: String },
    InvalidPayload { msg: String },
    Unauthorized { msg: String },
}

// Candid generator for exporting the Candid interface
ic_cdk::export_candid!();
