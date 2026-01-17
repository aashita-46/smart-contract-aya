# Role-Based On-Chain Counter (Rust Smart Contract)

 ## Contract Summary
This smart contract implements a simple on-chain counter with role-based access control.
Only authorized addresses (admins) are allowed to increment or decrement the counter.

The contract demonstrates core Rust smart contract fundamentals such as state management,
access control, safe mutation, and explicit error handling.

## Why This Design
Counters are a minimal but powerful abstraction in Web3.
They appear in governance systems, rate limiters, voting mechanisms, and protocol parameters.

This design was chosen to:
- Keep logic simple and auditable
- Clearly demonstrate ownership and permissions
- Be easy for beginners to understand and extend

## State & Flow

### State
- `count: i32` — the current counter value
- `admins: Vec<Addr>` — list of authorized addresses

### Initialization
- The deployer becomes the first admin
- Counter starts at zero

### Execute Messages
- `Increment {}` → increases count by 1 (admin only)
- `Decrement {}` → decreases count by 1 (admin only)
- `AddAdmin { address }` → adds a new admin (admin only)

### Query Messages
- `GetCount {}` → returns current count
- `IsAdmin { address }` → checks admin status

## Security Checks
- Only admins can mutate state
- Explicit authorization checks on every execute call
- Prevents unauthorized access with clear custom errors

## How to Build & Test
```bash
cargo wasm
cargo test
