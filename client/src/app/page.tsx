"use client";

import Footer from "@/src/components/ui/footer";
import FormMessage, { Message } from "@/src/components/ui/form/form-message";
import { Input } from "@/src/components/ui/form/input";
import { Label } from "@/src/components/ui/form/label";
import { SubmitButton } from "@/src/components/ui/form/submit-button";
import Header from "@/src/components/ui/home-header";
import Navbar from "@/src/components/ui/navbar";
import { LogIn } from "@/src/lib/api";
import { logInSchema, LogInSchema } from "@/src/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { userStore } from "../store/user";

export default function Home() {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LogInSchema>({
    resolver: zodResolver(logInSchema),
  });
  const [errorMessage, setErrorMessage] = useState<Message>();

  const router = useRouter();
  const setUser = userStore((state) => state.update);

  const onSubmit = async (formData: LogInSchema) => {
    const response = await LogIn(formData);

    if (response.error) {
      setErrorMessage({ error: response.error });
    } else {
      const { name, email, role } = response.user;

      setUser({
        name,
        email,
        role,
      });

      router.push(role === "Admin" ? "/admin/dashboard" : "/dashboard");
    }
  };

  return (
    <section className="flex flex-1 flex-col w-full items-center">
      <Navbar />
      <div className="flex flex-1 flex-row w-full items-center px-3 animate-in">
        <div className="hidden w-[400px] ml-32 lg:block">
          <Header />
        </div>
        <div className="flex flex-col flex-1 max-w-full items-center p-4">
          <form
            onSubmit={handleSubmit(onSubmit)}
            className="flex flex-1 flex-col w-screen justify-center gap-2 text-foreground [&>input]:mb-6 max-w-lg p-4 animate-in"
          >
            <div className="flex flex-col gap-3 [&>input]:mb-4 mt-8 rounded-lg p-9">
              <h1 className="text-3xl font-nippo-bold mb-8">Log In</h1>
              <p className="text-muted text-sm mb-5 font-nippo-extra-light">
                Enter your email below to login to your account
              </p>
              <Label htmlFor="email" className="text-xl">
                Email
              </Label>
              <Input
                {...register("email")}
                id="email"
                type="email"
                autoComplete="email"
                placeholder="you@example.com"
                className="placeholder:text-sm placeholder:font-nippo-extra-light"
              />
              {errors.email && (
                <p className="text-destructive text-sm mt-[-5px] mb-2">{`${errors.email.message}`}</p>
              )}
              <div className="flex justify-between items-center">
                <Label htmlFor="password" className="text-xl">
                  Password
                </Label>
              </div>
              <Input
                {...register("password")}
                id="password"
                type="password"
                placeholder="••••••••"
                className="placeholder:text-sm placeholder:font-nippo-extra-light"
              />
              {errors.password && (
                <p className="text-destructive text-sm mt-[-5px]">{`${errors.password.message}`}</p>
              )}
              <SubmitButton className="mt-5" pending={isSubmitting}>
                Log In
              </SubmitButton>
              {errorMessage && (
                <FormMessage className="mt-5" message={errorMessage} />
              )}
            </div>
          </form>
        </div>
      </div>
      <Footer />
    </section>
  );
}
