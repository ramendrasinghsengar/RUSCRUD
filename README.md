# CRUD USING RUST

Welcome to your new `CRUD using RUST` project and to the Internet Computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `CRUD using Rust`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd message_board/
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


## After Deploying the project and canister Follow the below commands to Perform Operation 


## Command Guide to Perform Operation

Enter these command in your Terminal 

### To `create message` Enter the below command 
```bash
dfx canister call message_board_backend create_message '("hello Rust",null)'
```

Your Output will be something like this 
```
(
  variant {
    17_724 = record {
      23_515 = 3 : nat64;
      272_465_847 = null;
      427_265_337 = "hello Rust";
      1_779_848_746 = 1_730_964_785_566_115_402 : nat64;
      1_937_500_811 = principal "<PRINCIPAL ID OF MESSAGE CREATOR>";
      1_962_907_452 = 0 : nat32;
      2_136_900_368 = null;
      2_871_942_152 = vec {};
    }
  },
)
```

### To `Read all message form author` Enter the below command 
```bash
dfx canister call message_board_backend get_messages_by_author '(principal "<YOUR PRINCIPAL ID>")'
```
Your Output will be something like this :
It will show all the created Message by author
```
(
  vec {
    record {
      23_515 = 1 : nat64;
      272_465_847 = null;
      427_265_337 = "hello world";
      1_779_848_746 = 1_730_961_371_261_578_644 : nat64;
      1_937_500_811 = principal "<PRINCIPAL ID OF MESSAGE CREATOR>";
      1_962_907_452 = 0 : nat32;
      2_136_900_368 = null;
      2_871_942_152 = vec {};
    };
    record {
      23_515 = 3 : nat64;
      272_465_847 = null;
      427_265_337 = "hello Rust";
      1_779_848_746 = 1_730_964_785_566_115_402 : nat64;
      1_937_500_811 = principal "<PRINCIPAL ID OF MESSAGE CREATOR>";
      1_962_907_452 = 0 : nat32;
      2_136_900_368 = null;
      2_871_942_152 = vec {};
    };
  },
)
```
### To `update the Message` Enter the below command 
```
dfx canister call message_board_backend update_message '(3 : nat64, record { content = "hello yash" })'
```
Your Output will be something like this :
It will Update your Message from `hello Rust` TO `hello yash`
```
(
  variant {
    17_724 = record {
      23_515 = 3 : nat64;
      272_465_847 = opt (1_730_965_266_630_815_943 : nat64);
      427_265_337 = "hello yash";
      1_779_848_746 = 1_730_964_785_566_115_402 : nat64;
      1_937_500_811 = principal "<PRINCIPAL ID OF MESSAGE CREATOR>";
      1_962_907_452 = 0 : nat32;
      2_136_900_368 = null;
      2_871_942_152 = vec {};
    }
  },
)
```

### To `Delete Message` Enter the below command 

```
dfx canister call message_board_backend delete_message '(3 : nat64)'
```
Your Output will be something like this :

```
WARN: Cannot fetch Candid interface for delete_message, sending arguments with inferred types.
(variant { 17_724 })
```
### To `Check Message Deleted or Not` Enter the below command 

```
dfx canister call message_board_backend is_message_deleted '(3 : nat64)'
```

Your Outpu will be something like this :

```
WARN: Cannot fetch Candid interface for is_message_deleted, sending arguments with inferred types.
(true)
```
If your message deleted successfully. It will rturn you `TRUE`  other wise It will throw you one Error. 
