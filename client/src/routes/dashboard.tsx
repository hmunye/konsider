import { createFileRoute } from "@tanstack/react-router";
import Navbar from "../components/ui/navbar";

export const Route = createFileRoute("/dashboard")({
  component: () => <Dashboard />,
});

const Dashboard = () => {
  return (
    <>
      <Navbar />
      <div className="flex flex-row justify-between gap-20 mt-20">
        <span>Hello /dashboard!</span>
      </div>
    </>
  );
};
