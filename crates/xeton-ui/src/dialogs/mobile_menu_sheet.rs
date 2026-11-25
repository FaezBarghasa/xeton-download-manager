use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn MobileMenuSheet(
    theme: ThemePalette,
    on_close: EventHandler<()>,
    on_select_item: EventHandler<String>,
) -> Element {
    let items = [
        (
            "browser",
            "Browser",
            "M21 12a9 9 0 0 1-9 9m9-9a9 9 0 0 0-9-9m9 9H3m9 9a9 9 0 0 1-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9",
        ),
        ("resume_all", "Resume All", "M5 3l14 9-14 9V3z"),
        ("stop_all", "Stop All", "M6 6h12v12H6z"),
        (
            "delete",
            "Delete",
            "M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2",
        ),
        ("per_host", "Per Host Settings", "M4 6h16M4 12h16M4 18h16"),
        (
            "settings",
            "Settings",
            "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
        ),
        (
            "help",
            "Help",
            "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm1 14h-2v-2h2zm0-4h-2a2 2 0 1 1 2-2z",
        ),
        (
            "about",
            "About",
            "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 15h-1v-4h2v4zm0-6h-1V9h2v2z",
        ),
    ];

    rsx! {
        div {
            class: "xeton-sheet-backdrop",
            onclick: move |_| on_close.call(()),
            div {
                class: "xeton-sheet-panel",
                onclick: move |e| e.stop_propagation(),
                div {
                    style: "
                        width: 40px; height: 4px; border-radius: 2px; background-color: {theme.border};
                        margin: 0 auto 16px auto;
                    "
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    for (id, label, icon_path) in items.iter() {
                        {
                            let item_id = id.to_string();
                            rsx! {
                                button {
                                    onclick: move |_| on_select_item.call(item_id.clone()),
                                    style: "
                                        display: flex; align-items: center; gap: 14px; width: 100%;
                                        padding: 12px 14px; border-radius: 10px; border: none;
                                        background-color: transparent; color: {theme.on_surface};
                                        font-size: 15px; font-weight: 500; cursor: pointer; text-align: left;
                                    ",
                                    svg {
                                        view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor",
                                        fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "{icon_path}" }
                                    }
                                    span { "{label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
