import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Link from "next/link";
import { Home, Users } from "lucide-react";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "PUBG Friends Tracker",
  description: "Track and compare your PUBG friends statistics",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <nav className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
          <div className="container mx-auto px-6 py-4">
            <div className="flex items-center justify-between">
              <Link href="/" className="flex items-center gap-2 text-xl font-bold">
                🎮 PUBG Tracker
              </Link>
              <div className="flex items-center gap-4">
                <Link
                  href="/"
                  className="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium hover:bg-muted transition-colors"
                >
                  <Home className="w-4 h-4" />
                  Dashboard
                </Link>
                <Link
                  href="/players"
                  className="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium hover:bg-muted transition-colors"
                >
                  <Users className="w-4 h-4" />
                  Players
                </Link>
              </div>
            </div>
          </div>
        </nav>
        <main>{children}</main>
      </body>
    </html>
  );
}
