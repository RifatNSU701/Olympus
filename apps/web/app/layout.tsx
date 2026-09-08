import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Olympus — Smarter Marketplace",
  description: "A premium multi-vendor marketplace built for modern commerce.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
