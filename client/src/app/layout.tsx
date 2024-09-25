import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";

/**
 * @license
 *
 * Font Family: Nippo
 * Designed by: Manushi Parikh
 * URL: https://www.fontshare.com/fonts/nippo
 * © 2024 Indian Type Foundry
 *
 * Nippo Variable (Variable font)
 *
 */
const nippoVariable = localFont({
  src: "../../public/fonts/Nippo-Variable.woff",
  variable: "--font-nippo",
  weight: "200 700",
});

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
      <body
        className={`min-h-screen flex flex-col items-center ${nippoVariable.variable} antialiased`}
      >
        {children}
      </body>
    </html>
  );
}
