import { getActiveGuild } from './active-guild';
import { post } from './util';

const url = () => `http://localhost:3030/${getActiveGuild()}/welcome`;

export default {
  fetchModuleData: async (): Promise<Response> => await fetch(url()),
  postModuleData: async (data: unknown): Promise<Response> =>
    await post(url(), data),
  uploadModuleImages: async (data: FormData): Promise<Response> =>
    await post(`${url()}/images`, data, { headers: undefined, body: data })
};
