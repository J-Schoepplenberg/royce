# royce

Extremely simple `axum` + `SolidJS` starter for a fullstack web application.  

Deploy immediately with a ready-to-use preconfigured `Docker` deployment.

Fully fledged for high-performance:
- Frontend: fine-grained reactivity framework `SolidStart`
- Backend: multi-threaded asynchronous Rust backend `axum`
- Sessions: in-memory database `redis`
- Database: object-relational database `PostgreSQL`
- Proxy: asynchronous proxy server `Nginx`
- Deployment: container virtualization `Docker`

Contains everything you need:
- Authentication
- Sessions
- CORS
- Middleware
- Compression
- Rate limiting
- Request size limiting
- Signed and encrypted cookies

## Why SolidJS?

This next-generation web framework looks and feels pretty much identical to React, but there is one big difference. Unlike React, it does not uselessly re-render components in cascade on every change, but is based on signals. 

To simplify, signals store and update values to represent state. But most importantly, they are reactive. Which means, they are smart enough to automatically propagate their latest value when they have changed to all places that depend on them.

In SolidJS things like components only run once and never re-render again. Only signals are recomputed and updated. This is why SolidJS beats React by a mile in performance benchmarks.

## Why Axum?

You already know all the advantages of Rust. Zero-cost abstractions, guaranteed memory safety, run-time speed and memory usage identical to C/C++, supreme ecosystem with Cargo, powerful type system.

Axum makes it extremely easy to write REST APIs and is backed by the Tokio team itself, which provides the de-facto standard runtime for async in Rust. Also, it is based on `hyper`, which is one of the fastest HTTP implementations that exist. It is fully interopabale with the `tower` ecosystem to easily plug-in any middleware you want.