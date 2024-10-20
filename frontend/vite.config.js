import { defineConfig } from 'vite';
import injectHTML from 'vite-plugin-html-inject';

export default defineConfig(({ command }) => {
  return {
    server: {
      historyApiFallback: false,  // Disable default fallback behavior
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          const filePath = path.join(__dirname, 'static/pages', req.url);

          // Check if the requested file exists in the pages directory
          if (req.url.startsWith('/pages/') && !fs.existsSync(filePath)) {
            // Return a 404 response for missing files
            res.statusCode = 404;
            res.end('404 - Page Not Found');
            return;
          }

          next();
        });
      }
    },
    plugins: [injectHTML()],
    base: command === 'build' ? '/static/' : '/',
  };
});
