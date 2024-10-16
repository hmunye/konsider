import { cn } from "@/src/lib/utils";

export type Message =
  | { success: string }
  | { error: string }
  | { message: string };

export default function ResponseMessage({
  message,
  className,
}: {
  message: Message;
  className: string;
}) {
  return (
    <div
      className={cn("flex flex-col gap-2 w-full max-w-md text-sm", className)}
    >
      {"success" in message && (
        <div className="text-primary border-l-2 border-primary px-4">
          {message.success}
        </div>
      )}
      {"error" in message && (
        <div className="text-destructive border-l-2 border-destructive px-4">
          {message.error}
        </div>
      )}
      {"message" in message && (
        <div className="text-foreground border-l-2 px-4">{message.message}</div>
      )}
    </div>
  );
}
