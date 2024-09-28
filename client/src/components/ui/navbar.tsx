"use client";

import Image from "next/image";
import Logo from "@/public/images/logo.png";
import { Button } from "@/src/components/ui/button";
import { userStore } from "@/src/store/user";

export default function Navbar() {
  const user = userStore((state) => state.user);

  return (
    <nav className="flex w-full justify-between items-center border-b border-b-foreground/10 p-2 px-10 lg:px-20">
      <div className="flex justify-evenly items-center">
        <Image src={Logo} alt="Konsider Logo" width={50} height={50} />
        <span className="text-3xl font-nippo-bold">Konsider</span>
      </div>
      {user ? (
        <div className="mt-2">
          <Button className="bg-muted text-sm">Log Out</Button>
        </div>
      ) : (
        <></>
      )}
    </nav>
  );
}
