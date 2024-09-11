import React from "react";
import { cn } from "../../lib/utils";

type AuthButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {};

const AuthButton = React.forwardRef<HTMLButtonElement, AuthButtonProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <button
        className={cn(
          "relative inline-block bg-primary text-foreground text-lg rounded-md transition-transform duration-100 transform translate-y-[-0.2em] hover:translate-y-[-0.5em] active:translate-y-0 hover:brightness-110",
          className,
        )}
        ref={ref}
        {...props}
      >
        <span className="block bg-primary text-background dark:text-foreground rounded-md px-4 py-2">
          {children}
        </span>
      </button>
    );
  },
);

AuthButton.displayName = "AuthButton";

export { AuthButton };
