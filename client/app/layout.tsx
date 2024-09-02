import type { Metadata } from "next";
import "./globals.css";

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
            <body>
                <main className="min-h-screen flex flex-col items-center">
                    {children}
                </main>
            </body>
        </html>
    );
}
