'use client';
import ImageUpload from "@/components/ImageUpload";
import ParameterControls from "@/components/ParameterControls";
import WasmImage2AsciiComponent from "@/components/WasmRender";
import { useAsciiConversion } from "@/hooks/useAsciiConversion";

export default function Home() {
    const {
        imageFile,
        setImageFile,
        previewUrl,
        params,
        debouncedParams,
        handleParamChange,
        imageUint8Array,
        downsampleRateRange,
    } = useAsciiConversion();

    return (
        <div className="min-h-screen w-full p-4 sm:p-6 lg:p-8">
            <header className="text-center mb-8">
                <h1 className="text-4xl font-bold text-gray-800 dark:text-white">Image to ASCII Art</h1>
                <p className="text-lg text-gray-600 dark:text-gray-400">Convert your images into colorful ASCII art using WebAssembly.</p>
            </header>

            <main className="grid grid-cols-1 lg:grid-cols-2 gap-8 max-w-7xl mx-auto">
                {/* Left Column: Controls */}
                <div className="flex flex-col gap-8">
                    <ImageUpload onImageUpload={setImageFile} previewUrl={previewUrl} />
                    <ParameterControls params={params} onParamChange={handleParamChange} downsampleRateRange={downsampleRateRange} />
                </div>

                {/* Right Column: ASCII Art Display */}
                <div className="lg:row-span-2 flex items-center justify-center bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 min-h-[300px] lg:min-h-0">
                    {imageFile && imageUint8Array ? (
                        <WasmImage2AsciiComponent
                            image_bytes={imageUint8Array}
                            downsample_rate={debouncedParams.downsampleRate}
                            edge_sobel_threshold={debouncedParams.edgeSobelThreshold}
                            ascii_chars_edge_str={debouncedParams.asciiCharsEdge}
                            ascii_chars_gray_str={debouncedParams.asciiCharsGray}
                        />
                    ) : (
                        <div className="text-center text-gray-500">
                            <p>Upload an image to see the ASCII magic!</p>
                        </div>
                    )}
                </div>
            </main>

            <footer className="text-center text-sm text-gray-500 mt-12">
                <p>Powered by WebAssembly and Next.js</p>
                <p>© 2025 550w.host</p>
            </footer>
        </div>
    );
}
