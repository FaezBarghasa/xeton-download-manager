use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmKind {
    DeleteDownload,
    ExitApp,
}

#[component]
pub fn ConfirmModal(
    kind: ConfirmKind,
    title: String,
    message: String,
    on_close: EventHandler<()>,
    on_confirm: EventHandler<bool>, // bool = extra toggle (e.g. "delete file from disk")
) -> Element {
    let mut delete_from_disk = use_signal(|| false);

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-sm overflow-hidden flex flex-col text-on-surface text-xs select-none",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-b border-border bg-surface-variant/30",
                    div {
                        class: "flex items-center gap-2",
                        svg {
                            class: "w-5 h-5 text-amber-500",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                            }
                        }
                        span { class: "font-semibold text-sm", "{title}" }
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
                        class: "text-xs text-on-surface/80 leading-relaxed",
                        "{message}"
                    }

                    if kind == ConfirmKind::DeleteDownload {
                        div {
                            class: "pt-1 flex items-center gap-2 cursor-pointer",
                            label {
                                class: "flex items-center gap-2 text-xs text-on-surface/90 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                    checked: delete_from_disk(),
                                    onchange: move |e| delete_from_disk.set(e.checked()),
                                }
                                span { "Also delete downloaded file from disk" }
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
                        class: "px-5 py-1.5 bg-error hover:bg-error/90 text-white rounded-lg text-xs font-medium shadow-sm transition-colors",
                        onclick: move |_| {
                            on_confirm.call(delete_from_disk());
                            on_close.call(());
                        },
                        "Confirm"
                    }
                }
            }
        }
    }
}
