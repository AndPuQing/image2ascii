'use client';

import { useState, useEffect, useCallback } from 'react';

const initialParams = {
    downsampleRate: 2,
    edgeSobelThreshold: 100,
    asciiCharsEdge: ' -/|\\',
    asciiCharsGray: '@?OPoc:. ',
};

export const useAsciiConversion = () => {
    const [imageFile, setImageFile] = useState<File | null>(null);
    const [previewUrl, setPreviewUrl] = useState<string | null>(null);
    const [params, setParams] = useState(initialParams);
    const [debouncedParams, setDebouncedParams] = useState(initialParams);
    const [imageUint8Array, setImageUint8Array] = useState<Uint8Array | null>(null);
    const [downsampleRateRange, setDownsampleRateRange] = useState({ min: 1, max: 10 });

    // Effect to create a preview URL and get image dimensions
    useEffect(() => {
        if (imageFile) {
            const objectUrl = URL.createObjectURL(imageFile);
            setPreviewUrl(objectUrl);

            const img = new Image();
            img.onload = () => {
                const { width } = img;
                // Adjust downsample rate based on image width to prevent performance issues
                const minRate = Math.max(1, Math.floor(width / 400));
                const maxRate = Math.max(minRate + 9, Math.floor(width / 20)); // Ensure a decent range
                const defaultRate = Math.min(maxRate, Math.max(minRate, Math.floor(width / 150)));

                setDownsampleRateRange({ min: minRate, max: maxRate });
                setParams(prev => ({ ...prev, downsampleRate: defaultRate }));
            };
            img.src = objectUrl;

            // Clean up the object URL on component unmount or when the file changes
            return () => URL.revokeObjectURL(objectUrl);
        }
        setPreviewUrl(null);
        // Reset to default if no image
        setDownsampleRateRange({ min: 1, max: 10 });
        setParams(initialParams);
    }, [imageFile]);

    // Effect to read the image file into a Uint8Array
    useEffect(() => {
        if (imageFile) {
            const reader = new FileReader();
            reader.onload = (e) => {
                if (e.target?.result) {
                    const arrayBuffer = e.target.result as ArrayBuffer;
                    setImageUint8Array(new Uint8Array(arrayBuffer));
                }
            };
            reader.readAsArrayBuffer(imageFile);
        } else {
            setImageUint8Array(null);
        }
    }, [imageFile]);

    // Debounce effect for parameters
    useEffect(() => {
        const handler = setTimeout(() => {
            setDebouncedParams(params);
        }, 200); // 200ms delay

        // Cleanup function to cancel the timeout if params change again
        return () => {
            clearTimeout(handler);
        };
    }, [params]);


    // Handler for parameter changes
    const handleParamChange = useCallback((paramName: string, value: string | number) => {
        setParams(prevParams => ({ ...prevParams, [paramName]: value }));
    }, []);

    return {
        imageFile,
        setImageFile,
        previewUrl,
        params, // for instant UI feedback
        debouncedParams, // for WASM processing
        handleParamChange,
        imageUint8Array,
        downsampleRateRange,
    };
};