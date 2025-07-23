'use client'

import dynamic from 'next/dynamic'
import { useEffect, useState } from 'react';
import AsciiDisplay from './AsciiDisplay';
import { useWasm } from '@/contexts/WasmContext';

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
    const wasm = useWasm();

    useEffect(() => {
        if (!wasm || downsample_rate <= 0 || downsample_rate == null) {
            return;
        }

        try {
            const renderResult = wasm.render(
                image_bytes,
                downsample_rate,
                edge_sobel_threshold,
                ascii_chars_edge_str,
                ascii_chars_gray_str
            );
            setResult(renderResult);
            console.log('WASM render result:', renderResult);
        } catch (error) {
            console.error("Failed to render ASCII art:", error);
            setResult(null);
        }

    }, [wasm, image_bytes, downsample_rate, edge_sobel_threshold, ascii_chars_edge_str, ascii_chars_gray_str]);

    return (
        <div>
            <>
                {result ? (<AsciiDisplay asciiArt={result} />) : (
                    <div className="text-center text-gray-500">
                        <p>Processing...</p>
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