use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ChartBar {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct ChartProps {
    #[props(default = 200.0)]
    pub height: f64,
    #[props(default = 400.0)]
    pub width: f64,
    #[props(default = 4.0)]
    pub bar_gap: f64,
    pub bars: Vec<ChartBar>,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Chart(props: ChartProps) -> Element {
    let max_val = props.bars.iter().map(|b| b.value).fold(0.0f64, f64::max);
    let bar_count = props.bars.len();
    let gap = props.bar_gap;
    let bar_width = if bar_count > 0 {
        (props.width - gap * (bar_count - 1) as f64) / bar_count as f64
    } else {
        0.0
    };
    let max = if max_val > 0.0 { max_val } else { 1.0 };
    let height = props.height;
    let bars = props.bars.clone();

    let rects: Vec<(String, f64, f64, f64, f64, String)> = if bar_count > 0 && max_val > 0.0 {
        bars
            .iter()
            .enumerate()
            .map(|(i, bar)| {
                let bar_height = (bar.value / max) * height;
                let x = i as f64 * (bar_width + gap);
                let y = height - bar_height;
                (
                    bar.label.clone(),
                    x,
                    y,
                    bar_width,
                    bar_height,
                    bar.color.clone(),
                )
            })
            .collect()
    } else {
        vec![]
    };

    rsx! {
        svg {
            width: "{props.width}",
            height: "{props.height}",
            "data-slot": "chart",
            class: "{props.class}",
            ..props.attributes,
            for (label, x, y, w, h, color) in rects {
                rect {
                    key: "{label}",
                    x: "{x}",
                    y: "{y}",
                    width: "{w}",
                    height: "{h}",
                    fill: "{color}",
                    rx: "2",
                }
            }
        }
    }
}
