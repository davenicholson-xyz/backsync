import { defineConfig } from 'vite';
import injectHTML from 'vite-plugin-html-inject';

export default defineConfig(({ command }) => {
  return {
    plugins: [injectHTML()],
    base: command === 'build' ? '/static/' : '/',
  };
});
