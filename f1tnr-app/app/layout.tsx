"use client"

import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { ThemeProvider } from "@/components/ui/theme-provider";
import { Separator } from "@/components/ui/separator";
import { ModeToggle } from "@/components/ui/modetoggle";
import { Button } from "@/components/ui/button";
import { GoArrowLeft, GoArrowRight } from "react-icons/go";
import Link from "next/link";
import { useContext, useState } from "react";
import { createContext } from "react";
import navbuttonclientwrapper from "@/components/navclientwrap";
const inter = Inter({ subsets: ["latin"] });






export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

  
  return (
    <html lang="en">

      <body className={inter.className}>
      <ThemeProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange>
        <div className="flex justify-between mt-2 mb-2">
          <div className="ml-4"><ModeToggle></ModeToggle></div>
          {navbuttonclientwrapper()}
        </div>
        
        <Separator />
        {children}
      </ThemeProvider>
      </body>
     
    </html>
  );
}