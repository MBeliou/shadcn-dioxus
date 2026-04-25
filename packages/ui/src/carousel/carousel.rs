use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CarouselContext {
    pub current_index: Signal<usize>,
    pub total: Signal<usize>,
    pub set_index: Callback<usize>,
    pub orientation: CarouselOrientation,
    pub auto_play: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct CarouselProps {
    #[props(default)]
    pub orientation: CarouselOrientation,
    #[props(default = false)]
    pub auto_play: bool,
    #[props(default = 3000)]
    pub auto_play_interval_ms: u64,
    pub total: usize,
    #[props(default = "20rem".to_string())]
    pub height: String,
    #[props(default = "100%".to_string())]
    pub width: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| 0);
    let total = use_signal(|| props.total);

    let auto_play = props.auto_play;
    let auto_play_interval_ms = props.auto_play_interval_ms;

    let set_index = use_callback(move |idx: usize| {
        let t = total();
        if t > 0 {
            current_index.set(idx % t);
        }
    });

    use_context_provider(|| CarouselContext {
        current_index,
        total,
        set_index,
        orientation: props.orientation,
        auto_play,
    });

    use_future(move || async move {
        if !auto_play {
            return;
        }
        loop {
            futures_timer::Delay::new(std::time::Duration::from_millis(auto_play_interval_ms))
                .await;
            let t = total();
            if t > 0 {
                let curr = current_index();
                let next = (curr + 1) % t;
                current_index.set(next);
            }
        }
    });

    let size_style = match props.orientation {
        CarouselOrientation::Horizontal => format!("width: {};", props.width),
        CarouselOrientation::Vertical => {
            format!("width: {}; height: {};", props.width, props.height)
        }
    };

    rsx! {
        div {
            "data-slot": "carousel",
            "data-orientation": props.orientation.as_str(),
            role: "region",
            "aria-roledescription": "carousel",
            class: "relative overflow-hidden {props.class}",
            style: "{size_style}",
            ..props.attributes,
            {props.children}
        }
    }
}
