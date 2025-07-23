'use client';
import React from 'react';

interface ParameterControlsProps {
    params: { [key: string]: any }; // eslint-disable-line @typescript-eslint/no-explicit-any
    onParamChange: (paramName: string, value: string | number) => void;
    downsampleRateRange: { min: number; max: number };
}

// Define your parameters structure for better typing and iteration
const paramDefinitions = [
    { name: 'downsampleRate', label: 'Detail Level', type: 'range', defaultValue: 2 },
    { name: 'edgeSobelThreshold', label: 'Edge Threshold', type: 'range', defaultValue: 100, min: 0, max: 255 },
    { name: 'asciiCharsEdge', label: 'Edge Characters', type: 'text', defaultValue: ' -/|\\' },
    { name: 'asciiCharsGray', label: 'Grayscale Characters', type: 'text', defaultValue: '@?OPoc:. ' },
];


export default function ParameterControls({ params, onParamChange, downsampleRateRange }: ParameterControlsProps) {
    return (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
            <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">Adjust Parameters</h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-4">
                {paramDefinitions.map(paramDef => {
                    const min = paramDef.name === 'downsampleRate' ? downsampleRateRange.min : paramDef.min;
                    const max = paramDef.name === 'downsampleRate' ? downsampleRateRange.max : paramDef.max;

                    return (
                        <div key={paramDef.name} className={paramDef.type === 'text' ? 'sm:col-span-2' : ''}>
                            <label htmlFor={paramDef.name} className="flex justify-between items-center mb-1 text-sm font-medium text-gray-700 dark:text-gray-300">
                                <span>{paramDef.label}</span>
                                {paramDef.type === 'range' && <span className="text-blue-600 dark:text-blue-400 font-semibold">{params[paramDef.name]}</span>}
                            </label>
                            <input
                                type={paramDef.type}
                                id={paramDef.name}
                                name={paramDef.name}
                                value={params[paramDef.name] ?? paramDef.defaultValue}
                                min={min}
                                max={max}
                                onChange={(e) => onParamChange(paramDef.name, paramDef.type === 'range' ? parseFloat(e.target.value) : e.target.value)}
                                className={
                                    paramDef.type === 'range'
                                        ? "w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                                        : "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                }
                            />
                        </div>
                    )
                })}
            </div>
        </div>
    );
}