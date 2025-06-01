'use client';
import ImageUpload from "@/components/ImageUpload";
import ParameterControls from "@/components/ParameterControls";
import WasmImage2AsciiComponent from "@/components/WasmRender";
import { useEffect, useState } from "react";


const initialParams = {
    downsampleRate: 2,
    edgeSobelThreshold: 100,
    asciiCharsEdge: ' -/|\\',
    asciiCharsGray: '@?OPoc:. ',
};

export default function Home() {
    const [imageFile, setImageFile] = useState<File | null>(null);
    const [previewUrl, setPreviewUrl] = useState<string | null>(null);
    const [params, setParams] = useState(initialParams);
    const [imageUint8Array, setImageUint8Array] = useState<Uint8Array | null>(null);

    useEffect(() => {
        if (imageFile) {
            const objectUrl = URL.createObjectURL(imageFile);
            setPreviewUrl(objectUrl);
            return () => URL.revokeObjectURL(objectUrl);
        }
        setPreviewUrl(null);
    }, [imageFile]);

    const handleParamChange = (paramName: string, value: string | number) => {
        setParams(prevParams => ({ ...prevParams, [paramName]: value }));
    };

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


    return (
        <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
            <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">

                <ImageUpload onImageUpload={setImageFile} previewUrl={previewUrl} />
                <ParameterControls params={params} onParamChange={handleParamChange} />

                {imageFile && imageUint8Array && (
                    <WasmImage2AsciiComponent
                        image_bytes={imageUint8Array}
                        downsample_rate={params.downsampleRate}
                        edge_sobel_threshold={params.edgeSobelThreshold}
                        ascii_chars_edge_str={params.asciiCharsEdge}
                        ascii_chars_gray_str={params.asciiCharsGray}
                    />
                )}
            </main >
            <footer className="row-start-3 flex gap-[24px] flex-wrap items-center justify-center">
                <div className="text-center text-sm text-gray-500 mt-8">
                    <p>Powered by WebAssembly and Next.js</p>
                    <p>© 2025 550w.host</p>
                </div>
            </footer>
        </div >
    );
}
