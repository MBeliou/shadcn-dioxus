# Development

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, and a `ui` crate for components that are shared between multiple platforms:

```
your_project/
├─ README.md
├─ Cargo.toml
└─ packages/
   ├─ web/
   │  └─ ... # Web specific UI/logic
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic
   └─  ui/
      └─ ... # Component shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:

```bash
dx serve
```

