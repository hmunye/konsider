"use client";

import { userStore } from "@/src/store/user";

export default function Dashboard() {
  const { user, loading } = userStore();

  if (loading) {
    return (
      <div className="animate-spin border-4 border-solid border-l-transparent rounded-2xl w-5 h-5 border-foreground brightness-105"></div>
    );
  }

  return (
    <div className="flex flex-row justify-between gap-20 mt-20">
      <span>Hello /admin/dashboard!</span>
      <span>Name: {user.name}</span>
      <span>Email: {user.email}</span>
      <span>Role: {user.role}</span>
    </div>
  );
}
