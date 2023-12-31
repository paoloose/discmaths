export const BACKEND_ADDR = import.meta.env.PUBLIC_VENNBASE_API;

type VennfetchOptions = {
  method?: string,
  abortSignal?: AbortSignal,
  body?: BodyInit | null | undefined
}

export async function vennfetch(endpoint: string, options?: VennfetchOptions) {
  const response = await fetch(`${BACKEND_ADDR}${endpoint}`, {
    method: options?.method ?? 'GET',
    signal: options?.abortSignal,
    body: options?.body,
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    }
  }).then(response => {
    if (response.status < 200 || response.status >= 300) {
      throw new Error(response.statusText);
    }
    return response;
  });
  return response;
}
