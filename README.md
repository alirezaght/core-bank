This is a simple core service for financial functionalities.

It can be integrated to any project but have in mind that you need to implement your own security layer and handle 
association of your customer to the addresses of this service.

## Getting started

This service needs to be connected to a postgres db. You can easily use the docker-compose-postgres.yaml to bring up
a postgres db. core bank service uses `DATABASE_URL` in os environments to connect to the db. so either set that env 
variable in a .env file if you want to run the project using `cargo run` or provide it on your docker-compose file or 
docker command if you want to run it through docker. 

## DB
After bringing up postgres db, you have to create a database named `cb`. To do that you can use postgres admin ui.
If you have brought up the postgres db by using the docker-compose-postgres.yaml compose file then you have access to 
postgres admin ui by opening the browser `http://localhost:5050`. create a server and connect to `core-bank-db`, the 
default password for user `postgres` is `postgres`.
Create a database and name it `cb`.

After that install diesel: 

`cargo install diesel_cli --no-default-features --features postgres`

And then set the `DATABASE_URL`.

`echo DATABASE_URL=postgres://postgres:postgres@localhost/cb > .env`

Run `diesel migration run` to create the required tables in db.


## Docker

To build the project you can run `docker build .` and after that an image named `core-bank` should be available on your machine.
There are docker-compose template files available that you can see how to bring up these images. 
by running `docker-compose -f docker-compose-postgres.yaml -f docker-compose-cb.yaml up` it should bring up all the services without any problem.

## Cargo

Instead of bringing the service inside a container you can build the project with cargo and run that.

- `cargo build --release`
- `./target/release/core-bank`

## Address

Every account has an address, and addresses are being generated using `Secp256k1`, so each address has private key and 
public key which you can use to sign a message or do any other cryptographic functionality on other layers.  

## Methods

Service will be up on port `3030` and you can access all the methods through rpc protocol.

You can even use curl to test the methods:

`curl -H "Content-type:application/json" -d '{"jsonrpc":"2.0", "id":"id","method":"create_address", "params":[] }' http://localhost:3030`

| Method name       	| Params                                                                               	| Description                                                                                                                                                                                                                                                                     	                                                      |
|-------------------	|--------------------------------------------------------------------------------------	|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| store_account     	| [address: String, detail: String, withdraw: bool, deposit: bool, comment: String]    	| Saves an account with the address and comment and withdaw or deposit permission.                                                                                                                                                                                                	                                                      |
| get_account       	| [address: String]                                                                    	| Returns the account's info associated with that address                                                                                                                                                                                                                         	                                                      |
| list_account      	| [page: i64, limit: i64]                                                              	| Returns all the accounts in the system with pagination                                                                                                                                                                                                                          	                                                      |
| create_address    	| [public_key: [u8]?]                                                                  	| Return an address. If public key is provided it generates the address based on that and won't return the private key, if the pub key is not provided, it will randomly use a pair key and use that to generate the address and will return the keys along the address as well.  	                                                      |
| block_withdraw    	| [address: String, comment: String]                                                   	| It will take away the withdraw permission of an address, and set the comment for that.                                                                                                                                                                                          	                                                      |
| unblock_withdraw  	| [address: String, comment: String]                                                   	| It will give the withdraw permission to that address, and set the comment for that action.                                                                                                                                                                                      	                                                      |
| block_deposit     	| [address: String, comment: String]                                                   	| It will take away the deposit permission of an address, and set the comment for that action.                                                                                                                                                                                    	                                                      |
| unblock_deposit   	| [address: String, comment: String]                                                   	| It will give the deposit permission to that address, and set the comment for that action.                                                                                                                                                                                       	                                                      |
| block             	| [address: String, comment: String]                                                   	| It will take away both withdraw and deposit permission from that address, and set the comment for that action.                                                                                                                                                                  	                                                      |
| unblock           	| [address: String, comment: String]                                                   	| It will give both withdraw and deposit permission to that address, and set the comment for that action.                                                                                                                                                                         	                                                      |
| lock              	| [address: String, amount: BigDecimal, comment: String, description: String]          	| This will block an exact amount of an address and will set the comment and desc for that action.                                                                                                                                                                                	                                                      |
| unlock            	| [address: String, amount: BigDecimal, comment: String, description: String]          	| This will unblock an exact amount of an address and will set the comment and desc for that action.                                                                                                                                                                              	                                                      |
| list_transactions 	| [address: String, page: i64, limit: i64]                                             	| This will return all the transactions of an address with pagination.                                                                                                                                                                                                            	                                                      |
| deposit           	| [address: String, amount: BigDecimal, comment: String, description: String]          	| This will deposit an amount to that address and will set comment and description for that action.                                                                                                                                                                                                                                    	  |
| withdraw          	| [address: String, amount: BigDecimal, comment: String, description: String]          	| This will withdraw an amount of an address and will set comment and description for that action.                                                                                                                                                                                                                                     	 |
| transfer          	| [from: String, to: String, amount: BigDecimal, comment: String, description: String] 	| This will transfer an amount from an address to another address and will set comment and description for that action.                                                                                                                                                           	                                                      |