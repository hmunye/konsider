import { cn } from "@/lib/utils";
import React from "react";

type SubmitButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  pending: boolean;
};

const SubmitButton = React.forwardRef<HTMLButtonElement, SubmitButtonProps>(
  ({ className, children, pending, ...props }, ref) => {
    return (
      <button
        className={cn(
          "bg-primary h-9 flex items-center justify-center font-medium text-md hover:brightness-125 duration-300 transition rounded-md text-foreground disabled:brightness-75",
          className,
        )}
        type="submit"
        disabled={pending}
        aria-disabled={pending}
        ref={ref}
        {...props}
      >
        {pending ? (
          <div className="animate-spin border-4 border-solid border-l-transparent rounded-2xl w-5 h-5 border-foreground brightness-75"></div>
        ) : (
          children
        )}
      </button>
    );
  },
);

SubmitButton.displayName = "SubmitButton";

export { SubmitButton };
