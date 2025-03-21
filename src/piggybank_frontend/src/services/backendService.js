import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../declarations/piggybank_backend";
import { Principal } from "@dfinity/principal";

const CANISTER_ID = import.meta.env.VITE_CANISTER_ID_PIGGYBANK_BACKEND;
const HOST = import.meta.env.VITE_DFX_NETWORK === "local" ? "http://127.0.0.1:4943" : "https://ic0.app";

class BackendService {
    constructor() {
        this.actor = null;
        this.initialized = false;
        console.log("BackendService initialized with canister ID:", CANISTER_ID);
        console.log("Using host:", HOST);
    }

    async init() {
        if (this.initialized) {
            return true;
        }

        try {
            console.log("Initializing backend service...");
            const agent = new HttpAgent({ 
                host: HOST,
                fetchRootKey: true
            });
            
            if (import.meta.env.VITE_DFX_NETWORK === "local") {
                console.log("Fetching root key for local development...");
                await agent.fetchRootKey();
            }

            console.log("Creating actor with canister ID:", CANISTER_ID);
            this.actor = Actor.createActor(idlFactory, {
                agent,
                canisterId: CANISTER_ID,
            });

            // Test the connection
            const testResult = await this.actor.get_balance();
            console.log("Test connection result:", testResult);

            this.initialized = true;
            console.log("Backend service initialized successfully");
            return true;
        } catch (error) {
            console.error("Failed to initialize backend service:", error);
            this.initialized = false;
            return false;
        }
    }

    async ensureInitialized() {
        if (!this.initialized) {
            await this.init();
        }
        if (!this.actor) {
            throw new Error("Failed to initialize actor");
        }
    }

    async getBalance() {
        try {
            await this.ensureInitialized();
            console.log("Getting balance...");
            const result = await this.actor.get_balance();
            console.log("Balance result:", result);
            if ("Ok" in result) {
                return result.Ok;
            } else if (result.Err.includes("User balance not found")) {
                // Initialize user balance if not found
                await this.mintTokens(BigInt(0), { ThreeMonths: null });
                return await this.getBalance(); // Retry getting balance
            } else {
                throw new Error(result.Err);
            }
        } catch (error) {
            console.error("Error getting balance:", error);
            throw error;
        }
    }

    async mintTokens(amount, lockPeriod) {
        try {
            await this.ensureInitialized();
            // Ensure amount is BigInt
            const mintAmount = BigInt(amount);
            console.log("Minting tokens...", { 
                amount: mintAmount.toString(),
                amountType: typeof mintAmount,
                isBigInt: mintAmount instanceof BigInt,
                lockPeriod 
            });
            const result = await this.actor.mint_tokens(mintAmount, lockPeriod);
            console.log("Mint result:", result);
            if ("Ok" in result) {
                return result.Ok;
            } else {
                throw new Error(result.Err);
            }
        } catch (error) {
            console.error("Error minting tokens:", error);
            throw error;
        }
    }

    async burnTokens(amount) {
        if (!this.initialized) {
            throw new Error("Service not initialized");
        }

        try {
            // Ensure amount is BigInt
            const burnAmount = BigInt(amount);
            console.log("Attempting to burn tokens:", {
                amount: burnAmount.toString(),
                amountType: typeof burnAmount,
                isBigInt: burnAmount instanceof BigInt
            });
            
            // First check the current balance
            const currentBalance = await this.getBalance();
            console.log("Current balance before burn:", {
                total: currentBalance?.total_balance?.toString(),
                available: currentBalance?.available_balance?.toString(),
                required: burnAmount.toString()
            });
            
            if (!currentBalance || !currentBalance.total_balance || currentBalance.total_balance < burnAmount) {
                console.error("Insufficient balance. Required:", burnAmount.toString(), "Available:", currentBalance?.total_balance?.toString());
                throw new Error("Insufficient balance");
            }

            const result = await this.actor.burn_tokens(burnAmount);
            console.log("Burn tokens result:", result);

            if (result.Err) {
                console.error("Error burning tokens:", result.Err);
                throw new Error(result.Err);
            }

            // Verify the balance after burn
            const newBalance = await this.getBalance();
            console.log("New balance after burn:", {
                total: newBalance?.total_balance?.toString(),
                available: newBalance?.available_balance?.toString()
            });

            return true;
        } catch (error) {
            console.error("Error in burnTokens:", error);
            throw error;
        }
    }

    async applyRewards() {
        try {
            await this.ensureInitialized();
            console.log("Applying rewards...");
            const result = await this.actor.apply_rewards();
            console.log("Apply rewards result:", result);
            if ("Ok" in result) {
                return result.Ok;
            } else {
                throw new Error(result.Err);
            }
        } catch (error) {
            console.error("Error applying rewards:", error);
            throw error;
        }
    }
}

export const backendService = new BackendService(); 