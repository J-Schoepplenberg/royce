import { useNavigate } from "@solidjs/router";
import { createEffect, createRenderEffect, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { Store, useStore } from "~/store";
import { Requests } from "~/utils/requests";

/** Register for the backend API. */
export default function Register() {
  const { store, setStore } = useStore();

  const [register, setRegister] = createStore({
    username: "",
    password: "",
    isRegistered: false,
  });

  // Fetch the backend API to register.
  const handleRegister = async (event: Event) => {
    event.preventDefault();

    const response = (await Requests.postJson("/api/register", {
      username: register.username,
      password: register.password,
    })) as Store;

    setStore(response);
    setRegister("isRegistered", true);
  };

  // Runs after registration.
  createEffect(() => {
    if (register.isRegistered) {
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
          Sign up
        </h1>
        <form class="flex flex-col items-center" onSubmit={handleRegister}>
          <input
            type="text"
            placeholder="Username"
            class="border p-2 m-2"
            value={register.username}
            onInput={(e) =>
              setRegister("username", (e.target as HTMLInputElement).value)
            }
          />
          <input
            type="password"
            placeholder="Password"
            class="border p-2 m-2"
            value={register.password}
            onInput={(e) =>
              setRegister("password", (e.target as HTMLInputElement).value)
            }
          />
          <button
            type="submit"
            class="bg-sky-700 text-white p-2 m-2"
          >
            Register
          </button>
        </form>
      </main>
    </Show>
  );
}