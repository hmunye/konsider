"use client";

import ResponseTable from "@/components/ui/ResponseTable";
import { useFetch } from "@/hooks/useFetch";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

export default function Home() {
  const apiUrl = "http://127.0.0.1:8000/data";

  useEffect(() => {}, [apiUrl]);

  const apiQuery = useQuery({
    queryKey: ["message"],
    queryFn: async () => {
      return useFetch({ url: apiUrl, method: "GET" });
    },
  });

  return (
    <main className="flex min-h-screen flex-col items-center justify-center">
      <h2 className="text-5xl uppercase font-bold">Konsider</h2>
      <div className="mt-16">
        <ResponseTable query={apiQuery} url={apiUrl} />
      </div>
    </main>
  );
}
