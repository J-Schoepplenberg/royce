import { useLocation } from "@solidjs/router";
import { Show } from "solid-js";
import { Store, useStore } from "~/store";
import { Requests } from "~/utils/requests";

/** Navigation of the application. */
export default function Nav() {
  const { store, setStore } = useStore();

  // Changes border styling based on the current path.
  const active = (path: string) =>
    path == useLocation().pathname
      ? "border-sky-600"
      : "border-transparent hover:border-sky-600";

  // Fetch the backend API to logout.
  const handleLogout = async () => {
    const response = (await Requests.postJson("/api/logout", {})) as Store;
    setStore(response);
  };

  return (
    <nav class="bg-sky-800">
      <div class="flex justify-between items-center p-3 text-gray-200">
        <ul class="flex">
          <li class={`border-b-2 ${active("/")} mx-1.5 sm:mx-6`}>
            <a href="/">Home</a>
          </li>
          <Show when={store.isAuthenticated}>
            <li class={`border-b-2 ${active("/private")} mx-1.5 sm:mx-6`}>
              <a href="/private">Protected</a>
            </li>
          </Show>
        </ul>
        <ul class="flex">
          <Show when={!store.isAuthenticated}>
            <li class={`border-b-2 ${active("/login")} mx-1.5 sm:mx-6`}>
              <a href="/login">Login</a>
            </li>
            <li class={`border-b-2 ${active("/register")} mx-1.5 sm:mx-6`}>
              <a href="/register">Sign up</a>
            </li>
          </Show>
          <Show when={store.isAuthenticated}>
            <li>
              <button
                class="border-b-2 border-transparent hover:border-sky-600 mx-1.5 sm:mx-6"
                onClick={handleLogout}
              >
                Logout
              </button>
            </li>
          </Show>
        </ul>
      </div>
    </nav>
  );
}
