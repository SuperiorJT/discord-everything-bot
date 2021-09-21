import preprocess from 'svelte-preprocess';
import path from 'path';

const config = {
  preprocess: [
    preprocess({
      postcss: true
    })
  ],
  kit: {
    vite: {
      optimizeDeps: {
        include: ['svelte-hero-icons']
      },
      ssr: {
        noExternal: ['svelte-hero-icons']
      },
      resolve: {
        alias: {
          $components: path.resolve('./src/components'),
          $utils: path.resolve('./src/utils'),
          $http: path.resolve('./src/http')
        }
      }
    }
  }
};

export default config;
