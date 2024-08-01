import { defineConfig } from "@solidjs/start/config";

/** 
 * The root of SolidStart apps and main point of configuration. 
 * Exports configuration for e.g. SolidStart, Vinxi and Vite.
 */

export default defineConfig({
    server: {
        /** 
         * Proxies requests to the backend server in development.
         * Avoids any CORS issues with sessions.
         * In production this is done by nginx.
         */
        devProxy: {
            "/api": {
                target: "http://127.0.0.1:3001",
                changeOrigin: true,
            },
        }
    }
});
