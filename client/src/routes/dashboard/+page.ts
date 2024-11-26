import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { type ApiResponse, fetchRequest } from "$lib/fetch.js";
import type {
  SoftwareRequestResponse,
  SoftwareResponse,
  SoftwareReviewResponse,
  UserResponse,
} from "$lib/types/types.js";
import type { PageLoad } from "./$types.js";

export const load: PageLoad = async ({ fetch }) => {
  const fetchPromises = [
    fetchRequest<UserResponse>(
      {
        url: `${PUBLIC_BASE_API_URL}/api/v1/users?per_page=1`,
        method: "GET",
      },
      fetch,
    ),
    fetchRequest<SoftwareResponse>(
      {
        url: `${PUBLIC_BASE_API_URL}/api/v1/software?per_page=1`,
        method: "GET",
      },
      fetch,
    ),
    fetchRequest<SoftwareRequestResponse>(
      {
        url: `${PUBLIC_BASE_API_URL}/api/v1/requests?per_page=7`,
        method: "GET",
      },
      fetch,
    ),
    fetchRequest<SoftwareReviewResponse>(
      {
        url: `${PUBLIC_BASE_API_URL}/api/v1/reviews?per_page=7`,
        method: "GET",
      },
      fetch,
    ),
  ];

  const responses: [
    ApiResponse<UserResponse>,
    ApiResponse<SoftwareResponse>,
    ApiResponse<SoftwareRequestResponse>,
    ApiResponse<SoftwareReviewResponse>,
  ] = (await Promise.all(fetchPromises)) as [
    ApiResponse<UserResponse>,
    ApiResponse<SoftwareResponse>,
    ApiResponse<SoftwareRequestResponse>,
    ApiResponse<SoftwareReviewResponse>,
  ];

  for (const response of responses) {
    if (response.error) {
      return {
        error: response.error,
      };
    }
  }

  return {
    users: responses[0].success,
    software: responses[1].success,
    software_requests: responses[2].success,
    software_reviews: responses[3].success,
  };
};