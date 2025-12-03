use dioxus::prelude::*;

use crate::cn;


#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Email,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DatetimeLocal,
    Month,
    Week,
    Color,
    Hidden,
    File,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Date => "date",
            Self::Time => "time",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Color => "color",
            Self::Hidden => "hidden",
            Self::File => "file",
        }
    }
}

const INPUT_BASE: &str = "selection:bg-primary selection:text-primary-foreground border-input ring-offset-background placeholder:text-muted-foreground shadow-xs flex h-9 w-full min-w-0 rounded-md border px-3 text-base outline-none transition-[color,box-shadow] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

const INPUT_DEFAULT: &str = "bg-background dark:bg-input/30 py-1";
const INPUT_FILE: &str = "dark:bg-input/30 bg-transparent pt-1.5 font-medium";

pub fn input_classes(input_type: InputType) -> String {
    match input_type {
        InputType::File => format!("{} {}", INPUT_BASE, INPUT_FILE),
        _ => format!("{} {}", INPUT_BASE, INPUT_DEFAULT),
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    #[props(default)]
    pub r#type: InputType,

    #[props(into, default)]
    pub class: String,

    #[props(into, default = "input".to_string())]
    pub data_slot: String,

    // Event handlers
    pub oninput: Option<EventHandler<FormEvent>>,
    pub onchange: Option<EventHandler<FormEvent>>,
    pub oninvalid: Option<EventHandler<FormEvent>>,
    pub onselect: Option<EventHandler<SelectionEvent>>,
    pub onfocus: Option<EventHandler<FocusEvent>>,
    pub onblur: Option<EventHandler<FocusEvent>>,
    pub onfocusin: Option<EventHandler<FocusEvent>>,
    pub onfocusout: Option<EventHandler<FocusEvent>>,
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    pub oncompositionstart: Option<EventHandler<CompositionEvent>>,
    pub oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    pub oncompositionend: Option<EventHandler<CompositionEvent>>,
    pub oncopy: Option<EventHandler<ClipboardEvent>>,
    pub oncut: Option<EventHandler<ClipboardEvent>>,
    pub onpaste: Option<EventHandler<ClipboardEvent>>,

    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let base_classes = input_classes(props.r#type);
    let classes = cn(&base_classes, &props.class);

    rsx! {
        input {
            "data-slot": "{props.data_slot}",
            r#type: props.r#type.as_str(),
            class: "{classes}",

            oninput: move |e| _ = props.oninput.map(|cb| cb(e)),
            onchange: move |e| _ = props.onchange.map(|cb| cb(e)),
            oninvalid: move |e| _ = props.oninvalid.map(|cb| cb(e)),
            onselect: move |e| _ = props.onselect.map(|cb| cb(e)),
            onfocus: move |e| _ = props.onfocus.map(|cb| cb(e)),
            onblur: move |e| _ = props.onblur.map(|cb| cb(e)),
            onfocusin: move |e| _ = props.onfocusin.map(|cb| cb(e)),
            onfocusout: move |e| _ = props.onfocusout.map(|cb| cb(e)),
            onkeydown: move |e| _ = props.onkeydown.map(|cb| cb(e)),
            onkeypress: move |e| _ = props.onkeypress.map(|cb| cb(e)),
            onkeyup: move |e| _ = props.onkeyup.map(|cb| cb(e)),
            oncompositionstart: move |e| _ = props.oncompositionstart.map(|cb| cb(e)),
            oncompositionupdate: move |e| _ = props.oncompositionupdate.map(|cb| cb(e)),
            oncompositionend: move |e| _ = props.oncompositionend.map(|cb| cb(e)),
            oncopy: move |e| _ = props.oncopy.map(|cb| cb(e)),
            oncut: move |e| _ = props.oncut.map(|cb| cb(e)),
            onpaste: move |e| _ = props.onpaste.map(|cb| cb(e)),

            ..props.attributes,
        }
    }
}
