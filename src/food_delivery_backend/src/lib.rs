#[macro_use]
extern crate serde;

use ic_cdk::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::collections::HashMap;
use std::cell::RefCell;
use validator::Validate;

mod types;
use types::*;

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

fn is_item_owner(item: &Item) -> Result<(), Error>{
    if item.item_owner != caller().to_string(){
        return Err(Error::Unauthorized { msg: format!("Caller is not the item owner of the item with id={}", item.id) })
    }else{
        Ok(())
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
        item_owner: caller().to_string(),
        // order_ids: Vec::new(),
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
    let item = get_food_item_by_id(id)?;
    is_item_owner(&item)?;

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
    // // Check if any reviews are found
    // match item.order_ids.len() {
    //     0 => (),
    //     _ => {
    //         for order_id in item.order_ids {
    //             ORDER_STORAGE.with(|s| s.borrow_mut().remove(&order_id));
    //         }
    //     }
    // }

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
                    // Check if the caller's principal is the client's principal
                    if client.client_principal == caller().to_string() {
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
                            msg: format!("Caller isn't the client's principal."),
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
    if payload.items.is_empty(){
        return Err(Error::InvalidPayload { msg: format!("Cannot create an order with no items.") })
    }
    let client = get_client_by_id(payload.client_id)?;

    if client.client_principal != caller().to_string(){
        return Err(Error::Unauthorized { msg: format!("Caller is not the client's principal") })
    }

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

    // let _save_updated_items = order_items.clone().into_iter().for_each(|mut item| {
    //     item.order_ids.push(id);
    //     ITEM_STORAGE.with(|s| s.borrow_mut().insert(item.id, item.clone()));
    // });


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
                    // Check if the caller's principal is the client's principal
                    if client.client_principal == caller().to_string() {
                        // Delete the Review from the storage
                        match REVIEW_STORAGE.with(|s| s.borrow_mut().remove(&review.id)) {
                            Some(_) => Ok(format!("Review id: {} deleted", review.id)),
                            None => Err(Error::NotFound {
                                msg: format!("Review id: {} could not be deleted", review.id),
                            }),
                        }
                    } else {
                        Err(Error::Unauthorized {
                            msg: format!("Caller is not the client's principal"),
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
fn get_all_clients() -> Result<Vec<Client>, Error> {
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
            Ok(clients)
        }
    }
}

// Define query functions to get a specific Client by id
#[ic_cdk::query]
fn get_client_by_id(id: u64) -> Result<Client, Error> {
    // Retrieve the client from the storage
    let client: Option<Client> = CLIENT_STORAGE.with(|s| s.borrow().get(&id));

    // Check if the client is found
    match client {
        Some(client) => {
            Ok(client)
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
        client_principal: caller().to_string(),
        name: payload.name,
        address: payload.address,
        phone: payload.phone,
        email: payload.email
    };

    // Store the new Client in the storage
    CLIENT_STORAGE.with(|s| s.borrow_mut().insert(id, client.clone()));

    Ok(client)
}

// Candid generator for exporting the Candid interface
ic_cdk::export_candid!();
