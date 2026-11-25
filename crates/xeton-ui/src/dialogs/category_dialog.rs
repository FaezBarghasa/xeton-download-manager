use dioxus::prelude::*;
use xeton_models::CustomCategory;

#[component]
pub fn CategoryModal(
    category: Option<CustomCategory>,
    on_close: EventHandler<()>,
    on_save: EventHandler<CustomCategory>,
) -> Element {
    let initial_cat = category.clone().unwrap_or_else(|| CustomCategory {
        id: "".to_string(),
        name: "".to_string(),
        icon: "folder".to_string(),
        color: "#3b82f6".to_string(),
        extensions: vec![],
        default_path: None,
    });

    let mut name = use_signal(|| initial_cat.name);
    let mut color = use_signal(|| initial_cat.color);
    let mut extensions_str = use_signal(|| initial_cat.extensions.join(", "));
    let mut default_path = use_signal(|| initial_cat.default_path.unwrap_or_default());

    let preset_colors = vec![
        "#3b82f6", "#ef4444", "#10b981", "#f59e0b", "#8b5cf6", "#ec4899", "#06b6d4", "#64748b",
    ];

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-md overflow-hidden flex flex-col text-on-surface text-xs select-none",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-b border-border bg-surface-variant/30",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            class: "w-4 h-4 text-primary",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
                            }
                        }
                        span { class: "font-semibold text-sm", if category.is_some() { "Edit Category" } else { "New Category" } }
                    }
                    button {
                        class: "p-1 rounded-md text-on-surface/50 hover:text-on-surface hover:bg-surface-variant/50 transition-colors",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-4 h-4",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M6 18L18 6M6 6l12 12"
                            }
                        }
                    }
                }

                // Body
                div {
                    class: "p-5 space-y-4",

                    // Category Name
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Category Name" }
                        input {
                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                            placeholder: "e.g., Compressed, Torrents, ISOs",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                        }
                    }

                    // Color Picker
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-2", "Label Color" }
                        div {
                            class: "flex items-center gap-2.5",
                            for c in preset_colors {
                                button {
                                    key: "{c}",
                                    class: format!(
                                        "w-6 h-6 rounded-full transition-transform {} hover:scale-110",
                                        if color() == c { "ring-2 ring-primary ring-offset-2 ring-offset-surface scale-110" } else { "" }
                                    ),
                                    style: format!("background-color: {}", c),
                                    onclick: move |_| color.set(c.to_string()),
                                }
                            }
                        }
                    }

                    // File Extensions
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Associated File Extensions (comma-separated)" }
                        input {
                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                            placeholder: "zip, rar, 7z, tar.gz",
                            value: "{extensions_str}",
                            oninput: move |e| extensions_str.set(e.value()),
                        }
                        div {
                            class: "text-[10px] text-on-surface/40 mt-1",
                            "Downloads matching these extensions will automatically assign to this category."
                        }
                    }

                    // Default Save Path
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Default Download Directory (Optional)" }
                        div {
                            class: "flex gap-2",
                            input {
                                class: "flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                placeholder: "Default folder...",
                                value: "{default_path}",
                                oninput: move |e| default_path.set(e.value()),
                            }
                            button {
                                class: "px-3 py-1.5 bg-surface-variant text-on-surface hover:bg-surface-variant/80 rounded-lg text-xs font-medium border border-border",
                                "Browse"
                            }
                        }
                    }
                }

                // Footer
                div {
                    class: "flex items-center justify-end gap-2.5 px-5 py-3.5 border-t border-border bg-surface-variant/20",
                    button {
                        class: "px-4 py-1.5 bg-surface-variant hover:bg-surface-variant/80 text-on-surface rounded-lg text-xs font-medium transition-colors",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "px-5 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors disabled:opacity-50",
                        disabled: name().trim().is_empty(),
                        onclick: move |_| {
                            let exts: Vec<String> = extensions_str()
                                .split(',')
                                .map(|s| s.trim().trim_start_matches('.').to_lowercase())
                                .filter(|s| !s.is_empty())
                                .collect();

                            let path_opt = if default_path().trim().is_empty() {
                                None
                            } else {
                                Some(default_path().trim().to_string())
                            };

                            let cat = CustomCategory {
                                id: if let Some(ref c) = category { c.id.clone() } else { format!("cat-{}", name().to_lowercase().replace(' ', "-")) },
                                name: name().trim().to_string(),
                                icon: "folder".to_string(),
                                color: color(),
                                extensions: exts,
                                default_path: path_opt,
                            };
                            on_save.call(cat);
                            on_close.call(());
                        },
                        "Save"
                    }
                }
            }
        }
    }
}
