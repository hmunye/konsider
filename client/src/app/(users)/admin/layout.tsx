"use client";

import Logo from "@/public/images/logo.png";
import LoadingSpinner from "@/src/components/ui/loading-spinner";
import { Sidebar, SidebarBody, SidebarLink } from "@/src/components/ui/sidebar";
import { API_URL, fetchData } from "@/src/lib/api";
import { cn } from "@/src/lib/utils";
import {
  IconBrandTabler,
  IconCloud,
  IconDatabaseEdit,
  IconLogout2,
  IconSettings,
  IconUserCircle,
  IconUserSearch,
  IconUsersGroup,
} from "@tabler/icons-react";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function AdminLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const router = useRouter();

  const onLogOut = async () => {
    try {
      setLoading(true);

      const response = await fetchData({
        url: `${API_URL}/v1/auth/logout`,
        method: "POST",
      });

      if (response.error) {
        throw new Error("An error returned in response to logout");
      }

      router.push("/");
    } catch {
      throw new Error("An error occurred during login");
    } finally {
      setLoading(false);
    }
  };

  const links = [
    {
      label: "Dashboard",
      href: "/admin/dashboard",
      icon: (
        <IconBrandTabler className="text-foreground h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Users",
      href: "/admin/users",
      icon: (
        <IconUsersGroup className="text-foreground h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Requesters",
      href: "/admin/requesters",
      icon: (
        <IconUserSearch className="text-foreground h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Software",
      href: "/admin/software",
      icon: <IconCloud className="text-foreground h-8 w-8 flex-shrink-0" />,
    },
    {
      label: "Software Reviews",
      href: "/admin/software-reviews",
      icon: (
        <IconDatabaseEdit className="text-foreground h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Settings",
      href: "/admin/settings",
      icon: <IconSettings className="text-foreground h-8 w-8 flex-shrink-0" />,
    },
    {
      label: "Log Out",
      href: "#",
      onClick: async () => onLogOut(),
      icon: loading ? (
        <div className="pl-[5px]">
          <LoadingSpinner />
        </div>
      ) : (
        <IconLogout2 className="text-foreground h-8 w-8 flex-shrink-0" />
      ),
    },
  ];

  return (
    <div
      className={cn(
        "rounded-md flex flex-col md:flex-row bg-background w-full flex-1 max-w-screen mx-auto overflow-hidden max-h-screen animate-in",
      )}
    >
      <Sidebar open={open} setOpen={setOpen}>
        <SidebarBody className="justify-between gap-10">
          <div className="flex flex-col flex-1 overflow-y-auto overflow-x-hidden">
            {open ? (
              <div className="flex items-center">
                <Image src={Logo} alt="Konsider Logo" width={50} height={50} />
                <span className="text-3xl px-3 font-nippo-bold">Konsider</span>
              </div>
            ) : (
              <Image src={Logo} alt="Konsider Logo" width={42} height={42} />
            )}
            <div className="mt-8 flex flex-col gap-5">
              {links.map((link, idx) => (
                <SidebarLink key={idx} link={link} />
              ))}
            </div>
          </div>
          <div>
            <SidebarLink
              link={{
                label: "Name",
                href: "#",
                icon: (
                  <IconUserCircle className="text-foreground h-8 w-8 flex-shrink-0" />
                ),
              }}
            />
          </div>
        </SidebarBody>
      </Sidebar>
      <div className="flex flex-1">{children}</div>
    </div>
  );
}
