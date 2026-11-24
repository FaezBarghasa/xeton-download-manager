use dioxus::prelude::*;
use xeton_models::DownloadItem;

#[component]
pub fn EditDownloadModal(
    item: DownloadItem,
    on_close: EventHandler<()>,
    on_save: EventHandler<(String, String, String)>, // (id, new_url, new_name)
) -> Element {
    let mut url = use_signal(|| item.url.clone());
    let mut name = use_signal(|| item.name.clone());
    let mut save_path = use_signal(|| item.file_path.clone());

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col text-on-surface text-xs select-none",
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
                                d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                            }
                        }
                        span { class: "font-semibold text-sm", "Edit Download / Refresh Link" }
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

                    div {
                        class: "p-3 bg-amber-500/10 border border-amber-500/20 rounded-lg text-amber-400 text-[11px] flex items-start gap-2",
                        span { "💡" }
                        span { "If a download expired or returned 403 Forbidden, generate a fresh download link in your browser and replace the URL below to resume without losing downloaded parts." }
                    }

                    // Download URL
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Download URL (Address)" }
                        input {
                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface font-mono focus:outline-none focus:border-primary",
                            value: "{url}",
                            oninput: move |e| url.set(e.value()),
                        }
                    }

                    // File Name
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "File Name" }
                        input {
                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                        }
                    }

                    // Save Location
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Save Location" }
                        div {
                            class: "flex gap-2",
                            input {
                                class: "flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                value: "{save_path}",
                                oninput: move |e| save_path.set(e.value()),
                            }
                            button {
                                class: "px-3 py-1.5 bg-surface-variant text-on-surface hover:bg-surface-variant/80 rounded-lg text-xs font-medium border border-border",
                                "Change..."
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
                        class: "px-5 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors",
                        onclick: {
                            let item_id = item.id.clone();
                            move |_| {
                                on_save.call((item_id.clone(), url().trim().to_string(), name().trim().to_string()));
                                on_close.call(());
                            }
                        },
                        "Save Changes"
                    }
                }
            }
        }
    }
}
