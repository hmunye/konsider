import { Button } from "./Button";

export default function Navbar({ toggleModal }: { toggleModal: () => void }) {
  return (
    <nav className="border-b border-b-foreground/10 w-full h-20 flex justify-between items-center lg:px-16 md:px-14 sm:px-8 px-4 pb-5">
      <section>
        <h2 className="text-3xl uppercase font-bold">Konsider</h2>
      </section>
      <section>
        <Button variant={"default"} size={"lg"} onClick={toggleModal}>
          Create Post
        </Button>
      </section>
    </nav>
  );
}
