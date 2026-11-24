use dioxus::prelude::*;

#[component]
pub fn IconClose(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(16);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "18", y1: "6", x2: "6", y2: "18" }
            line { x1: "6", y1: "6", x2: "18", y2: "18" }
        }
    }
}

#[component]
pub fn IconMinimize(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(16);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "5", y1: "12", x2: "19", y2: "12" }
        }
    }
}

#[component]
pub fn IconMaximize(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(16);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "3", width: "18", height: "18", rx: "2", ry: "2" }
        }
    }
}

#[component]
pub fn IconPlus(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(18);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "12", y1: "5", x2: "12", y2: "19" }
            line { x1: "5", y1: "12", x2: "19", y2: "12" }
        }
    }
}

#[component]
pub fn IconResume(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(18);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "{c}",
            polygon { points: "5 3 19 12 5 21 5 3" }
        }
    }
}

#[component]
pub fn IconPause(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(18);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "{c}",
            rect { x: "6", y: "4", width: "4", height: "16" }
            rect { x: "14", y: "4", width: "4", height: "16" }
        }
    }
}

#[component]
pub fn IconDelete(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(18);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "3 6 5 6 21 6" }
            path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
        }
    }
}

#[component]
pub fn IconSettings(size: Option<u32>, color: Option<String>) -> Element {
    let s = size.unwrap_or(18);
    let c = color.unwrap_or_else(|| "currentColor".into());
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{c}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "3" }
            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
        }
    }
}
