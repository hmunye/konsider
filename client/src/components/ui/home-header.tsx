export default function Header() {
  return (
    <header className="flex flex-1 flex-col justify-center px-4">
      <div className="text-5xl">
        <span className="text-5xl relative inline-block bg-primary text-background dark:text-foreground font-nippo-bold p-2 rounded-xl duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Create
        </span>
        {" , "}
        <span className="text-5xl relative inline-block mt-5 bg-secondary text-foreground dark:text-background font-nippo-bold p-2 rounded-xl duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Manage
        </span>
        {" , and "}
        <span className="text-5xl relative inline-block mt-5 bg-accent text-background dark:text-foreground font-nippo-bold p-2 rounded-xl duration-300 translate-y-[-0.2em] hover:translate-y-[-0.5em]">
          Visualize
        </span>
        {" , "}
        <span className="text-5xl flex justify-center mt-5">
          Software and Software Reviews
        </span>
      </div>
    </header>
  );
}
