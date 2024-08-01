// @refresh reload
import { mount, StartClient } from "@solidjs/start/client";

/** 
 * In the browser the application starts here.
 * Puts the application into the HTML template specified in `entry-server.tsx`.
 */

mount(() => <StartClient />, document.getElementById("app")!);
