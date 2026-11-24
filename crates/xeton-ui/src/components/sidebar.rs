use dioxus::prelude::*;
use crate::theme::ThemePalette;
use xeton_models::CategoryType;

#[component]
pub fn Sidebar(
    theme: ThemePalette,
    selected_category: Signal<CategoryType>,
    download_counts: ReadOnlySignal<std::collections::HashMap<CategoryType, usize>>,
) -> Element {
    let categories = [
        (CategoryType::All, "All Downloads"),
        (CategoryType::Compressed, "Compressed"),
        (CategoryType::Documents, "Documents"),
        (CategoryType::Music, "Music"),
        (CategoryType::Programs, "Programs"),
        (CategoryType::Videos, "Videos"),
        (CategoryType::Other, "Other"),
    ];

    rsx! {
        div {
            class: "sidebar",
            style: "
                width: 220px;
                background-color: {theme.surface};
                border-right: 1px solid {theme.border};
                display: flex;
                flex-direction: column;
                padding: 12px 8px;
                height: 100%;
                box-sizing: border-box;
            ",
            div {
                style: "font-size: 11px; font-weight: 700; text-transform: uppercase; color: {theme.primary}; padding: 4px 8px; margin-bottom: 4px;",
                "Categories"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                for (cat, label) in categories.iter() {
                    {
                        let is_selected = *selected_category.read() == *cat;
                        let bg = if is_selected { theme.primary } else { "transparent" };
                        let fg = if is_selected { theme.on_primary } else { theme.on_surface };
                        let cat_copy = *cat;
                        let count = download_counts.read().get(cat).copied().unwrap_or(0);
                        rsx! {
                            button {
                                onclick: move |_| {
                                    *selected_category.write() = cat_copy;
                                },
                                style: "
                                    display: flex;
                                    align-items: center;
                                    justify-content: space-between;
                                    padding: 8px 12px;
                                    border-radius: 6px;
                                    border: none;
                                    background-color: {bg};
                                    color: {fg};
                                    font-size: 13px;
                                    font-weight: 500;
                                    cursor: pointer;
                                    text-align: left;
                                    transition: background-color 0.15s ease;
                                ",
                                span { "{label}" }
                                span {
                                    style: "font-size: 11px; opacity: 0.8;",
                                    "{count}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
