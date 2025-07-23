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
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
            <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">Upload Image</h2>
            <div className="flex items-center justify-center w-full">
                <label htmlFor="imageUpload" className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-colors">
                    <div className="flex flex-col items-center justify-center pt-5 pb-6">
                        <svg className="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                            <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"/>
                        </svg>
                        <p className="mb-2 text-sm text-gray-500 dark:text-gray-400"><span className="font-semibold">Click to upload</span> or drag and drop</p>
                        <p className="text-xs text-gray-500 dark:text-gray-400">SVG, PNG, JPG or GIF</p>
                    </div>
                    <input id="imageUpload" type="file" className="hidden" onChange={handleFileChange} accept="image/*" />
                </label>
            </div>
            {previewUrl && (
                <div className="mt-6">
                    <h3 className="text-lg font-medium text-gray-800 dark:text-white mb-2">Preview:</h3>
                    <img src={previewUrl} alt="Preview" className="w-full max-h-64 object-contain rounded-lg border border-gray-200 dark:border-gray-700" />
                </div>
            )}
        </div>
    );
}