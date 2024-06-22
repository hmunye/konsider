import { QueryProvider } from "@/components/provider/QueryProvider";
import { Toaster } from "@/components/ui/Toaster";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Konsider",
  description: "",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <QueryProvider>
      <html lang="en">
        <body className={inter.className}>
          <main className="text-foreground bg-background">{children}</main>
          <Toaster />
        </body>
      </html>
    </QueryProvider>
  );
}
