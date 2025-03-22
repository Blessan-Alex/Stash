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

# PiggyBank - Internet Computer DeFi Platform

A decentralized finance (DeFi) platform built on the Internet Computer blockchain that allows users to deposit INR and earn interest through PIGGYUSD tokens.

## Features

- Internet Identity Authentication
- INR to PIGGYUSD token conversion
- Flexible deposit periods with different interest rates
- Early withdrawal option with penalty
- Real-time balance tracking
- Modern and responsive UI

## Tech Stack

### Frontend
- React
- Vite
- Internet Identity
- DFINITY Agent
- React Router DOM

### Backend
- Rust
- Internet Computer (DFINITY)
- Candid Interface

## Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable version)
- DFINITY SDK (dfx)
- Internet Identity

## Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/piggybank.git
cd piggybank
```

2. Install frontend dependencies:
```bash
cd src/piggybank_frontend
npm install
```

3. Start the frontend development server:
```bash
npm start
```

4. Deploy the backend canister:
```bash
cd ../piggybank_backend
dfx deploy
```

## Environment Variables

Create a `.env` file in the frontend directory with the following variables:
```
VITE_CANISTER_ID=your_canister_id
VITE_II_CANISTER_ID=your_ii_canister_id
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- DFINITY Foundation
- Internet Computer Community
- React Team
- Rust Team

# `piggybank`

# `piggybank`

Welcome to your new `piggybank` project and to the Internet Computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `piggybank`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd piggybank/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
