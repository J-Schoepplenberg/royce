import { useNavigate } from "@solidjs/router";
import { createEffect, createRenderEffect, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { Store, useStore } from "~/store";
import { Requests } from "~/utils/requests";

/** Login for the backend API. */
export default function Login() {
  const { store, setStore } = useStore();

  const [login, setLogin] = createStore({
    username: "",
    password: "",
    isLoggedIn: false,
  });

  // Fetch the backend API to login.
  const handleLogin = async (event: Event) => {
    event.preventDefault();

    const response = (await Requests.postJson("/api/login", {
      username: login.username,
      password: login.password,
    })) as Store;

    setStore(response);
    setLogin("isLoggedIn", true);
  };

  // Runs after login.
  createEffect(() => {
    if (login.isLoggedIn) {
      const navigate = useNavigate();
      navigate("/", { replace: true });
    }
  });

  // Runs already during the render phase and earlier than onMount.
  createRenderEffect(() => {
    if (store.isAuthenticated) {
      const navigate = useNavigate();
      navigate("/", { replace: true });
    }
  });

  // Render only if not authenticated.
  return (
    <Show when={!store.isAuthenticated}>
      <main class="text-center mx-auto text-gray-700 p-4">
        <h1 class="max-6-xs text-6xl text-sky-700 font-thin uppercase my-16">
          Login
        </h1>
        <form class="flex flex-col items-center" onSubmit={handleLogin}>
          <input
            type="text"
            placeholder="Username"
            class="border p-2 m-2"
            value={login.username}
            onInput={(e) =>
              setLogin("username", (e.target as HTMLInputElement).value)
            }
          />
          <input
            type="password"
            placeholder="Password"
            class="border p-2 m-2"
            value={login.password}
            onInput={(e) =>
              setLogin("password", (e.target as HTMLInputElement).value)
            }
          />
          <button type="submit" class="bg-sky-700 text-white p-2 m-2">
            Login
          </button>
        </form>
      </main>
    </Show>
  );
}
