import Link from "next/link";

export function BackLink({ url }: { url: string }) {
  return (
    <Link
      href={url}
      className="absolute left-8 top-8 py-2 px-4 rounded-md no-underline text-foreground bg-background hover:brightness-200 flex items-center group text-sm"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="mr-2 h-4 w-4 transition-transform group-hover:-translate-x-1"
      >
        <polyline points="15 18 9 12 15 6" />
      </svg>{" "}
      Back
    </Link>
  );
}
