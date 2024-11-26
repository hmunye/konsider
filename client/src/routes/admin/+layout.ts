import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { fetchRequest } from "$lib/fetch.js";
import type { User } from "$lib/types/types.js";
import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch, url }) => {
  const response = await fetchRequest<{ user: User }>(
    {
      url: `${PUBLIC_BASE_API_URL}/api/v1/auth/check`,
      method: "GET",
    },
    fetch,
  );

  if (response.error) {
    const fromUrl = url.pathname + url.search;

    throw redirect(
      302,
      `/?redirectTo=${fromUrl}&message=${response.error.message}`,
    );
  }

  if (response.success?.user.role !== "ADMIN") {
    const fromUrl = url.pathname + url.search;

    throw redirect(
      302,
      `/?redirectTo=${fromUrl}&message=Log in as an administrator to view this page`,
    );
  }

  return {
    user: response.success?.user,
  };
};
