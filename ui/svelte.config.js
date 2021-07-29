import preprocess from "svelte-preprocess";

const config = {
  preprocess: [preprocess({
    defaults: {
      script: "typescript",
      style: "postcss"
    },
    postcss: true
  })],
};

export default config;