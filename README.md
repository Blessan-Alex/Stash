# Stash - Your Smart, Secure Piggy Bank

**Guard your cash, grow your stash**

Stash is a smart piggy bank that securely stores your money while growing your savings through incentivized lock-in periods. Built on the Internet Computer Protocol (ICP) blockchain, Stash provides fast, secure, and low-cost financial services for everyone.

## 🚨 Problem We're Solving
75% of Indians do not have emergency funds. Stash turns your savings into a secure, self-growing financial reserve, so you're always ready for life's surprises.

## 🔑 Key Features

- **Secure Blockchain Savings**: Store your funds on the ICP blockchain for maximum security
- **Interest on Deposits**: Earn rewards based on customizable lock-in periods
- **Flexible Lock-up Options**: Choose from 3, 6, or 12-month plans with increasing interest rates
- **Real-time Balance Tracking**: Monitor your savings and growth in real-time
- **Transparent Transaction History**: View detailed records of all transactions
- **Early Withdrawal Option**: Access your funds before the lock-in period (with penalty)
- **Secure Authentication**: Internet Identity integration for secure logins

## 🛠️ Technology Stack

### Frontend
- React (JavaScript/JSX)
- Vite (Build tool and development server)
- React Router for navigation
- CSS for styling

### Backend
- Internet Computer Protocol (ICP) Blockchain
- Rust for smart contract development
- Candid for interface definitions

## 📋 How to Run the Project

1. **Install Prerequisites**:
   ```bash
   # Install dfx (Internet Computer SDK)
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

2. **Clone and Setup**:
   ```bash
   git clone <repository-url>
   cd piggybank
   ```

3. **Start the Local Replica**:
   ```bash
   dfx start --clean
   ```

4. **Deploy the Canisters**:
   ```bash
   # In a new terminal
   dfx deploy
   ```

5. **Start the Frontend**:
   ```bash
   cd src/piggybank_frontend
   npm install
   npm start
   ```

6. **Access the Application**:
   - Frontend: http://localhost:3001/
   - Backend canister: Check the URLs shown after deployment

## 🗺️ Roadmap

### Short Term
- Finalize stable coin integration and UI polish
- Conduct comprehensive testing and deploy on testnet

### Long Term
- Integrate real fiat currency (INR to USDC stable coin) using Transack
- Expand reward features and user engagement mechanisms
- Develop mobile applications for wider accessibility

## 💡 Technical Implementation

- Built on the Internet Computer Protocol for decentralized storage and computation
- Smart contracts written in Rust for efficiency and security
- User authentication via Internet Identity for secure and private access
- Low computational overhead for reduced costs
- High-performance blockchain for faster transactions

---

Stash - The smart way to save for tomorrow, today.
