import { get, writable } from "svelte/store";

export const activeGuild = writable(import.meta.env.VITE_DEFAULT_GUILD_ID.toString() || '');

export const setActiveGuild = (id: string): void => {
    activeGuild.set(id);
}

export const getActiveGuild = (): string => {
    return get(activeGuild);
}