"use client";

import {
  Modal,
  ModalBody,
  ModalContent,
  ModalTrigger,
} from "@/src/components/ui/animated-modal";
import { Button } from "@/src/components/ui/button";
import Footer from "@/src/components/ui/footer";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/src/components/ui/form";
import Header from "@/src/components/ui/home-header";
import { Input } from "@/src/components/ui/input";
import Navbar from "@/src/components/ui/navbar";
import ResponseMessage, { Message } from "@/src/components/ui/response-message";
import { API_URL, fetchData } from "@/src/lib/api";
import { logInSchema, LogInSchema, User } from "@/src/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { IconLogin } from "@tabler/icons-react";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useForm } from "react-hook-form";

export default function Home() {
  const form = useForm<LogInSchema>({
    resolver: zodResolver(logInSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const [errorMessage, setErrorMessage] = useState<Message>();
  const router = useRouter();

  const onSubmit = async (formData: LogInSchema) => {
    try {
      const response = await fetchData<User>({
        url: `${API_URL}/v1/auth/login`,
        method: "POST",
        requestBody: formData,
      });

      if (response.error) {
        setErrorMessage({ error: response.error });
        return;
      }

      const user = response.success;

      console.log(user);

      router.push(user!.role === "Admin" ? "/admin/dashboard" : "/dashboard");
    } catch {
      throw new Error("An error occurred during login");
    }
  };

  return (
    <section className="flex flex-1 flex-col w-full items-center">
      <Navbar />
      <div className="flex flex-1 flex-row w-full items-center px-3 animate-in">
        <div className="hidden w-[400px] ml-32 lg:block">
          <Header />
        </div>
        <div className="flex flex-1 items-center justify-center">
          <Modal>
            <ModalTrigger className="bg-primary text-foreground flex justify-center group/modal-btn text-2xl">
              <span className="group-hover/modal-btn:translate-x-40 text-center transition duration-500">
                Start Here
              </span>
              <div className="-translate-x-48 group-hover/modal-btn:translate-x-0 flex items-center justify-center absolute inset-0 transition duration-500 text-foreground z-20">
                <IconLogin className="h-8 w-8" />
              </div>
            </ModalTrigger>
            <ModalBody>
              <ModalContent>
                <div className="flex flex-col flex-1 max-w-full items-center p-4">
                  <Form {...form}>
                    <form
                      onSubmit={form.handleSubmit(onSubmit)}
                      className="flex flex-1 flex-col w-screen justify-center gap-2 text-foreground [&>input]:mb-6 max-w-lg p-4 animate-in"
                    >
                      <div className="flex flex-col gap-10 [&>input]:mb-4 mt-8 rounded-lg p-9">
                        <h1 className="text-3xl font-nippo-bold mb-4">
                          Log In
                        </h1>
                        <FormField
                          control={form.control}
                          name="email"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel htmlFor="email" className="text-xl">
                                Email
                              </FormLabel>
                              <FormControl>
                                <Input
                                  id="email"
                                  type="email"
                                  autoComplete="email"
                                  placeholder="you@example.com"
                                  className="placeholder:text-sm placeholder:font-light"
                                  {...field}
                                />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                        <FormField
                          control={form.control}
                          name="password"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel htmlFor="password" className="text-xl">
                                Password
                              </FormLabel>
                              <FormControl>
                                <Input
                                  id="password"
                                  type="password"
                                  placeholder="••••••••"
                                  className="placeholder:text-sm placeholder:font-light"
                                  {...field}
                                />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                        <Button
                          className="mt-5"
                          type="submit"
                          pending={form.formState.isSubmitting}
                        >
                          Log In
                        </Button>
                        {errorMessage && (
                          <ResponseMessage
                            className="mt-5"
                            message={errorMessage}
                          />
                        )}
                      </div>
                    </form>
                  </Form>
                </div>
              </ModalContent>
            </ModalBody>
          </Modal>
        </div>
      </div>
      <Footer />
    </section>
  );
}
