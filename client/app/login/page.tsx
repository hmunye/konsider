import { BackLink } from "@/components/ui/back-link";
import { FormMessage, Message } from "@/components/ui/form/form-message";
import { Input } from "@/components/ui/form/input";
import { Label } from "@/components/ui/form/label";
import { SubmitButton } from "@/components/ui/form/submit-button";
import { redirect } from "next/navigation";

export default function Login({ searchParams }: { searchParams: Message }) {
  const signIn = async (formData: FormData) => {
    "use server";

    return redirect("/protected");
  };

  return (
    <div className="flex flex-col flex-1 p-4 w-full items-center">
      <BackLink url={"/"} />
      <form className="flex-1 flex flex-col w-full justify-center gap-2 text-foreground [&>input]:mb-6 max-w-md p-4 animate-in">
        <h1 className="text-3xl font-bold">Log In</h1>
        <div className="flex flex-col gap-2 [&>input]:mb-8 mt-8">
          <Label htmlFor="email" className="text-lg">
            Email
          </Label>
          <Input name="email" placeholder="you@example.com" required />
          <div className="flex justify-between items-center">
            <Label htmlFor="password" className="text-lg">
              Password
            </Label>
          </div>
          <Input
            type="password"
            name="password"
            placeholder="••••••••"
            required
          />
          <SubmitButton formAction={signIn} pendingText="Signing In...">
            Log In
          </SubmitButton>
          <FormMessage message={searchParams} />
        </div>
      </form>
    </div>
  );
}
