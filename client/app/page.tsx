import { Button } from "@/components/ui/Button";

export default function Home() {
    return (
        <section className="flex-1 w-full flex flex-col gap-20 items-center">
            <nav className="w-full flex justify-between items-center border-b border-b-foreground/10 h-20">
                <div className="w-full flex justify-between items-center px-20">
                    <h1 className="text-3xl font-bold">Konsider</h1>
                    <Button className="mt-2">Login</Button>
                </div>
            </nav>

            <div className="flex-1 flex flex-col gap-20 max-w-4xl px-3">
                <main className="flex-1 flex flex-col gap-6 px-4">
                    <h2 className="font-bold text-2xl mb-4">Header Section</h2>
                </main>
            </div>

            <footer className="w-full border-t border-t-foreground/10 p-8 flex justify-center text-center text-xs">
                <p>Vendor Risk Management</p>
            </footer>
        </section>
    );
}
