import { cn } from "@/src/lib/utils";
import React from "react";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {};

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <button
        className={cn(
          "relative inline-block bg-primary text-background dark:text-foreground px-4 py-2 text-lg rounded-md transition-transform duration-100 transform translate-y-[-0.2em] hover:translate-y-[-0.5em] active:translate-y-0 hover:brightness-110",
          className,
        )}
        ref={ref}
        {...props}
      >
        {children}
      </button>
    );
  },
);

Button.displayName = "Button";

export { Button };
