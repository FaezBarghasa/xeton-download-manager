use dioxus::prelude::*;
use xeton_models::CategoryType;
use crate::theme::ThemePalette;

#[component]
pub fn MobileBottomBar(
    theme: ThemePalette,
    selected_category: CategoryType,
    on_select_category: EventHandler<CategoryType>,
    on_menu_click: EventHandler<()>,
    on_search_click: EventHandler<()>,
    on_add_click: EventHandler<()>,
) -> Element {
    let categories = [
        (CategoryType::All, "All"),
        (CategoryType::Compressed, "Compressed"),
        (CategoryType::Documents, "Documents"),
        (CategoryType::Music, "Music"),
        (CategoryType::Programs, "Programs"),
        (CategoryType::Videos, "Videos"),
        (CategoryType::Other, "Other"),
    ];

    rsx! {
        div {
            style: "
                position: fixed;
                bottom: 16px;
                left: 16px;
                right: 16px;
                height: 58px;
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 4px 6px;
                background-color: {theme.surface};
                border: 1px solid {theme.border};
                border-radius: 30px;
                box-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);
                box-sizing: border-box;
                z-index: 1000;
            ",
            // Menu Drawer Button
            button {
                onclick: move |_| on_menu_click.call(()),
                style: "
                    width: 44px; height: 44px; border-radius: 50%; border: none;
                    background-color: transparent; color: {theme.on_surface};
                    display: flex; align-items: center; justify-content: center;
                    cursor: pointer; flex-shrink: 0;
                ",
                svg {
                    view_box: "0 0 24 24", width: "20", height: "20",
                    stroke: "currentColor", fill: "none", stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    line { x1: "3", y1: "12", x2: "21", y2: "12" }
                    line { x1: "3", y1: "6", x2: "21", y2: "6" }
                    line { x1: "3", y1: "18", x2: "21", y2: "18" }
                }
            }
            // Search Button
            button {
                onclick: move |_| on_search_click.call(()),
                style: "
                    width: 44px; height: 44px; border-radius: 50%; border: none;
                    background-color: transparent; color: {theme.on_surface_muted};
                    display: flex; align-items: center; justify-content: center;
                    cursor: pointer; flex-shrink: 0;
                ",
                svg {
                    view_box: "0 0 24 24", width: "18", height: "18",
                    stroke: "currentColor", fill: "none", stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    circle { cx: "11", cy: "11", r: "8" }
                    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                }
            }
            // Horizontal Scrollable Category Pills
            div {
                style: "
                    flex: 1; display: flex; align-items: center; gap: 6px;
                    overflow-x: auto; scrollbar-width: none; -ms-overflow-style: none;
                    padding: 0 4px;
                ",
                for (cat, label) in categories.iter() {
                    {
                        let is_selected = selected_category == *cat;
                        let bg = if is_selected { theme.primary } else { "transparent" };
                        let fg = if is_selected { theme.on_primary } else { theme.on_surface_muted };
                        let current_cat = *cat;
                        rsx! {
                            button {
                                onclick: move |_| on_select_category.call(current_cat),
                                style: "
                                    padding: 6px 14px; border-radius: 16px; border: none;
                                    background-color: {bg}; color: {fg}; font-size: 13px; font-weight: 600;
                                    white-space: nowrap; cursor: pointer; flex-shrink: 0;
                                    transition: background-color 0.15s ease;
                                ",
                                "{label}"
                            }
                        }
                    }
                }
            }
            // Floating Action Add Button
            button {
                onclick: move |_| on_add_click.call(()),
                style: "
                    width: 44px; height: 44px; border-radius: 50%; border: none;
                    background-color: {theme.primary}; color: {theme.on_primary};
                    display: flex; align-items: center; justify-content: center;
                    cursor: pointer; flex-shrink: 0; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
                ",
                svg {
                    view_box: "0 0 24 24", width: "22", height: "22",
                    stroke: "currentColor", fill: "none", stroke_width: "2.5",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    line { x1: "12", y1: "5", x2: "12", y2: "19" }
                    line { x1: "5", y1: "12", x2: "19", y2: "12" }
                }
            }
        }
    }
}
