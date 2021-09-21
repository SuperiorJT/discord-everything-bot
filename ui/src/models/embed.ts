export interface IDiscordEmbed {
  title?: string;
  description?: string;
  url?: string;
  timestamp?: string;
  color?: number;
  footer?: {
    text: string;
    icon_url?: string;
  };
  image?: {
    url?: string;
    height?: number;
    width?: number;
  };
  thumbnail?: {
    url?: string;
    height?: number;
    width?: number;
  };
  author?: {
    name?: string;
    url?: string;
    icon_url?: string;
  };
  fields?: {
    name: string;
    value: string;
    inline?: boolean;
  }[];
}

export interface IEmbed {
  title?: string;
  description?: string;
  url?: string;
  timestamp?: string;
  color?: string | number;
  footer?: {
    text: string;
    image?: File | string;
  };
  image?: File | string;
  thumbnail?: File | string;
  author?: {
    name?: string;
    url?: string;
    image?: File | string;
  };
  fields?: {
    name: string;
    value: string;
    inline?: boolean;
  }[];
}