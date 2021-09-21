export const post = async (url: string, body: unknown, config?: RequestInit): Promise<Response> => {
  return await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body),
    ...config
  });
};
