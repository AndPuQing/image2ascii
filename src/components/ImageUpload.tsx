// components/ImageUpload.tsx
'use client';
import React, { ChangeEvent } from 'react';

interface ImageUploadProps {
    onImageUpload: (file: File) => void;
    previewUrl?: string | null;
}

export default function ImageUpload({ onImageUpload, previewUrl }: ImageUploadProps) {
    const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
        if (event.target.files && event.target.files[0]) {
            onImageUpload(event.target.files[0]);
        }
    };

    return (
        <div className="mb-6">
            <label htmlFor="imageUpload" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                Upload Image
            </label>
            <input
                type="file"
                id="imageUpload"
                accept="image/*"
                onChange={handleFileChange}
                className="bg-blue-100 rounded-lg p-2 text-sm text-gray-900"
            />
            {previewUrl && (
                <div className="mt-4">
                    <img src={previewUrl} alt="Preview" className="max-w-xs max-h-64 rounded-lg shadow-md" />
                </div>
            )}
        </div>
    );
}