"use client";

import Logo from "@/public/images/logo.png";
import Image from "next/image";

export default function Navbar() {
  return (
    <nav className="flex w-full justify-between items-center border-b border-b-foreground/10 p-2 px-10 lg:px-20">
      <div className="flex justify-evenly items-center">
        <Image src={Logo} alt="Konsider Logo" width={50} height={50} />
        <span className="text-3xl font-nippo-bold">Konsider</span>
      </div>
    </nav>
  );
}
