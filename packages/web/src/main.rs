use dioxus::prelude::*;
use ui::PortalProvider;
use web::routes::Route;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "The Component Library for Dioxus" }
        document::Meta { charset: "utf-8" }
        document::Meta { name: "description", content: "A set of beautifully designed components that you can customize, extend, and build on." }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { property: "og:title", content: "The Component Library for Dioxus" }
        document::Meta { property: "og:description", content: "A set of beautifully designed components that you can customize, extend, and build on." }
        document::Meta { property: "og:image", content: "https://shadcn-dioxus.com/assets/og.png" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:image", content: "https://shadcn-dioxus.com/assets/og.png" }
        document::Meta { name: "apple-mobile-web-app-title", content: "Shadcn Dioxus" }
        document::Link { rel: "icon", r#type: "image/png", href: "/favicon-96x96.png", sizes: "96x96" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/favicon.svg" }
        document::Link { rel: "shortcut icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/apple-touch-icon.png" }
        document::Link { rel: "manifest", href: "/site.webmanifest" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        PortalProvider {
            Router::<Route> {}
        }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new());
const _: Asset = asset!("/assets/og.png", AssetOptions::image().with_hash_suffix(false));
const _: Asset = asset!("/assets/web-app-manifest-192x192.png", AssetOptions::image().with_hash_suffix(false));
const _: Asset = asset!("/assets/web-app-manifest-512x512.png", AssetOptions::image().with_hash_suffix(false));
