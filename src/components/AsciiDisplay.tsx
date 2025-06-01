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

    return (
        <div className="p-2 sm:p-4 rounded-lg shadow-inner overflow-x-auto">
            <pre
                className="text-[10px] sm:text-xs leading-none" // 
            >
                {asciiArt.lines.map((row, rowIndex) => (
                    <React.Fragment key={rowIndex}>
                        {row.map((charInfo, charIndex) => (
                            <span
                                key={charIndex}
                                style={{ color: `rgb(${charInfo.r}, ${charInfo.g}, ${charInfo.b})` }}
                            >
                                {charInfo.char === ' ' ? '\u00A0' : charInfo.char}
                            </span>
                        ))}

                        {rowIndex < asciiArt.lines.length - 1 && '\n'}
                    </React.Fragment>
                ))}
            </pre>
        </div>
    );
}