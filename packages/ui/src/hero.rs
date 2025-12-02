use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
    div {
        id: "hero",
        class: "container flex flex-col items-center gap-2 py-8 text-center md:py-16 lg:py-20 xl:gap-4",
        h1 { class: "text-primary leading-tighter text-4xl font-semibold tracking-tight text-balance lg:leading-[1.1] lg:font-semibold xl:text-5xl xl:tracking-tighter max-w-4xl",
            "The Foundation for your Design System"
        }
        p { class: "text-foreground max-w-3xl text-base text-balance sm:text-lg",
            "A set of beautifully designed components that you can customize, extend, and build on. Start here then make it your own. Open Source. Open Code."
        }
        div { class: "flex w-full items-center justify-center gap-2 pt-2 **:data-[slot=button]:shadow-none",
            a {
                class: "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*=size-])]:size-4 shrink-0 [&amp;_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 h-8 rounded-md gap-1.5 px-3 has-[&gt;svg]:px-2.5",
                "data-slot": "button",
                href: "/docs/components",
                "View Components"
            }
        }
    }
        }
}
