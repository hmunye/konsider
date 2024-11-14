# K6R Design & Architecture

---

## Objective

Konsider (K6R) allows users to create and manage software reviews for software requests submitted through TeamDynamix. Each review captures details about the software and requester, and includes the option to export as PDFs for easy attachment to the corresponding TeamDynamix request. By centralizing all reviews in one location, it prevents duplicate reviews from being created, as users can quickly reference existing records. In the past, reviews were stored as individual documents, making it hard to track reviews made. Now, all review data is kept in one location, ensuring it's availability and streamlining the creation process

> Note: A `software review` is an initial evaluation of the requested software, checking its compliance with campus standards, guidelines, and overall suitability, along with any other relevant findings

---

## Application Components
### Storage:
We need to store data for users, requesters, software, software requests, and software reviews. PostgreSQL will be the chosen database. While database usage are expected to be low, it should be capable of handling a moderate volume of reads and writes efficiently

### API:
The API will manage all system interactions, such as submitting software reviews, retrieving past reviews, exporting PDF reports, querying for specific software details, and other CRUD operations for each resource. It will act as an intermediary between the frontend and database. Traffic is expected to be low, as this will be an internal application

### Frontend:
The frontend application will provide an interface for managing and viewing software reviews, requesters, and software details. Users will be able to easily submit, search, and view API data, as well as generate PDFs of reviews to attach to TeamDynamix requests. An admin dashboard will offer the same functionality as the user interface, with added capabilities for managing user accounts

---

## Backend Architecture

The backend will follow a Layered Architecture approach to promote separation of concerns and modularity. This approach structures the application into distinct layers, each responsible for a specific set of tasks. The layers I will implement include the Repository Layer, Service Layer, and Controller Layer

### Repository Layer:
- Provides an abstraction for data access, allowing interaction with the database without exposing the underlying storage details
- Decouples the domain logic in the Service Layer from the specifics of how data is stored or retrieved
### Service Layer:
- Contains the core business logic of the application, acting as an intermediary between the Repository and Controller Layers
- Handles the primary operations, processes data, and applies business rules
- Holds most of the application-specific logic
### Controller Layer:
- Manages the HTTP request/response cycle, acting as a bridge between user input and domain logic
- Processes incoming HTTP requests, invokes the necessary services, and formats the response
- Separates external interactions (e.g., web requests) from the internal domain logic, keeping the core logic independent of any specific framework or HTTP handling mechanism
### Models:
Models represent the core data structures in the application, used to define and manipulate entities that interact with the database

- **Repository Layer**: Models define the structure of the data stored in the database
- **Service Layer**: Models are the data structures that the business logic works with and processes
- **Controller Layer**: Models are converted into DTOs (Data Transfer Objects) to structure the data for responses
### Dependency Flow:
- The inner layers (Repository and Service) are decoupled from the outer layer (Controller)
- The outer layer (Controller) depends on the Service Layer, meaning the controller's responsibility is limited to routing data to the service without worrying about how the data is processed or stored
- The Service Layer depends on the Repository Layer to fetch and save data. The service layer focuses on business logic and doesn't need to know the details of data storage or retrieval, only how to interact with the repository

---
## Security
### JWT:
To authorize users securely, we will use JSON Web Tokens (JWTs). Though session-based authorization is another option, JWTs provide a more scalable, stateless approach that avoids database lookups on each request, which becomes an advantage if the application grows

#### Authentication/Authorization Flow:
1. **Authentication:** Users submit their credentials (email and password) to authenticate. Upon success, the server generates a JWT and sends it through a `Set-Cookie` header
2. **Token Storage:** The JWT is stored client-side in a secure, HTTP-only cookie, which is automatically sent with subsequent requests
3. **Token Validation:** The server validates each JWT symmetrically, checking authenticity and integrity before granting access
4. **Authorization:** Based on JWT claims (e.g., user role), the server determines if the user has permissions to access the requested resource
> Note: Symmetric encryption uses the same key to both sign and validate the JWT, while asymmetric encryption uses a private key to sign the JWT, and a public key for validation

#### JWT Claims:
The JWT claims include:

- `sub`: User ID
- `role`: User's Role
- `iat`: Issued At
- `exp`: Expiration Time
- `jti`: JWT ID for revocation tracking
> Note: `Role` is a custom claim defined for authorization purposes

> Note: `User ID` is used as the subject because it uniquely identifies a user within the system, while being less sensitive than a name or email

> Note: `jti` is a case-sensitive unique identifier for each JWT, which will be used in the revocation process

#### Token Revocation:
I prefer an approach that doesn’t burden the client with token management. Instead, token revocation is managed server-side with a background process, which polls the database and updates an in-memory cache. The approach ensures revoked tokens are promptly marked invalid without involving the client

##### Revocation Flow:
1. Long-Lived JWTs: Each JWT has a 24-hour expiration (`exp`), minimizing frequent renewals while keeping users logged in
2. Database Polling: A background process runs every 10–15 minutes to check the database for tokens flagged as revoked. After each polling cycle, it updates an in-memory cache of revoked tokens
3. Request Handling: For each incoming request, the server first validates the token normally, then checks the `jti` of the token against the in-memory cache:
- If the token is initially invalid, the request is rejected 
- If the token is valid but found in the cache, the request is rejected 
- If the token is valid and not found in the cache, the request proceeds as usual
##### Logout Flow:
- Token Revocation: When a user logs out, their JWT is marked as revoked in the database
- Cache Update: To ensure immediate effect, the revoked token is also added to the in-memory cache, preventing the delay of the next polling cycle
##### Pros:
- Efficient Authorization: Caching revoked tokens minimizes database queries by allowing quick lookup, reducing load
- Client Simplicity: Long-lived JWTs and server-managed revocation eliminate client-side token handling, simplifying the user experience
##### Cons:
- Scalability: If the application scales, switching to a shared, distributed cache (e.g., Redis) will help maintain consistent token state across servers
- Revocation Gap: The 10-15-minute polling interval introduces a brief window before revoked tokens are fully inactive, allowing limited use if compromised
- Database/Memory Management: As more tokens are stored in the cache and database, memory and storage usage grows. Periodic pruning of inactive tokens might be required

##### Token Pruning Strategy:
To manage cache memory and database efficiency, unused tokens are periodically pruned based on their activity:

1. Pruning Condition: Tokens with an `updated_at` timestamp older than the expiration period (e.g, 24 hours) are considered inactive and are eligible for deletion
2. Scheduled Pruning Job: During each database polling cycle, inactive tokens are deleted to optimize storage and memory

### HTTPS:
Both frontend and backend will be served over HTTPS, securing data transmissions between the client, frontend, and API

### Access Control:
Access to resources and actions is managed through role-based controls. User roles (Admin and Reviewer) are defined within the system and stored in JWT claims

---
## Deployment

The application will initially be hosted on a single server, with the frontend, backend, and database each running in separate Docker containers. Docker Compose will be used to orchestrate and manage these containers

---
## Future Improvements and Considerations

### Distributed Caching:
If the application scales and the volume of users grows, the in-memory cache for JWT revocation and token validation could potentially be moved to a distributed cache like Redis. This would ensure that multiple instances of the backend can share a consistent token state

### Rate Limiting & Throttling:
Implement rate limiting for API endpoints to protect against abuse or accidental overloading of the system

### Alternative JWT Expiration and Refresh Logic:
Implementing a refresh token system could be an alternative approach to provide a flexible and secure method for maintaining long user sessions

### Idempotency Keys:
Introducing idempotency keys ensures that repeated requests (e.g., retries due to network issues) produce the same result, preventing unintended side effects like duplicate transactions or operations