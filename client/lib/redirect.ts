"use client";

import { useRouter } from "next/navigation";

export function encodedRedirect(
  type: "error" | "success",
  path: string,
  message: string,
) {
  const router = useRouter();
  router.push(`${path}?${type}=${encodeURIComponent(message)}`);
}
