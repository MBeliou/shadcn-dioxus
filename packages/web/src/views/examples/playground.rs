use dioxus::prelude::*;
use lucide_dioxus::{HardDriveDownload, RotateCcw, SquarePen, Dna};
use ui::{
    Button,
    ButtonGroup::{Root as ButtonGroup, Separator},
    ButtonVariant, Label, NativeSelect, NativeSelectOptGroup, NativeSelectOption, Textarea,
};
#[component]
pub fn PlaygroundExample() -> Element {
    rsx! {
        div { class: "container-wrapper pb-6 ",
            div { class: "container scroll-mt-20 mx-auto",
                div { class: "border rounded-lg overflow-hidden bg-background flex-1 xl:rounded-xl",
                    div { class: "h-full flex flex-col",
                        div { class: "container flex flex-col items-start justify-between space-y-2 py-4 sm:flex-row sm:items-center sm:space-y-0 md:h-16 px-6",
                            h2 { class: "text-lg font-semibold", "Playground" }
                            div { class: "ms-auto flex w-full space-x-2 sm:justify-end",
                                Button { variant: ui::ButtonVariant::Secondary, "Save" }
                                Button { variant: ui::ButtonVariant::Secondary, "View code" }
                                Button { variant: ui::ButtonVariant::Secondary, "Share" }
                                Button { variant: ui::ButtonVariant::Secondary, "..." }
                            }
                        }
                        Separator { orientation: ui::separator::SeparatorOrientation::Horizontal }
                        div { class: "flex flex-col gap-2 flex-1 px-6",
                            div { class: "container h-full py-6",
                                div { class: "grid h-full items-stretch gap-6 md:grid-cols-[1fr_200px]",
                                    div { class: "flex-1 outline-none mt-0 border-0 p-0",
                                        div { class: "flex h-full flex-col space-y-4",
                                            Textarea {
                                                class: "grow min-h-[500px]",
                                                placeholder: "Write a tagline for an ice cream shop" }
                                            div { class: "flex items-center gap-2",
                                                Button { variant: ButtonVariant::Default,
                                                    "Submit"
                                                }
                                                Button { variant: ButtonVariant::Secondary,
                                                    RotateCcw {}
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-col space-y-4",
                                            div {
                                                Label { "Mode" }
                                                // TODO: replace with tabs once we've got them
                                                ButtonGroup {
                                                    class: "mt-2",
                                                    Button { variant: ButtonVariant::Secondary,
                                                        SquarePen{}
                                                    }
                                                    Button { variant: ButtonVariant::Secondary,
                                                     HardDriveDownload{}
                                                    }
                                                    Button { variant: ButtonVariant::Secondary,
                                                        Dna{}
                                                    }
                                                }
                                            },
                                            div {
                                                  Label { "Model" }
                                                  NativeSelect{
                                                    class: "mt-2",
                                                    NativeSelectOptGroup{
                                                        label: "GPT-3",
                                                        NativeSelectOption{
                                                        "text-davinci-003"
                                                    },
                                                    NativeSelectOption{
                                                        "text-davinci-003"
                                                    },
                                                    NativeSelectOption{
                                                        "text-davinci-003"
                                                    },
                                                    NativeSelectOption{
                                                        "text-davinci-003"
                                                    },
                                                    },
                                                    NativeSelectOptGroup{
                                                        label: "Codex",
                                                        NativeSelectOption{
                                                        "code-davinci-002"
                                                    },
                                                    NativeSelectOption{
                                                        "code-cushman-001"
                                                    },
                                                    }

                                                  }
                                            }
                                            div {
                                                div {
                                                    class: "flex items-center justify-between  pt-2",
                                                    Label{
                                                        "Temperature"
                                                    },
                                                    span {
                                                        class: "text-sm text-muted-foreground",
                                                        "1"
                                                     }

                                                },
                                                // TODO: replace with real progress component
                                                div {

                                                    class:"relative mt-4",
                                                    div {
                                                        class: "rounded-full w-full h-1.5 bg-primary",
                                                    },
                                                    div{
                                                        class: "absolute right-0 top-0 rounded-full bg-background border-primary size-4 translate-y-[-25%] border-2"
                                                    }
                                                }
                                            },
                                            div {
                                                div {
                                                    class: "flex items-center justify-between pt-2",
                                                    Label{
                                                        "Maximum Length"
                                                    },
                                                    span {
                                                        class: "text-sm text-muted-foreground",
                                                        "200"
                                                     }

                                                },
                                                // TODO: replace with real progress component
                                                div {
                                                    class:"relative flex items-center mt-4",
                                                    div {
                                                        class: "rounded-l-full w-[10%] h-1.5 bg-primary",
                                                    },
                                                    div {
                                                        class: "rounded-r-full grow h-1.5 bg-muted",
                                                    },
                                                    div{
                                                        class: "absolute left-[10%] top-0 rounded-full bg-background border-primary size-4 translate-y-[-25%] border-2"
                                                    }
                                                }
                                            }
                                            div {
                                                div {
                                                    class: "flex items-center justify-between  pt-2",
                                                    Label{
                                                        "Top P"
                                                    },
                                                    span {
                                                        class: "text-sm text-muted-foreground",
                                                        "0.9"
                                                     }

                                                },
                                                // TODO: replace with real progress component
                                                div {
                                                    class:"relative flex items-center mt-4",
                                                    div {
                                                        class: "rounded-l-full w-[90%] h-1.5 bg-primary",
                                                    },
                                                    div {
                                                        class: "rounded-r-full grow h-1.5 bg-muted",
                                                    },
                                                    div{
                                                        class: "absolute left-[90%] top-0 rounded-full bg-background border-primary size-4 translate-y-[-25%] border-2"
                                                    }
                                                }
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
