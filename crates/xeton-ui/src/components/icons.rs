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

#[component]
pub fn IconXetonLogo(size: Option<u32>) -> Element {
    let s = size.unwrap_or(22);
    rsx! {
        svg {
            width: "{s}",
            height: "{s}",
            view_box: "0 0 512 512",
            fill: "none",
            rect {
                x: "20", y: "20", width: "472", height: "472", rx: "112",
                fill: "#0B0F19",
                stroke: "url(#xeton_rim)",
                stroke_width: "8"
            }
            defs {
                linearGradient {
                    id: "xeton_rim",
                    x1: "32", y1: "32", x2: "480", y2: "480",
                    gradient_units: "userSpaceOnUse",
                    stop { offset: "0%", stop_color: "#00F2FE" }
                    stop { offset: "50%", stop_color: "#8B5CF6" }
                    stop { offset: "100%", stop_color: "#EC4899" }
                }
                linearGradient {
                    id: "xeton_cyan",
                    x1: "120", y1: "100", x2: "280", y2: "340",
                    gradient_units: "userSpaceOnUse",
                    stop { offset: "0%", stop_color: "#38BDF8" }
                    stop { offset: "100%", stop_color: "#0284C7" }
                }
                linearGradient {
                    id: "xeton_magenta",
                    x1: "392", y1: "100", x2: "232", y2: "340",
                    gradient_units: "userSpaceOnUse",
                    stop { offset: "0%", stop_color: "#F472B6" }
                    stop { offset: "100%", stop_color: "#9333EA" }
                }
                linearGradient {
                    id: "xeton_core",
                    x1: "256", y1: "180", x2: "256", y2: "400",
                    gradient_units: "userSpaceOnUse",
                    stop { offset: "0%", stop_color: "#FFFFFF" }
                    stop { offset: "40%", stop_color: "#38BDF8" }
                    stop { offset: "100%", stop_color: "#0369A1" }
                }
            }
            path {
                d: "M124 120 C124 114, 130 110, 136 112 L206 136 C214 139, 219 146, 222 154 L256 244 L212 284 L142 168 C138 162, 134 152, 128 142 Z",
                fill: "url(#xeton_cyan)"
            }
            path {
                d: "M388 120 C388 114, 382 110, 376 112 L306 136 C298 139, 293 146, 290 154 L256 244 L300 284 L370 168 C374 162, 378 152, 384 142 Z",
                fill: "url(#xeton_magenta)"
            }
            path {
                d: "M136 324 L204 364 C212 369, 218 377, 220 386 L226 410 C228 418, 220 424, 212 420 L148 376 C138 369, 132 358, 132 346 L134 330 C134 324, 140 320, 146 324 Z",
                fill: "#0284C7",
                opacity: "0.6"
            }
            path {
                d: "M376 324 L308 364 C300 369, 294 377, 292 386 L286 410 C284 418, 292 424, 300 420 L364 376 C374 369, 380 358, 380 346 L378 330 C378 324, 372 320, 366 324 Z",
                fill: "#9333EA",
                opacity: "0.6"
            }
            path {
                d: "M256 188 L302 248 L272 248 L272 312 L240 312 L240 248 L210 248 Z",
                fill: "url(#xeton_core)"
            }
            path {
                d: "M256 296 L272 312 L256 392 L240 312 Z",
                fill: "url(#xeton_core)"
            }
            polygon {
                points: "256,210 270,234 256,252 242,234",
                fill: "#FFFFFF"
            }
        }
    }
}
