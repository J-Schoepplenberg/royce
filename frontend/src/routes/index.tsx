import { createResource } from "solid-js";
import { Requests } from "~/utils/requests";

/** Some public endpoint. */
export default function Home() {
  const [data] = createResource(
    async () => (await Requests.getJson("/api")) as string
  );

  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <h1 class="max-6-xs text-6xl text-sky-700 font-thin uppercase my-16">
        Public Endpoint
      </h1>
      <div>{data()}</div>
    </main>
  );
}
