'use client';
import React from 'react';

interface ParameterControlsProps {
    params: { [key: string]: any };
    onParamChange: (paramName: string, value: string | number) => void;
}

// Define your parameters structure for better typing and iteration
const paramDefinitions = [
    { name: 'downsampleRate', label: 'Downsample Rate', type: 'number', defaultValue: 2, min: 1, max: 10 },
    { name: 'edgeSobelThreshold', label: 'Edge Sobel Threshold', type: 'number', defaultValue: 100, min: 0, max: 255 },
    { name: 'asciiCharsEdge', label: 'ASCII Chars for Edges', type: 'text', defaultValue: ' -/|\\' },
    { name: 'asciiCharsGray', label: 'ASCII Chars for Gray', type: 'text', defaultValue: '@?OPoc:. ' },
];


export default function ParameterControls({ params, onParamChange }: ParameterControlsProps) {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            {paramDefinitions.map(paramDef => (
                <div key={paramDef.name}>
                    <label htmlFor={paramDef.name} className="block mb-1 text-sm font-medium text-gray-900 dark:text-white">
                        {paramDef.label}
                    </label>
                    <input
                        type={paramDef.type}
                        id={paramDef.name}
                        name={paramDef.name}
                        value={params[paramDef.name] ?? paramDef.defaultValue}
                        min={paramDef.min}
                        max={paramDef.max}
                        onChange={(e) => onParamChange(paramDef.name, paramDef.type === 'number' ? parseFloat(e.target.value) : e.target.value)}
                        className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    />
                </div>
            ))}
        </div>
    );
}