import type { Metadata } from "next";
import { Analytics } from "@vercel/analytics/next"
import { WasmProvider } from "@/contexts/WasmContext";
import "./globals.css";


export const metadata: Metadata = {
    title: "Image to ASCII Art",
    description: "Convert images to ASCII art using WebAssembly",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body
                className={`antialiased`}
            >
                <WasmProvider>
                    {children}
                </WasmProvider>
                <Analytics />
            </body>
        </html>
    );
}
