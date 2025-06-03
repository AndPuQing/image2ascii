'use client'

import dynamic from 'next/dynamic'
import { useEffect, useState } from 'react';
import AsciiDisplay from './AsciiDisplay';
import * as wasm_js from '@/../pkg/image2ascii.js';

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

    useEffect(() => {
        if (downsample_rate <= 0 || downsample_rate == null) {
            return;
        }
        fetch("@/../pkg/image2ascii_bg.wasm").then(response => {
            return response.arrayBuffer();
        }).then(bytes => {
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
            const _wasm_binary = wasm_js.initSync(bytes);
            const result = wasm_js.render(
                image_bytes,
                downsample_rate,
                edge_sobel_threshold,
                ascii_chars_edge_str,
                ascii_chars_gray_str
            );
            setResult(result);
            console.log('WASM render result:', result);
        })
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