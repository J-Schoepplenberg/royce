import { useNavigate } from "@solidjs/router";
import { createRenderEffect, createResource, Show } from "solid-js";
import { useStore } from "~/store";
import { Requests } from "~/utils/requests";

/** Some private endpoint. */
export default function Private() {
  const { store } = useStore();

  const [data] = createResource(
    async () => (await Requests.getJson("/api/protected")) as string
  );

  // Runs already during the render phase and earlier than onMount.
  createRenderEffect(async () => {
    if (!store.isAuthenticated) {
      const navigate = useNavigate();
      navigate("/login", { replace: true });
    }
  });

  // Render only if authenticated.
  return (
    <Show when={store.isAuthenticated}>
      <main class="text-center mx-auto text-gray-700 p-4">
        <h1 class="max-6-xs text-6xl text-sky-700 font-thin uppercase my-16">
          Private Endpoint
        </h1>
        <div>{data()}</div>
      </main>
    </Show>
  );
}
