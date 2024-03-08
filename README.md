# Sebden

This Rust project employs Domain-Driven Design (DDD) and Hexagonal Architecture (also known as Ports and Adapters pattern) to construct a user management system. Below is the documentation for the various components:
## Domain Layer (domain)
### User Entity (domain/src/user/user.rs)
Defines a User struct containing basic user information: id, username, email, password_hash, and bio. It provides a new method for creating new User instances.
### User Repository Interface (domain/src/user/repository.rs)
Defines a UserRepository trait specifying methods that a user repository should implement: get_users, get_user, and create_user. These methods are for fetching all users, fetching a single user by ID, and creating a new user, respectively.
## Infrastructure Layer (infrastructure)
### User Repository Implementation (infrastructure/src/repositories/user_repository_impl.rs)
Implements the UserRepository trait, using sqlx and a PostgreSQL database for data persistence. Includes operations for retrieving all users from the database, retrieving a user by ID, and inserting a new user into the database.
### Configuration Management (infrastructure/src/config/app_config.rs)
Defines an AppConfig struct for loading and initializing server and database configurations. If the configuration file does not exist, a default configuration will be created.
### Database Connection Pool (infrastructure/src/database.rs)
Provides a create_database_pool function to create a database connection pool based on the application configuration.
## Application Layer (application)
### User Service (application/src/user_service.rs)
Defines a UserService struct that encapsulates user-related business logic. It relies on UserRepository for data access.
### Factory Method (application/src/factories.rs)
Provides a create_user_service function for creating instances of UserService.
### Interface Adapter Layer (adapters)
### HTTP User Interface (adapters/src/http/user)
Includes route definitions (routes.rs), request handlers (handlers.rs), and data models (models.rs). Defines the logic for handling HTTP requests and mapping them to operations in the application service layer.
## API Server (api-server)
Main Program (api-server/src/main.rs)
Sets up the application's startup logic, including configuration loading, database connection pool creation, route setup, and HTTP server launch.
## Route Configuration (api-server/src/router.rs)
Defines the application's route configuration, mapping URL paths to specific request handlers.

---
This architecture, by decomposing the application into different layers (Domain, Infrastructure, Application, and Interface Adapter), achieves separation of concerns, enhancing code maintainability and extensibility. The use of DDD methodology guides the design, emphasizing the central role of domain logic. The Hexagonal Architecture allows the application to interact with external systems (such as databases and web interfaces) through ports and adapters, increasing the system's flexibility and testability.