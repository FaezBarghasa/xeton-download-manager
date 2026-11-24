use dioxus::prelude::*;
use crate::theme::ThemePalette;

#[component]
pub fn MobileHeader(
    theme: ThemePalette,
    title: String,
    on_search_click: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 12px 16px;
                background-color: {theme.surface};
                border-bottom: 1px solid {theme.border};
                user-select: none;
                height: 56px;
                box-sizing: border-box;
            ",
            div {
                style: "display: flex; align-items: center; gap: 10px;",
                // Xeton Authentic Hexagonal Vector Logo
                svg {
                    view_box: "0 0 100 100",
                    width: "28",
                    height: "28",
                    polygon {
                        points: "50,5 95,27.5 95,72.5 50,95 5,72.5 5,27.5",
                        fill: "{theme.primary}",
                    }
                    path {
                        d: "M 32 30 L 48 50 L 32 70 L 42 70 L 52 57 L 62 70 L 72 70 L 56 50 L 72 30 L 62 30 L 52 43 L 42 30 Z",
                        fill: "#FFFFFF"
                    }
                }
                span {
                    style: "font-size: 17px; font-weight: 700; color: {theme.on_surface}; letter-spacing: -0.3px;",
                    "{title}"
                }
            }
            button {
                onclick: move |_| on_search_click.call(()),
                style: "
                    background: transparent; border: none; padding: 6px; border-radius: 50%;
                    color: {theme.on_surface_muted}; cursor: pointer; display: flex; align-items: center; justify-content: center;
                ",
                svg {
                    view_box: "0 0 24 24",
                    width: "20",
                    height: "20",
                    stroke: "currentColor",
                    fill: "none",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "11", cy: "11", r: "8" }
                    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                }
            }
        }
    }
}
