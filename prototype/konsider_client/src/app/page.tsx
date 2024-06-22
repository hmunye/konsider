"use client";

import { CreatePostForm } from "@/components/ui/CreatePostForm";
import Navbar from "@/components/ui/Navbar";
import PostsView from "@/components/ui/PostsView";
import ResponseTable from "@/components/ui/ResponseTable";
import { useFetch } from "@/hooks/useFetch";
import { useQuery } from "@tanstack/react-query";
import { useState } from "react";

const apiUrl = "http://127.0.0.1:8000/data";

export default function Home() {
  const [modalVisible, setModalVisible] = useState<boolean>(false);

  const apiQuery = useQuery({
    queryKey: ["message"],
    queryFn: async () => {
      return useFetch({ url: apiUrl, method: "GET" });
    },
  });

  const toggleModal = () => {
    setModalVisible(!modalVisible);
  };

  return (
    <main className="flex min-h-screen flex-col items-center px-10 py-5">
      <Navbar toggleModal={toggleModal} />
      <section className="flex flex-col justify-center items-center gap-10 mt-16">
        <h2 className="text-2xl uppercase font-bold">Test Response Details</h2>
        <ResponseTable query={apiQuery} url={apiUrl} />
      </section>
      {modalVisible && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="fixed inset-0 bg-background opacity-50"
            onClick={toggleModal}
          ></div>
          <div className="z-50 bg-muted p-16 lg:w-1/2 md:w-1/2 sm:w-1/2 rounded-lg">
            <CreatePostForm toggleModal={toggleModal} />
          </div>
        </div>
      )}
      <section className="flex flex-col justify-center items-center gap-10 mt-32">
        <PostsView />
      </section>
    </main>
  );
}
