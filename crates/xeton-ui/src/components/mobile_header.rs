use dioxus::prelude::*;
use crate::components::icons::{IconSearch, IconXetonLogo};
use crate::theme::ThemePalette;

#[component]
pub fn MobileHeader(
    theme: ThemePalette,
    title: String,
    on_search_click: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "xeton-mobile-header",
            div {
                style: "display: flex; align-items: center; gap: 10px;",
                IconXetonLogo { size: Some(28) }
                span {
                    style: "font-size: 16px; font-weight: 700; color: {theme.on_surface}; letter-spacing: -0.3px;",
                    "{title}"
                }
            }
            button {
                class: "xeton-touch-target",
                onclick: move |_| on_search_click.call(()),
                style: "
                    background: transparent; border: none; border-radius: 50%;
                    color: {theme.on_surface_muted};
                ",
                IconSearch { size: 20 }
            }
        }
    }
}
