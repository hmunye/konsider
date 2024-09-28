import Navbar from "@/src/components/ui/navbar";

export default function UserLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <main className="flex flex-1 flex-col w-full items-center">
      <Navbar />
      {children}
    </main>
  );
}
