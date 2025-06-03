'use client'

import dynamic from 'next/dynamic'
import { useEffect, useState } from 'react';
import AsciiDisplay from './AsciiDisplay';

interface WasmRenderProps {
    image_bytes: Uint8Array
    downsample_rate: number
    edge_sobel_threshold: number
    ascii_chars_edge_str: string
    ascii_chars_gray_str: string
}

interface AsciiCharInfo {
    char: string;
    r: number;
    g: number;
    b: number;
}

interface AsciiArtData {
    lines: AsciiCharInfo[][];
    width: number;
    height: number;
}


const AsciiComponent = ({ image_bytes, downsample_rate, edge_sobel_threshold, ascii_chars_edge_str, ascii_chars_gray_str }: WasmRenderProps) => {
    const [result, setResult] = useState<AsciiArtData | null>(null);
    const [wasm, setWasm] = useState<any>(null);

    useEffect(() => {
        async function loadWasm() {
            try {
                const wasmModule = await import("@/../public/pkg/image2ascii");
                setWasm(wasmModule);
            } catch (err) {
                console.error("Failed to load WASM:", err);
            }
        }
        loadWasm();
    }, []);

    useEffect(() => {
        if (downsample_rate <= 0 || downsample_rate == null) {
            return;
        }
        if (!wasm) {
            console.warn('WASM module not loaded yet');
            return;
        } else {
            const result = wasm.render(
                image_bytes,
                downsample_rate,
                edge_sobel_threshold,
                ascii_chars_edge_str,
                ascii_chars_gray_str
            );
            setResult(result);
            console.log('WASM render result:', result);
        }
    }, [image_bytes, downsample_rate, edge_sobel_threshold, ascii_chars_edge_str, ascii_chars_gray_str]);

    return (
        <div>
            <>
                {result ? (<AsciiDisplay asciiArt={result} />) : (
                    <div className="text-center text-gray-500">
                        <p>Loading...</p>
                    </div>
                )}
            </>
        </div>
    )
}

const WasmImage2AsciiComponent = dynamic(() => Promise.resolve(AsciiComponent), {
    ssr: false
});

export default WasmImage2AsciiComponent;