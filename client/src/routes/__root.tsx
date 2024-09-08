import { Outlet, createRootRoute } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: () => (
    <>
      <main className="min-h-screen flex flex-col items-center">
        <Outlet />
      </main>
    </>
  ),
});
