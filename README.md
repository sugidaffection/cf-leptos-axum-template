# Leptos

This template demonstrates the use of the [Leptos](https://leptos.dev/)
framework on Workers, including support for server side rendering and
server functions.

Frontend assets are built using `cargo leptos` by compiling the crate
with the `hydrate` feature. The backend module uses `workers-rs` and
is built by compiling the crate using `worker-build` with the `ssr`
feature. This is done automatically when using `wrangler` with
the custom build command specified in `wrangler.toml`.

Frontend assets are served using Workers Assets. Any request which
matches an asset path will be served directly and not invoke the
Worker. Requests which do not match an asset path will invoke the
Worker. This includes requests to `index.html` (which will be
server-side rendered) and any server function (API) routes.

# Using This Template

This template is designed to be used with `cargo generate`. To create a new project based on this template, you need to install `cargo-generate` first:

```
cargo install cargo-generate
```

Then you can create a new project using this template:

```
cargo generate --git https://github.com/sugidaffection/cf-leptos-axum-template
```

This will prompt you to enter a project name and whether you want to use nightly features.

# Setup

## Prerequisites

Before you can use this template, you need to install the following:

1. [Node.js](https://nodejs.org/) (LTS version recommended) and npm
   - Alternatively, you can use [Bun](https://bun.sh/) or [Yarn](https://yarnpkg.com/)

2. [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos) is required
   to build the project.

```
cargo install --locked cargo-leptos
```

3. Wrangler CLI for Cloudflare Workers

With npm (recommended):
```
npm install -D wrangler@latest
```

With Bun:
```
bun install -D wrangler@latest
```

With Yarn:
```
yarn add -D wrangler@latest
```

Note: Cloudflare recommends installing Wrangler locally in your project rather than globally.

# Run Locally

```
npx wrangler dev
```

Or if you're using Bun:
```
bunx wrangler dev
```

Or if you're using Yarn:
```
yarn wrangler dev
```

# Deploy

```
npx wrangler deploy
```

Or if you're using Bun:
```
bunx wrangler deploy
```

Or if you're using Yarn:
```
yarn wrangler deploy
```

# Run Development Server

For local development, you can also use the cargo-leptos development server which provides hot reloading:

```
cargo leptos watch --bin-features tokio
```

This command runs the project using the tokio runtime, providing a development server with automatic reloading when you make changes to your code.