# 🎰 Solana CryptoMineX DApp

A **provably fair CryptoMineX system on Solana** using **Anchor**, **Switchboard VRF**, **Axum (Rust backend)**, and **React frontend**.

---

## 🚀 Features

* 🎟️ Buy tickets (1–25 numbers)
* ⏱️ Automated round lifecycle
* 🎲 Verifiable randomness using Switchboard VRF
* 🏆 Winner selection on-chain
* 💰 Automatic reward distribution
* 🔄 Fully automated game rounds (no manual intervention)
* 📊 Real-time UI updates

---

## 🧱 Tech Stack

### 🟣 Blockchain

* Anchor (Solana Smart Contracts)
* Solana Web3.js
* Switchboard VRF (Randomness)

### 🟢 Backend

* Rust + Axum
* MongoDB (optional for indexing)
* Tokio (async runtime)

### 🔵 Frontend

* React + TypeScript
* TailwindCSS
* Framer Motion (animations)
* Solana Wallet Adapter

---

## 📁 Project Structure

```
.
├── program/            # Anchor smart contract
├── backend/            # Axum (Rust) server
├── frontend/           # React app
├── scripts/            # Deployment & utilities
```

---

## ⚙️ Smart Contract (Anchor)

### Core Accounts

* `GlobalState`
* `Round`
* `Ticket`
* `UserTicket`

### Round Lifecycle

1. 🟢 Create Round
2. 🎟️ Users buy tickets
3. 🎲 Commit randomness (VRF)
4. 🏆 Reveal winner
5. 💰 Distribute rewards
6. 🔴 Close round

---

## 🔄 Backend (Axum)

### Run server

```bash
cd backend
cargo run
```

### Features

* Solana program integration
* REST APIs for frontend
* Automated round lifecycle (Tokio scheduler)
* VRF integration

---

## 🌐 Frontend

### Run frontend

```bash
cd frontend
npm install
npm run dev
```

### Features

* Wallet connect (Phantom, etc.)
* Ticket selection UI (1–25 grid)
* Countdown timer
* Winner animation 🎉
* Live round updates

---

## 🎲 Game Flow

```
Start Round
   ↓
Users Buy Tickets
   ↓
Round Ends
   ↓
Commit VRF
   ↓
Reveal Winner
   ↓
Distribute Rewards
   ↓
Close Round
   ↓
Repeat 🔁
```

---

## 🧠 Key Concepts

### 🔐 Provably Fair Randomness

Using **Switchboard VRF**, ensuring:

* No manipulation
* On-chain verification

### ⚡ Automation

* Backend scheduler handles full lifecycle
* No manual admin actions required

---

## 🛠️ Environment Setup

### 1. Install Dependencies

* Rust
* Node.js
* Solana CLI
* Anchor

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor anchor-cli
```

---

### 2. Configure Environment

Create `.env`:

```env
RPC_URL=https://api.devnet.solana.com
PRIVATE_KEY=your_keypair
PORT=5200
```

---

### 3. Deploy Program

```bash
anchor build
anchor deploy
```

---

## 🧪 Testing

```bash
anchor test
```

---

## 📸 UI Preview

* 🎰 CryptoMineX Dashboard
* 🎟️ Number Grid
* ⏱️ Countdown Timer
* 🏆 Winner Highlight Animation

---

## 🔥 Future Improvements

* 🪙 SPL Token support (instead of SOL)
* 📱 Mobile UI optimization
* 📊 Analytics dashboard
* 🧾 Transaction history
* 🧠 AI-based lucky number suggestion 😉

---

## 🤝 Contributing

Pull requests are welcome!

1. Fork the repo
2. Create your branch
3. Commit changes
4. Open PR

---

## 📜 License

MIT License

---

## 👨‍💻 Author

Built with ❤️ on Solana
