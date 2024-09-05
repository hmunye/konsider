import Image from "next/image";
import LogoImage from "@/public/logo.png";

export function Logo() {
  return (
    <div className="flex flex-row items-center">
      <Image
        src={LogoImage}
        alt="Konsider Logo"
        width={230}
        height={230}
      ></Image>
    </div>
  );
}
