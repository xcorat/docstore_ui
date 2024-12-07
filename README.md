# Test doc server

This project has a rust-rocket backend and a svelte frontend. To test, start the backend with `cargo run` and open the frontend in a browser with `npm run dev` (run `pnpm i` first).

The data is stored in a sqlite database, and the files are also stored alongside the database. The default path for the database is `<tmpdir>/docstore_files/docstore.db`, and the default files are stored in `<tmpdir>/docstore_files/<client_id>/filename.pdf`.


----
# Old README

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npx sv create

# create a new project in my-app
npx sv create my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
