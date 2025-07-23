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

    // Effect to create a preview URL when a new image is uploaded
    useEffect(() => {
        if (imageFile) {
            const objectUrl = URL.createObjectURL(imageFile);
            setPreviewUrl(objectUrl);
            // Clean up the object URL on component unmount or when the file changes
            return () => URL.revokeObjectURL(objectUrl);
        }
        setPreviewUrl(null);
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
    };
};