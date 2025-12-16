use dioxus::prelude::*;
use lucide_dioxus::Terminal;
use ui::cn;



/**
 * Base Cli Display block
 */
#[component]
pub fn PmBlock(command: String) -> Element {
    rsx! {
        figure {
            class: "bg-code rounded-md overflow-hidden text-code-foreground",
            div {
                class: "overflow-x-auto",
                div {
                    class: "border-border/50 flex items-center gap-2 border-b px-3 py-1",
                    div {
                        class: "bg-foreground  rounded-[1px] opacity-70",
                        Terminal{
                            class: "text-code size-4"
                        }
                    }
                    div {
                        "data-state": "active",
                        class: cn("data-[state=active]:bg-background dark:data-[state=active]:text-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30 text-foreground dark:text-muted-foreground inline-flex h-[calc(100%-1px)] items-center justify-center gap-1.5 whitespace-nowrap rounded-md border border-transparent px-2 py-1 text-sm font-medium transition-[color,box-shadow] focus-visible:outline-1 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 data-[state=active]:shadow-sm [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0","flex items-center data-[state=active]:bg-accent data-[state=active]:border-input h-7 border border-transparent pt-0.5  data-[state=active]:shadow-none self-start w-fit"),
                        "dx"
                    }
                }
            }
            div {
                class: "mt-0 px-4 py-3.5",
                pre {  
                    code {
                        class: "font-mono text-sm leading-none",
                        "data-language": "bash",
                        "{command}"
                    }
                }
            }
        }

    }
}
