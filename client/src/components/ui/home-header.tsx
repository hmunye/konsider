export function Header() {
  return (
    <main className="flex-1 flex flex-col justify-center gap-6 px-4">
      <div className="text-5xl mb-4 w-72 md:w-full">
        <span className="relative inline-block bg-primary text-background dark:text-foreground p-2 rounded-md duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Create
        </span>
        {" , "}
        <span className="relative inline-block mt-5 bg-secondary text-foreground dark:text-background p-2 rounded-md duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Manage
        </span>
        {" , and "}
        <span className="relative inline-block mt-5 bg-accent text-background dark:text-foreground p-2 rounded-md duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Visualize
        </span>
        {" , "}
        <span className="flex justify-center mt-2">Software Reviews</span>
      </div>
    </main>
  );
}
