// components/AsciiDisplay.tsx
'use client';

import React from "react";

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

interface AsciiDisplayProps {
    asciiArt: AsciiArtData | null;
}

export default function AsciiDisplay({ asciiArt }: AsciiDisplayProps) {
    if (!asciiArt) {
        return null;
    }

    const { lines, width, height } = asciiArt;
    // A more robust estimation for monospace fonts (e.g., 10x16 or similar ratio)
    const charWidth = 10;
    const charHeight = 16; // A common height for a 10px-wide character
    const viewBoxWidth = width * charWidth;
    const viewBoxHeight = height * charHeight;

    return (
        <div className="w-full h-full overflow-hidden bg-gray-900 dark:bg-black rounded-md p-2 flex items-center justify-center">
            <svg
                viewBox={`0 0 ${viewBoxWidth} ${viewBoxHeight}`}
                preserveAspectRatio="xMidYMid meet"
                style={{ width: '100%', height: '100%' }}
            >
                {/* Add a background to the SVG to ensure contrast */}
                <rect width="100%" height="100%" fill="#1e1e1e" />
                <text
                    x="0"
                    y="0"
                    style={{
                        fontFamily: 'monospace',
                        fontSize: `${charHeight}px`,
                        whiteSpace: 'pre',
                    }}
                >
                    {lines.map((row, rowIndex) => (
                        <tspan key={rowIndex} x="0" dy={`${charHeight}px`}>
                            {row.map((charInfo, charIndex) => (
                                <tspan
                                    key={charIndex}
                                    fill={`rgb(${charInfo.r}, ${charInfo.g}, ${charInfo.b})`}
                                >
                                    {charInfo.char}
                                </tspan>
                            ))}
                        </tspan>
                    ))}
                </text>
            </svg>
        </div>
    );
}