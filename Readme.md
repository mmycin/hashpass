# HashPass - Password Manager Tutorial

This tutorial will guide you through building a simple password manager in Rust.

## Prerequisites

- Rust installed on your system
- Basic knowledge of Rust programming
- Cargo package manager

## Step 1: Project Setup

1. Create a new Rust project:

```bash
cargo new hashpass
cd hashpass
```

2. Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
bcrypt = "0.15"
thiserror = "1.0"
```

3. Create the basic project structure:
   - `src/lib.rs`: Core password hashing functionality
   - `src/main.rs`: CLI interface

## Step 2: Core Functionality

The core functionality is implemented in `lib.rs` with two main functions:

- `hash_password`: Takes a password string and number of rounds, returns a hashed password
- `verify_password`: Verifies a password against a hash

Example usage:

```rust
use hashpass::{hash_password, verify_password};

// Hash a password
let hashed = hash_password("mypassword", 12)?;

// Verify the password
let is_valid = verify_password("mypassword", &hashed)?;
```

## Step 3: CLI Interface

The project includes a command-line interface for:
- Hashing passwords
- Verifying passwords against existing hashes

Usage examples will be covered in the following sections...
