import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Konsider",
    description: "Vendor Risk Management",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className={inter.className}>
                <main className="min-h-screen flex flex-col items-center bg-background text-foreground font-mono">
                    {children}
                </main>
            </body>
        </html>
    );
}
