// @refresh reload
import { createHandler, StartServer } from "@solidjs/start/server";

/**
 * On the server the application starts here, but we are using client-only rendering.
 * Nevertheless, it contains the HTML template the application is rendered into.
 */

export default createHandler(() => (
  <StartServer
    document={({ assets, children, scripts }) => (
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="icon" href="/favicon.ico" />
          {assets}
        </head>
        <body>
          <div id="app">{children}</div>
          {scripts}
        </body>
      </html>
    )}
  />
));
