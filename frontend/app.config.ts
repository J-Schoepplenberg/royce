import { defineConfig } from "@solidjs/start/config";

/** 
 * The root of SolidStart apps and main point of configuration. 
 * Exports configuration for e.g. SolidStart, Vinxi and Vite.
 */

export default defineConfig({
    server: { // Nitro configurations.
        devProxy: { // Proxy requests to backend server to avoid CORS issues.
            "/api/": {
                target: "http://0.0.0.0:8000/api/",
                changeOrigin: true,
            },
        },
        compressPublicAssets: true, // Pre-compress public assets.
        minify: true, // Minifies bundle.
        output: { dir: "dist" }, // Bundle output directory.
    },
    ssr: false, // Toggle to client-side rendering
});
