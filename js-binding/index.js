"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const path = require("path");
const os = require('os');
const fs = require("fs");

class ModuleLoader {
    constructor() {
        this.module = null;
        this.moduleType = null;
        this.initialized = false;
    }

    loadNative() {
        let binaryPath;
        const platform = os.platform();
        
        switch (platform) {
            case 'darwin':
                binaryPath = path.join(__dirname, 'dist', 'darwin', 'index.node');
                break;
            case 'win32':
                binaryPath = path.join(__dirname, 'dist', 'win32', 'index.node');
                break;
            case 'linux':
                binaryPath = path.join(__dirname, 'dist', 'linux', 'index.node');
                break;
            default:
                throw new Error(`Unsupported platform: ${platform}`);
        }
        
        const modulePath = path.resolve(binaryPath);
        if (!fs.existsSync(modulePath)) {
            throw new Error(`Failed to find native module in: ${modulePath}`);
        }
        return require(modulePath);
    }

    async loadWasm() {
        const wasmPath = path.join(__dirname, 'dist', 'wasm');
        
        // Check if WASM files exist
        const wasmJsPath = path.join(wasmPath, 'gasless.js'); // Replace with actual name
        const wasmFilePath = path.join(wasmPath, 'gasless_bg.wasm'); // Replace with actual name
        
        if (!fs.existsSync(wasmJsPath) || !fs.existsSync(wasmFilePath)) {
            throw new Error('WASM files not found');
        }
        
        // Load the WASM module
        const wasmModule = require(wasmJsPath);
        
        // If the WASM module needs initialization (common with wasm-pack)
        if (typeof wasmModule.default === 'function') {
            // Initialize the WASM module
            await wasmModule.default(fs.readFileSync(wasmFilePath));
            return wasmModule;
        }
        
        return wasmModule;
    }

    async initialize() {
        if (this.initialized) {
            return;
        }

        try {
            // Try native first
            this.module = this.loadNative();
            this.moduleType = 'native';
        } catch (nativeError) {
            console.warn(`Failed to load native module: ${nativeError.message}`);
            console.warn('Falling back to WASM...');
            
            try {
                // Fallback to WASM
                this.module = await this.loadWasm();
                this.moduleType = 'wasm';
            } catch (wasmError) {
                throw new Error(
                    `Failed to load both native and WASM modules:\n` +
                    `Native: ${nativeError.message}\n` +
                    `WASM: ${wasmError.message}`
                );
            }
        }
        
        this.initialized = true;
    }

    async mineGasForTransaction(...args) {
        await this.initialize();
        
        if (!this.module || !this.module.mineGasForTransaction) {
            throw new Error('mineGasForTransaction function not available');
        }
        
        return this.module.mineGasForTransaction(...args);
    }

    getModuleType() {
        return this.moduleType;
    }
}

// Create a singleton instance
const moduleLoader = new ModuleLoader();

// Export synchronous version (maintains backward compatibility)
function mineGasForTransaction(...args) {
    if (!moduleLoader.initialized) {
        // For synchronous calls, we can only try native
        try {
            moduleLoader.module = moduleLoader.loadNative();
            moduleLoader.moduleType = 'native';
            moduleLoader.initialized = true;
        } catch (error) {
            throw new Error(
                `Synchronous loading failed. Use mineGasForTransactionAsync for WASM fallback. Error: ${error.message}`
            );
        }
    }
    
    return moduleLoader.module.mineGasForTransaction(...args);
}

// Export asynchronous version (supports both native and WASM)
async function mineGasForTransactionAsync(...args) {
    return await moduleLoader.mineGasForTransaction(...args);
}

// Exports
exports.mineGasForTransaction = mineGasForTransaction;
exports.mineGasForTransactionAsync = mineGasForTransactionAsync;

// Optional: Export module type for debugging
Object.defineProperty(exports, '_moduleType', {
    get() {
        return moduleLoader.getModuleType();
    }
});