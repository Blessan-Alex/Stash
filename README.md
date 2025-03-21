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

# Stash - Your PiggyBank on STEROIDS

Stash is a revolutionary decentralized savings platform built on the Internet Computer blockchain. It transforms traditional savings into an engaging, rewarding experience with modern technology and innovative incentives.

## 🌟 Key Features

### 💰 Smart Savings
- **Automated Savings**: Set up recurring deposits and watch your money grow
- **Goal Tracking**: Create and monitor multiple savings goals with visual progress
- **Smart Notifications**: Get alerts for milestones and opportunities

### 🎁 Rewards & Incentives
- **Earn While You Save**: Get rewarded for maintaining consistent saving habits
- **Achievement System**: Unlock badges and rewards for reaching savings milestones
- **Community Challenges**: Participate in group savings challenges for extra rewards

### 🔒 Security & Privacy
- **Blockchain Security**: Your funds are protected by the Internet Computer's advanced cryptography
- **Self-Custody**: You maintain full control of your assets
- **Privacy-First**: Your financial data stays private and secure

### 💎 Premium Features
- **Savings Analytics**: Track your spending patterns and get personalized insights
- **Investment Options**: Grow your savings with integrated investment opportunities
- **Family Accounts**: Manage shared savings goals with family members

## 🚀 Why Choose Stash?

- **Modern Interface**: Clean, intuitive design for a seamless user experience
- **Instant Transactions**: Lightning-fast deposits and withdrawals
- **No Hidden Fees**: Transparent fee structure with no surprise charges
- **Cross-Platform**: Access your savings from any device, anywhere

## 🛠️ Technical Stack

- **Frontend**: React with Vite
- **Backend**: Internet Computer (Rust)
- **Authentication**: Internet Identity
- **Smart Contracts**: Motoko

## 🔐 Getting Started

1. Visit [Stash App](https://your-app-url.ic0.app)
2. Connect with Internet Identity
3. Create your first savings goal
4. Start earning rewards!

## 📱 Supported Platforms

- Web Browser (Chrome, Firefox, Safari)
- Mobile Web Browser
- Progressive Web App (PWA) support coming soon

## 🔮 Future Roadmap

- [ ] Mobile App Development
- [ ] Advanced Investment Features
- [ ] Social Savings Features
- [ ] AI-Powered Savings Recommendations
- [ ] Integration with Traditional Banking

## 🤝 Contributing

We welcome contributions! Please read our contributing guidelines before submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Built with ❤️ on the Internet Computer

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
