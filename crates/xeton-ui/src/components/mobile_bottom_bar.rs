use dioxus::prelude::*;
use xeton_models::CategoryType;
use crate::components::icons::{IconMenu, IconPlus, IconSearch};
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
            class: "xeton-mobile-pill",
            // Menu Drawer Button (48x48dp touch target)
            button {
                class: "xeton-touch-target",
                onclick: move |_| on_menu_click.call(()),
                style: "
                    border-radius: 50%; border: none;
                    background-color: transparent; color: {theme.on_surface};
                    flex-shrink: 0;
                ",
                IconMenu { size: 20 }
            }

            // Search Toggle Button (48x48dp touch target)
            button {
                class: "xeton-touch-target",
                onclick: move |_| on_search_click.call(()),
                style: "
                    border-radius: 50%; border: none;
                    background-color: transparent; color: {theme.on_surface_muted};
                    flex-shrink: 0;
                ",
                IconSearch { size: 18 }
            }

            // Scrollable Categories
            div {
                style: "
                    display: flex; align-items: center; gap: 6px; flex: 1;
                    overflow-x: auto; scrollbar-width: none; padding: 0 4px;
                ",
                for (cat, label) in categories.iter() {
                    {
                        let is_selected = selected_category == *cat;
                        let bg = if is_selected { theme.primary } else { "transparent" };
                        let fg = if is_selected { theme.on_primary } else { theme.on_surface_muted };
                        let cat_copy = *cat;
                        rsx! {
                            button {
                                onclick: move |_| on_select_category.call(cat_copy),
                                style: "
                                    padding: 8px 14px; min-height: 36px; border-radius: 18px; border: none;
                                    background-color: {bg}; color: {fg};
                                    font-size: 13px; font-weight: 600; white-space: nowrap;
                                    cursor: pointer; flex-shrink: 0;
                                    transition: background-color 140ms var(--ease-expo), color 140ms var(--ease-expo);
                                ",
                                "{label}"
                            }
                        }
                    }
                }
            }

            // Add Download FAB (48x48dp touch target)
            button {
                class: "xeton-touch-target",
                onclick: move |_| on_add_click.call(()),
                style: "
                    border-radius: 50%; border: none;
                    background-color: {theme.primary}; color: {theme.on_primary};
                    flex-shrink: 0; box-shadow: 0 4px 14px rgba(71, 145, 191, 0.45);
                ",
                IconPlus { size: 22 }
            }
        }
    }
}
