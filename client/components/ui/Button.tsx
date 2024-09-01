import { cn } from "@/utils/cn";
import React, { ForwardedRef } from "react";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string;
  children?: React.ReactNode;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ children, ...props }, ref: ForwardedRef<HTMLButtonElement>) => {
    return (
      <button
        ref={ref}
        className={cn(
          `relative inline-block bg-primary text-foreground font-bold text-md 
           rounded-md transition-transform duration-100 transform translate-y-[-0.2em] 
           hover:translate-y-[-0.4em] active:translate-y-0`,
          props.className,
        )}
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
