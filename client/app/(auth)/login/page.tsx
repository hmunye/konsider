"use client";

import { BackLink } from "@/components/ui/back-link";
import { FormMessage, Message } from "@/components/ui/form/form-message";
import { Input } from "@/components/ui/form/input";
import { Label } from "@/components/ui/form/label";
import { SubmitButton } from "@/components/ui/form/submit-button";
import { LogInSchema, logInSchema } from "@/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { logIn } from "../actions/server";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function Login({ searchParams }: { searchParams: Message }) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LogInSchema>({
    resolver: zodResolver(logInSchema),
  });

  const router = useRouter();

  const onSubmit = async (formData: LogInSchema) => {
    const response = await logIn(formData);

    if (response.error) {
      router.push(`/login?error=${encodeURIComponent(response.error)}`);
    } else {
      router.push("/dashboard");
    }
  };

  return (
    <div className="flex flex-col flex-1 p-4 w-full items-center">
      <BackLink url={"/"} />
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex-1 flex flex-col w-full justify-center gap-2 text-foreground [&>input]:mb-6 max-w-lg p-4 animate-in"
      >
        <div className="flex flex-col gap-2 [&>input]:mb-4 mt-8 border-2 rounded-lg p-9">
          <h1 className="text-3xl font-bold mb-8">Log In</h1>
          <p className="text-muted text-sm mb-5">
            Enter your email below to login to your account
          </p>
          <Label htmlFor="email" className="text-lg">
            Email
          </Label>
          <Input
            {...register("email")}
            type="email"
            placeholder="you@example.com"
          />
          {errors.email && (
            <p className="text-destructive text-sm">{`${errors.email.message}`}</p>
          )}
          <div className="flex justify-between items-center">
            <Label htmlFor="password" className="text-lg">
              Password
            </Label>
          </div>
          <Input
            {...register("password")}
            type="password"
            placeholder="••••••••"
          />
          {errors.password && (
            <p className="text-destructive text-sm">{`${errors.password.message}`}</p>
          )}
          <SubmitButton className="mt-5" pending={isSubmitting}>
            Log In
          </SubmitButton>
          <FormMessage className="mt-5" message={searchParams} />
        </div>
      </form>
    </div>
  );
}
