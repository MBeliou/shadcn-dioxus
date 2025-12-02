use dioxus::prelude::*;
use lucide_dioxus::LoaderCircle;

use crate::cn;
/*
<script lang="ts">
    import { cn } from "$lib/utils.js";
    import Loader2Icon from "@lucide/svelte/icons/loader-2";
    import type { ComponentProps } from "svelte";

    let { class: className, ...restProps }: ComponentProps<typeof Loader2Icon> = $props();
</script>

<Loader2Icon
    role="status"
    aria-label="Loading"
    class={cn("size-4 animate-spin", className)}
    {...restProps}
/>
 */
#[derive(Clone, PartialEq, Props)]
pub struct SpinnerProps {
    #[props(into, default)]
    pub class: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    rsx! {
        LoaderCircle {
            class: cn("size-4 animate-spin", &props.class)

        }
    }
}
