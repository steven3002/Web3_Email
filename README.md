# 📩 Stylus Email DApp

Stylus Email DApp is a decentralized email application built on Arbitrum Stylus using Rust. It allows users to send, receive, and manage messages securely on the blockchain while offering features like blocking unwanted senders.

## 🚀 Features
- **Send Emails:** Users can send messages to other addresses.
- **Receive Emails:** Retrieve messages sent to your address.
- **Block Users:** Prevent specific addresses from sending emails.
- **View Message History:** Access previously received messages with timestamps.

## 📜 Smart Contract Overview

The contract defines a decentralized email system using Solidity-like storage structures in Rust via `sol_storage!`.

### 📂 Storage Structure
- `system`: Stores messages for each address.
- `len_`: Tracks the number of messages received.
- `block_`: Maintains a list of blocked addresses.

### 🔧 Functions


#### Working Contract Address
```rust
0x8e76d792920bcee1debb604a1b372fb79fbd59e2
```


#### Working Abi 
```rust
interface IApp {
    function sendEmail(address to, string calldata message) external;

    function getLen() external view returns (uint256);

    function getMail(uint128 start, uint128 end) external view returns (address,string,uint64)[] memory;

    function blocking(address block_ad, bool status) external;

    function blockingStatus(address b_s) external view returns (bool);
}
```

#### 📧 Sending an Email
```rust
pub fn send_email(&mut self, to: Address, message: String)
```
Sends an email to the specified address if the recipient has not blocked the sender.

#### 📬 Get Total Emails
```rust
pub fn get_len(&self) -> U256
```
Returns the number of emails received by the caller.

#### 📜 Retrieve Emails
```rust
pub fn get_mail(&self, start: u128, end: u128) -> Vec<(Address, String, u64)>
```
Fetches a range of messages from the caller's inbox.

#### 🚫 Block/Unblock Users
```rust
pub fn blocking(&mut self, block_ad: Address, status: bool)
```
Allows users to block or unblock an address.

#### 🔍 Check Block Status
```rust
pub fn blocking_status(&self, b_s: Address) -> bool
```
Checks if a specific address is blocked by the caller.

## 🔨 Installation & Deployment

### Prerequisites
- Rust & Cargo installed
- Arbitrum Stylus SDK setup

### Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/your-repo/stylus-email-dapp.git
   cd stylus-email-dapp
   ```

2. Export ABI:
   ```sh
   cargo stylus export-abi
   ```
4. Deploy to Arbitrum Stylus Testnet/Mainnet using your preferred method.


