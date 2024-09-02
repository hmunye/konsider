import { cn } from "@/lib/utils";
import React from "react";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {};

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <button
        className={cn(
          "relative inline-block bg-primary text-foreground font-bold text-md rounded-md transition-transform duration-100 transform translate-y-[-0.2em] hover:translate-y-[-0.5em] active:translate-y-0 hover:brightness-125",
          className,
        )}
        ref={ref}
        {...props}
      >
        <span className="block bg-primary text-foreground rounded-md px-4 py-2">
          {children}
        </span>
      </button>
    );
  },
);

Button.displayName = "Button";

export { Button };
