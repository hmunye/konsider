import { createFileRoute, Link } from "@tanstack/react-router";
import { Button } from "../components/ui/button";
import { Footer } from "../components/ui/footer";
import { Header } from "../components/ui/home-header";
import { Logo } from "../components/ui/logo";

export const Route = createFileRoute("/")({
  component: () => <Home />,
});

const Home = () => {
  return (
    <section className="flex-1 w-full flex flex-col gap-20 items-center">
      <nav className="w-full flex justify-between items-center border-b border-b-foreground/10 h-20">
        <div className="w-full flex justify-between items-center px-16">
          <Logo />
          <Link to={"/login"}>
            <Button className="mt-2">Log In</Button>
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
