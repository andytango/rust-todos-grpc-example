# Rust Service Template

This repository provides a template for building a Rust service using Tonic for
gRPC, SQLx for database interaction (PostgreSQL), and dotenv for environment
variable management.

## Introduction

This template offers a basic structure for a Rust service, including database
connection management, environment configuration, and gRPC setup. It's designed
to be a starting point for building more complex services.

## Dependencies

- [Rust toolchain](https://www.rust-lang.org/tools/install]). Rust is generally
  backwards compatible, if you have encounter issues then v1.84.0 is
  known to work.
- A vanilla [PostgreSQL database](https://www.postgresql.org/download/), version
  15 or above.
- For linux (debian or ubuntu), the following APT packages: `build-essential`,
  `lib-ssl-dev`, `pkg-config`
- [Protocol Buffer Compiler](https://grpc.io/docs/protoc-installation). The
  version from APT appears to work fine.Rust toolchain. On mac OS this can be
  installed using [homebrew](https://brew.sh).

## Codebase Structure

This project follows a modular structure to organize the code and improve
maintainability. Here's a brief overview of the key directories and modules:

* **`src/bin/server.rs`**: This file is the entry point for the application. It
  initializes the common parts of the application, sets up the gRPC server, and
  starts listening for incoming requests.
* **`src/lib.rs`**:  Contains the core logic of the service. This is where the
  gRPC server is defined and interactions with the database occur.
* **`src/lib/common.rs`**:  Houses common utility functions and types used
  across the project, promoting code reuse and consistency. For example, this
  includes logging and environment variable utilities.
* **`src/lib/database.rs`**:  Handles database interactions, including
  connection management and schema migrations using SQLx. This module abstracts
  database operations, making it easier to change database implementations or
  configurations if needed.
* **`src/lib/services`**: This directory contains modules related to specific
  gRPC services. For example, `src/lib/services/todos` houses the implementation
  of the To-Do service, including the gRPC service definition and the business
  logic for managing to-do items.
    * **`src/lib/services/todos/common.rs`**: Contains utilities related to the
      To-Do service. For example, mapping database row format into the protocol
      buffer format used by gRPC.
    * **`src/lib/services/todos/create.rs`**: Contains the implementation to
      handle creating a new To-Do item. Following this structure the To-Do
      service also has modules named  `delete.rs`, `get.rs`, `list.rs` and
      `update.rs` implementing the various gRPC server methods.
* **`src/proto`**:  Contains the Protocol Buffer (protobuf) definitions for the
  gRPC services. These files define the service interface and the structure of
  the data exchanged between the client and server.
* **`tests`**: This directory includes tests for different modules and
  functionalities, ensuring code quality and reliability.
* **`migrations`**: This directory contains SQL migration files used by
  `sqlx migrate` to manage the database schema. These migrations allow for easy
  schema updates and rollbacks.

This modular design separates concerns and makes it easier to maintain and
extend the application.

The use of well-defined modules and the `services` directory allows you to
easily add new gRPC services without impacting existing code, keeping the
project scalable and manageable.

## Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/rust-service-template.git
   ```

2. **Install dependencies:**

You will need to install the sqlx command line tool to run the database scripts:
```bash
cargo install sqlx-cli
```

3. **`.env` File:**

The `.env.example` contains all the environment variables you will need, 
along with documentation for each one.

4. **Database Management:**

Once you have updated the `.env` file with the database connection string, you 
can use the following commands to set up the database:

```
sqlx database create
sqlx migrate run
```

5. **Running the Server:**

After setting up the database and the `.env` file, you can run the server:

```bash
cargo run --bin server
```

## Database Scripts

* **`sqlx database create`**: Create a database based on the DATABASE\_URL
  (useful if it doesn't exist yet).
* **`sqlx database drop`**: Drops the database associated with the
  DATABASE\_URL, use with extreme care.

* **`sqlx migrate add <name>`**: Adds a new migration.
* **`sqlx migrate run`**: Applies all pending migrations.
* **`sqlx migrate info`**: Shows the current migration status.
* **`sqlx migrate revert`**: Reverts the last applied migration.

Refer to the SQLx documentation for more detailed information about the CLI and
its capabilities.