use dioxus::prelude::*;
use ui::ButtonGroup;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 w-full ",
            div { class: "flex flex-col gap-2",
                div { class: "flex items-start justify-between",
                    h1 { class: "scroll-m-20 text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xl",
                        "Button"
                    }
                    ButtonGroup::Root {
                        ButtonGroup::Root {

                            ui::Button { variant: ui::ButtonVariant::Outline,
                                // size: ui::ButtonSize::Icon,
                                svg {
                                    class: "tabler-icon tabler-icon-copy",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M7 7m0 2.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667z" }
                                    path { d: "M4.012 16.737a2.005 2.005 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" }
                                }
                                "Copy Page"
                            }
                            ui::Button {
                                variant: ui::ButtonVariant::Outline,
                                size: ui::ButtonSize::Icon,
                                svg {
                                    class: "tabler-icon tabler-icon-copy",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M7 7m0 2.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667z" }
                                    path { d: "M4.012 16.737a2.005 2.005 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" }
                                }
                            }
                        
                        }
                    }
                }
            }
        }
    }
}
