# Food Delivery Tracking

## Overview

This is Rust implementation of a smart contract for Food Delivery Tracking System deployed on the Internet Computer (ICP) blockchain. The smart contract is meticulously designed to handle every aspect of food delivery by managing clients, orders, items, reviews, and provides various functionalities related to these entities.
It utilizes the `ic_cdk` framework, and the data is stored in a thread-local storage structure (`StableBTreeMap`) managed by a `MemoryManager`.

## Getting Started

### Prerequisites

- Rust
- Internet Computer SDK
- IC CDK

### Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/calvin-79/food-delivery-tracking.git
    cd food-delivery-tracking
    ```

## Structure

The smart contract defines the following data structures:

- **Client**: Represents a client with information such as ID, name, address, phone, email, password, and a list of associated order IDs.

- **Order**: Represents an order with information such as ID, client ID, items (as a HashMap), total amount, order status, and delivery status.

- **Review**: Represents a review with information such as ID, client ID, item ID, rating, and comments.

- **Item**: Represents an item with information such as ID, name, description, price, and category.

The smart contract also implements traits like `Storable` and `BoundedStorable` for these data structures to enable serialization and storage functionalities.

## Functionality

The smart contract provides the following functionalities:

- **Create, Read, Update, and Delete (CRUD)** operations for clients, orders, reviews, and items.

- Retrieve all food items.

- Retrieve a specific food item by ID.

- Confirm delivery of an order.

- Update the status of an order.

- Retrieve all orders, a specific order by ID, and orders associated with a specific client.

- Retrieve all reviews, reviews associated with a specific item, and create a new review.

- Retrieve all clients and a specific client by ID.

- Create a new client.

## Data Storage

The contract utilizes thread-local static variables for memory management and storage, including a `MemoryManager` and separate `StableBTreeMap` instances for clients, orders, reviews, and items.

## Usage

The smart contract exposes a Candid interface, allowing users to interact with the contract's functionalities. It includes query and update functions for various operations.

### Query Functions

- `get_all_food_items()`: Retrieve all food items.

- `get_food_item_by_id(id: u64)`: Retrieve a specific food item by ID.

- `get_all_orders()`: Retrieve all orders.

- `get_order_by_id(id: u64)`: Retrieve a specific order by ID.

- `get_orders_by_client_id(client_id: u64)`: Retrieve orders associated with a specific client.

- `get_all_reviews()`: Retrieve all reviews.

- `get_reviews_by_item_id(item_id: u64)`: Retrieve reviews associated with a specific item.

- `get_all_clients()`: Retrieve all clients.

- `get_client_by_id(id: u64)`: Retrieve a specific client by ID.

### Update Functions

- `create_food_item(payload: ItemPayload)`: Create a new food item.

- `delete_food_item_by_id(id: u64)`: Delete a specific food item by ID.

- `confirm_delivery(payload: ConfirmDeliveryPayload)`: Confirm delivery of an order.

- `update_order_status(order_id: u64, status: String)`: Update the status of an order.

- `create_order(payload: OrderPayload)`: Create a new order.

- `create_review(payload: ReviewPayload)`: Create a new review.

- `delete_review_by_id(payload: DeleteReviewPayload)`: Delete a specific review by ID.

- `create_client(payload: ClientPayload)`: Create a new client.

## Error Handling

The smart contract defines an `Error` enum to handle various error scenarios, such as not found, already delivered, invalid payload, and unauthorized access.

## License

This smart contract is licensed under the [MIT License](LICENSE).

## More

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with food_delivery, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.icp0.io)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd food_delivery-tracking/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `production` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
