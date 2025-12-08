use crate::components::Hero;
use crate::Route;
use dioxus::prelude::*;
use ui::{Button, ButtonVariant};

#[component]
pub fn FullLayout() -> Element {
    rsx! {
        Hero {},
        div {
                class: "container-wrapper scroll-mt-24",
                id: "examples",
                div {
                    class: "container mx-auto flex items-center justify-between gap-4 py-4",
                    div {
                        class: "flex items-center [&>a:first-child]:text-primary flex-1 overflow-hidden",
                        div {
                            class: "flex items-center space-x-2",
                            Button {
                                variant: ButtonVariant::Ghost,
                                href: "/",
                                "Examples"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                disabled: true,
                                href: "/examples/dashboard",
                                "Dashboard"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                disabled: true,
                                href: "/examples/tasks",
                                "Tasks"
                            }
                             Button {
                                variant: ButtonVariant::Ghost,
                                href: "/examples/playground",
                                "Playground"
                            }

                            Button {
                                variant: ButtonVariant::Ghost,
                                disabled: true,
                                href: "/examples/authentication",
                                "Authentication"
                            }
                        }

                    }
                }
            },
        Outlet::<Route> {}
    }
}
