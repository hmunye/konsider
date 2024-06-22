"use client";

import { Button } from "@/components/ui/Button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/Form";
import { Input } from "@/components/ui/Input";
import { useFetch } from "@/hooks/useFetch";
import { createPostFormSchema } from "@/types/types";
import { getCurrentTimeFormatted } from "@/utils/getCurrentTimeFormatted";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useToast } from "./UseToast";

const apiUrl = "http://127.0.0.1:8000/create-post";

export function CreatePostForm({ toggleModal }: { toggleModal: () => void }) {
  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);
  const { toast } = useToast();

  const form = useForm<z.infer<typeof createPostFormSchema>>({
    resolver: zodResolver(createPostFormSchema),
    defaultValues: {
      title: "",
      content: "",
    },
  });

  const onSubmit = async (postData: z.infer<typeof createPostFormSchema>) => {
    try {
      setIsSubmitting(true);

      useFetch({
        url: apiUrl,
        method: "POST",
        requestBody: postData,
      });

      toast({
        title: "Post Created Successfully:",
        description: getCurrentTimeFormatted(),
      });
    } catch (error) {
      toast({
        title: "Post Creation Failed: ",
        description: `${error}`,
      });
    } finally {
      setIsSubmitting(false);
      toggleModal();
    }
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <h2 className="text-2xl uppercase font-bold">Create Post</h2>
        <FormField
          control={form.control}
          name="title"
          render={({ field }) => (
            <FormItem>
              <FormLabel className="font-bold">Title</FormLabel>
              <FormControl>
                <Input placeholder="Enter Title" {...field} />
              </FormControl>
              <FormDescription></FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="content"
          render={({ field }) => (
            <FormItem>
              <FormLabel className="font-bold">Content</FormLabel>
              <FormControl>
                <Input placeholder="Enter Content" {...field} />
              </FormControl>
              <FormDescription></FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? "Submitting..." : "Submit"}
        </Button>
      </form>
    </Form>
  );
}
