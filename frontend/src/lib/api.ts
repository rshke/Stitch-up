import { API_BASE_URL } from "./config";

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const url = `${API_BASE_URL}${path.startsWith("/") ? "" : "/"}${path}`;
    const response = await fetch(url, {
        ...options,
        headers: {
            "Content-Type": "application/json",
            ...options.headers,
        },
    });

    if (!response.ok) {
        throw new Error(`API Error ${response.status}: ${response.statusText}`);
    }

    if (response.status === 204) {
        return {} as T;
    }

    return response.json();
}

export const api = {
    get: <T>(path: string, options?: RequestInit) =>
        request<T>(path, { ...options, method: "GET" }),
    post: <T>(path: string, body?: any, options?: RequestInit) =>
        request<T>(path, {
            ...options,
            method: "POST",
            body: body ? JSON.stringify(body) : undefined,
        }),
    delete: <T>(path: string, options?: RequestInit) =>
        request<T>(path, { ...options, method: "DELETE" }),
};
