/* eslint-disable react-hooks/rules-of-hooks */

import { useFetch } from "../hooks/useFetch";
import { LogInSchema } from "../lib/types";

const API_LOGIN = "/api/v1/auth/login";

export async function logIn(formData: LogInSchema) {
    try {
        const response = await useFetch({
            url: API_LOGIN,
            method: "POST",
            requestBody: formData,
        });

        if (response.error) {
            return { error: response.error };
        }

        return response;
    } catch {
        return { error: "An error occurred during login" };
    }
}

export async function logOut() { }
