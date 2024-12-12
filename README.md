# Konsider

## Overview
A Vendor Risk Management application to create, manage, and visualize software and software reviews

## Usage

### 1. **Prerequisites**

Before starting, ensure the following tools are installed:

- **Docker**: Install Docker by following the instructions on [Get Docker](https://docs.docker.com/get-started/get-docker/).

### 2. **Clone the Repository**

```bash
git clone https://github.com/hmunye/konsider.git
```

```bash
cd konsider
```

### 3. **Running the Application**

Start by creating a `production.toml` file in `server` directory and `.env.production` file in `client` directory

```bash
touch ./server/config/production.toml
```

```bash
touch ./client/.env.production
```

Edit these files to suit your configuration. Here's an example setup:

`production.toml`:

```toml
[server]
port = 8443
host = "0.0.0.0"
environment = "production"
origin = "<server-ip>"
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
> Replace `<server-ip>` with the IP address of the server hosting the application
> Example: `192.168.0.29`

`.env.production`:

```env
PUBLIC_BASE_API_URL=https://<server-ip>
```
> Replace `<server-ip>` with the IP address of the server hosting the application
> Example: `192.168.0.29`


Create a Self-Signed Certificate to serve the PostgreSQL instance over HTTPS:

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

Run this command to restrict permissions of private key:

```bash
chmod og-rwx ./certs/server.key
```

Create a Self-Signed Certificate to serve the API over HTTPS:

```shell
mkdir ./server/certs && openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
-keyout ./server/certs/server.key -out ./server/certs/server.crt
```

Modify the `nginx.conf` file in the `docker` directory by replacing each instance of `server_name`
with the IP address of the server hosting the application

Example:

```bash
server_name 192.168.0.29;
```
> Make sure to update the value for every occurrence of `server_name`


Run this script to set up the Docker containers for the application:

```bash
./docker_start.sh
```

Once initialized, the application can be accessed at `https://<server-ip>`, where `<server-ip>` 
is the IP address assigned during the setup process

### 4. **What Next**

After successfully setting up the application, you can log in using the default administrator account:

- **Email:** `admin@brockport.edu`
- **Password:** `password123`

Once logged in, follow these steps to update your account details and change your password:

1. **Navigate to the Users Page:**
   Go to the **Users** page from the main navigation

2. **Access Account Settings:**
   - Find the **Ellipsis (three dots)** icon next to your account
   - Click the **Ellipsis** to view available actions for your account

3. **Select Action:**
   - To update your password, select **Change Password**
   - To update your account details (name, email, etc.), select **Edit**
