'use client';

import { createContext, useContext, useEffect, useState, ReactNode } from 'react';
import * as wasm_js from '@/../pkg/image2ascii.js';

// Define the type for the WASM module functions we need
type WasmModule = typeof wasm_js;

// Create the context with an initial null value
const WasmContext = createContext<WasmModule | null>(null);

// Custom hook for easy access to the WASM module
export const useWasm = () => {
    const context = useContext(WasmContext);
    if (!context) {
        throw new Error('useWasm must be used within a WasmProvider');
    }
    return context;
};

// Provider component that loads the WASM module
export const WasmProvider = ({ children }: { children: ReactNode }) => {
    const [wasm, setWasm] = useState<WasmModule | null>(null);

    useEffect(() => {
        const loadWasm = async () => {
            try {
                // Fetch the WASM binary
                const response = await fetch('/pkg/image2ascii_bg.wasm');
                const bytes = await response.arrayBuffer();
                
                // Initialize the WASM module
                wasm_js.initSync(bytes);
                
                // Set the loaded module to state
                setWasm(wasm_js);
                console.log('WASM module loaded and initialized.');
            } catch (error) {
                console.error('Failed to load WASM module:', error);
            }
        };

        loadWasm();
    }, []); // Empty dependency array ensures this runs only once

    if (!wasm) {
        // You might want to render a loading indicator here
        return <div>Loading WebAssembly Module...</div>;
    }

    return (
        <WasmContext.Provider value={wasm}>
            {children}
        </WasmContext.Provider>
    );
};