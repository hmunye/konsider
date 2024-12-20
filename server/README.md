# Konsider REST API (Standalone)

## Project Structure

```
├── .dockerignore
├── .env
├── .gitignore
├── .sqlx                               		# Query metadata for sqlx offline usage
├── Cargo.lock                     
├── Cargo.toml                          		# Manifest file for the project; specifies metadata and dependencies
├── Dockerfile
├── README.md                       
├── assets                                              # Fonts and images used in PDF generation
│   ├── font
│   │   ├── NotoSans-Regular.ttf
│   │   └── NotoSansSymbols2-Regular.ttf
│   └── img
│       └── logo.bmp
├── config                          
│   ├── local.toml                      		# Local development configuration settings
│   └── production.toml                 		# Production environment configuration settings
├── docker_entrypoint.sh
├── docs                          
│   ├── database_design.md              
│   └── design_and_architecture.md
├── logs                                		# Directory for storing log files generated by the application during runtime
├── migrations                          		# Database migrations directory for managing schema changes over time
├── scripts                         
│   └── init_db.sh                      		# Bash script to initialize a PostgreSQL database instance using Docker
├── src                         
│   ├── api                         
│   │   ├── controllers                 		# Manages incoming HTTP requests and prepares outgoing responses
│   │   ├── middleware                  		# Contains middleware functions
│   │   ├── models                      		# Defines data structures that map to database tables
│   │   ├── repositories                		# Manages data persistence and interactions with the database
│   │   ├── routes                      		# Defines application routes and their handlers, mapping URLs to controllers
│   │   ├── services                    		# Contains business logic and service functions
│   │   └── utils
│   │       ├── cookie.rs               		# Handles the creation of cookies with customizable flags and security settings
│   │       ├── generate_pdf.rs                         # Function to generate PDF response of a software review
│   │       ├── json_extractor.rs       		# Wrapper for axum::Json to customize errors
│   │       ├── jwt
│   │       │   ├── claims.rs           		# Defines the structure for JWT claims
│   │       │   ├── decode.rs           		# Function for decoding and validating JWT tokens
│   │       │   ├── generate.rs         		# Function for generating JWT tokens
│   │       │   ├── poll_database_worker.rs 	        # Background worker that periodically checks the database for valid tokens and updates token cache
│   │       │   ├── token_cache.rs                      # In-memory cache for storing active, valid JWT tokens
│   │       │   └── token_extractor.rs                  # Extracts JWT tokens from Cookie header in requests
│   │       ├── log_cleanup_worker.rs                   # Function to remove log files given path and retention days
│   │       ├── path_extractor.rs                       # Wrapper for axum::Path extractor to customize errors
│   │       └── query_extractor.rs                      # Wrapper for axum::Query extractor to extract custom query params
│   ├── config.rs                       		# Code for loading and managing application configuration
│   ├── error.rs                        		# Defines server and client error types and handling logic
│   ├── lib.rs                     
│   ├── log.rs                          		# Configures and manages the application’s logging behavior
│   ├── main.rs                 
│   └── server.rs                       		# Sets up and runs the application server
└── tests
    └── api
        ├── auth
        ├── common.rs
        ├── health
        ├── main.rs
        ├── requester
        ├── software
        ├── software_request
        ├── software_review
        └── user
```

## Usage

### 1. **Prerequisites**

Before starting, ensure the following tools are installed:

- **Rust**: Install Rust and Cargo using the [rustup](https://www.rust-lang.org/tools/install) tool.
- **Docker**: Install Docker by following the instructions on [Get Docker](https://docs.docker.com/get-started/get-docker/).
- **sqlx-cli**: Install the [sqlx-cli](https://github.com/launchbadge/sqlx) tool using Cargo with the following command:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. **Clone the Repository**

```bash
git clone https://github.com/hmunye/konsider.git
```
```bash
cd konsider/server
```

### 3. **Running the API**

#### Step 1: Configuration

For a production environment, start by creating a `production.toml` file, otherwise skip this step:

```bash
touch ./config/production.toml
```

Edit this file to suit your production configuration. Here's an example setup:

```toml
[server]
port = 8443
host = "0.0.0.0"
environment = "production"
# IP Address of the Server
origin = "192.168.0.29"
jwt_secret = ""

[log]
retention_days = 3

[database]
user = "k6r_user"
password = ""
database = "k6r"
db_host = "postgres"
db_port = 5432
require_ssl = true
```


Create a Self-Signed Certificate to serve the API over HTTPS:

```shell
mkdir certs && openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
-keyout certs/server.key -out certs/server.crt
```
> req -x509: Creates a self-signed certificate

> -nodes: Tells OpenSSL not to encrypt the private key

> -days 365: The certificate will be valid for 365 days

> -newkey rsa:2048: Generates a new RSA key that is 2048 bits long

> -keyout server.key: The private key will be saved to server.key

> -out server.crt: The certificate will be saved to server.crt


#### Step 2: Initialize Database

Use the `init_db.sh` script to spin up a PostgreSQL database container via Docker and apply migrations:

```bash
./scripts/init_db.sh
```
> Note: By default, the script will read values from `local.toml` configuration file

#### Step 3: Start the API

Local Environment:

```bash
cargo run
```
Production Environment:

```bash
ENVIRONMENT=production cargo run
```

#### Step 4: Health Check

Ensure the API is up and running by sending a health check request:

```bash
curl -v http://<your_host>:<your_port>/api/v1/health
```
> Note: A `204 No Content` response confirms the server is operational. `https` for production environment

## Testing

To run the API tests, follow these steps:

#### Step 1: Initialize Database

```bash
./scripts/init_db.sh
```
> Note: By default, the script will read values from `local.toml` configuration file

#### Step 2: Run unit and integration tests:

```bash
cargo test
```

Enable logging during tests with:

```bash
TEST_LOG=true cargo test
```

## Logging

This project uses structured logging in JSON format following Bunyan-style conventions.
By default, it logs to files with hourly rotation. Logs are cleaned up through a background
`tokio` process every 24 hours by default. The retention days can be configured in the
configuration

## sqlx

To update cached query metadata for sqlx offline usage, run the following command:

```bash
cargo sqlx prepare -- --all-targets
```
