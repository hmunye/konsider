import { createFileRoute, Link } from "@tanstack/react-router";
import { Footer } from "../components/ui/footer";
import { Header } from "../components/ui/home-header";
import { AuthButton } from "../components/ui/auth-button";

export const Route = createFileRoute("/")({
  component: () => <Home />,
});

const Home = () => {
  return (
    <section className="flex-1 w-full flex flex-col gap-20 items-center">
      <nav className="w-full flex justify-between items-center border-b border-b-foreground/10 h-20">
        <div className="w-full flex justify-between items-center px-16">
          <div className="flex flex-row items-center">
            <span className="text-3xl font-bold">Konsider</span>
          </div>
          <Link to={"/login"}>
            <AuthButton className="mt-2">Log In</AuthButton>
          </Link>
        </div>
      </nav>

      <div className="flex-1 flex flex-col gap-20 max-w-4xl px-3 animate-in">
        <Header />
      </div>
      <Footer />
    </section>
  );
};
