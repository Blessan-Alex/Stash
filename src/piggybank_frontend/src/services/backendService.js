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
            console.log("Minting tokens...", { amount, lockPeriod });
            const result = await this.actor.mint_tokens(amount, lockPeriod);
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
        try {
            await this.ensureInitialized();
            console.log("Burning tokens...", { amount });
            const result = await this.actor.burn_tokens(amount);
            console.log("Burn result:", result);
            if ("Ok" in result) {
                return result.Ok;
            } else {
                throw new Error(result.Err);
            }
        } catch (error) {
            console.error("Error burning tokens:", error);
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