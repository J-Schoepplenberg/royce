# royce

Extremely simple `Axum` + `SolidJS` starter for a fullstack web application. 

Deploy immediately with a ready-to-use preconfigured `Docker` deployment.

Fully fledged for high-performance:
- **Frontend**: fine-grained reactivity framework `SolidStart`
- **Backend**: multi-threaded asynchronous Rust backend `Axum`
- **Sessions**: in-memory database `Redis`
- **Database**: object-relational database `PostgreSQL`
- **Deployment**: container virtualization `Docker`

Contains everything you need:
- Authentication
- Sessions
- CORS
- Middleware
- Rate limiting
- Response compression
- Request size limiting
- Signed and encrypted cookies

## About

The [Google Lighthouse](https://en.wikipedia.org/wiki/Google_Lighthouse) score:

![Lighthouse](https://i.imgur.com/JAuPJPF.png)

Styling is done with Tailwind CSS, but you can easily switch to whatever solution you prefer.

The frontend, backend, database and session store are all run as a multi-container Docker Compose application.

## Usage

1. Install Git and Docker.
2. Navigate to the directory where you want to clone this repository.
3. Run in the terminal:
```bash
git clone https://github.com/J-Schoepplenberg/royce.git
cd royce
docker-compose up --build
```
4. Navigate in your browser to `http://localhost:8000`.

## Development without Docker

1. Install rustup, NodeJS and PostgreSQL.
2. Start the PostgreSQL server on your machine.
3. Clone this repository `git clone https://github.com/J-Schoepplenberg/royce.git`.
4. Switch for the session store from `RedisStore` to `MemoryStore::default()` in `royce\backend\src\main.rs`.
    - Alternatively, you can install Redis on your machine and start the Redis server.
5. Open a new terminal in `royce\backend` and run `cargo run --release`.
6. Open a new terminal in `royce\frontend` and run `npm run dev`.

The frontend will now run separately from the backend with HMR enabled at `http://localhost:3000/`. 

The backend will be served at `http://localhost:8000/api/`.

## Why SolidJS?

This next-generation web framework looks and feels pretty much identical to React, but there is one big difference. Unlike React, it does not uselessly re-render components in cascade on every change, but is based on signals. 

To simplify, signals store and update values to represent state. But most importantly, they are reactive. Which means, they are smart enough to automatically propagate their latest value when they have changed to all places that depend on them.

In SolidJS things like components are functions that run once and never re-render again. Only signals are recomputed and updated. This is why SolidJS beats React by a mile in performance benchmarks.

## Why Axum?

You already know all the advantages of Rust: zero-cost abstractions, guaranteed memory safety, run-time speed and memory usage identical to C/C++, supreme ecosystem with Cargo, powerful type system.

Axum makes it extremely easy to write REST APIs and is backed by the Tokio team itself, which provides the de-facto standard runtime for async in Rust. Also, it is based on `hyper`, which is one of the fastest HTTP implementations that exist. It is fully interopabale with the `tower` ecosystem to easily plug-in any middleware you want. By adding extractors to routes (= arguments in the handler functions) you can super easily extract things (e.g. headers) from requests without any effort.