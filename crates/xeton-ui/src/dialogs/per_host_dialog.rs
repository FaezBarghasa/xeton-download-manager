use dioxus::prelude::*;
use xeton_models::PerHostConfig;

#[component]
pub fn PerHostModal(
    on_close: EventHandler<()>,
) -> Element {
    let mut hosts = use_signal(|| vec![
        PerHostConfig {
            host: "github.com".to_string(),
            max_connections: 8,
            speed_limit_bytes_per_sec: None,
            user_agent: None,
        },
        PerHostConfig {
            host: "archive.org".to_string(),
            max_connections: 4,
            speed_limit_bytes_per_sec: Some(2048 * 1024),
            user_agent: None,
        },
    ]);

    let mut new_host = use_signal(|| "".to_string());
    let mut new_conns = use_signal(|| 8u32);

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-xl overflow-hidden flex flex-col text-on-surface text-xs select-none",
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
                                d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
                            }
                        }
                        span { class: "font-semibold text-sm", "Per-Host Connection Rules" }
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
                        class: "text-[11px] text-on-surface/60",
                        "Customize concurrency limits and speed throttling per specific domain or server."
                    }

                    // Host Rules Table
                    div {
                        class: "border border-border rounded-lg overflow-hidden bg-background max-h-52 overflow-y-auto",
                        table {
                            class: "w-full text-left border-collapse",
                            thead {
                                class: "bg-surface-variant/40 border-b border-border text-[11px] text-on-surface/50 font-medium",
                                tr {
                                    th { class: "px-3 py-2", "Host Domain" }
                                    th { class: "px-3 py-2 w-24 text-center", "Max Conns" }
                                    th { class: "px-3 py-2 w-28 text-center", "Limit (KB/s)" }
                                    th { class: "px-3 py-2 w-16 text-center", "Action" }
                                }
                            }
                            tbody {
                                class: "divide-y divide-border/60",
                                for (idx, h) in hosts.read().iter().enumerate() {
                                    tr {
                                        key: "{h.host}",
                                        class: "hover:bg-surface-variant/20 transition-colors",
                                        td { class: "px-3 py-2 font-mono text-xs", "{h.host}" }
                                        td { class: "px-3 py-2 text-center", "{h.max_connections}" }
                                        td {
                                            class: "px-3 py-2 text-center text-on-surface/70",
                                            if let Some(limit) = h.speed_limit_bytes_per_sec {
                                                "{limit / 1024}"
                                            } else {
                                                "Unlimited"
                                            }
                                        }
                                        td {
                                            class: "px-3 py-2 text-center",
                                            button {
                                                class: "text-error hover:text-error/80 text-[11px]",
                                                onclick: move |_| {
                                                    hosts.write().remove(idx);
                                                },
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Add Host Row
                    div {
                        class: "p-3 bg-surface-variant/20 border border-border rounded-lg flex items-center gap-2",
                        input {
                            class: "flex-1 bg-background border border-border rounded-lg px-2.5 py-1.5 text-xs text-on-surface font-mono focus:outline-none focus:border-primary",
                            placeholder: "e.g., rapidgator.net or *.example.com",
                            value: "{new_host}",
                            oninput: move |e| new_host.set(e.value()),
                        }
                        input {
                            r#type: "number",
                            min: "1",
                            max: "32",
                            class: "w-20 bg-background border border-border rounded-lg px-2.5 py-1.5 text-xs text-on-surface text-center focus:outline-none focus:border-primary",
                            value: "{new_conns}",
                            oninput: move |e| {
                                if let Ok(val) = e.value().parse::<u32>() {
                                    new_conns.set(val);
                                }
                            }
                        }
                        button {
                            class: "px-3 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors disabled:opacity-50",
                            disabled: new_host().trim().is_empty(),
                            onclick: move |_| {
                                let h = new_host().trim().to_lowercase();
                                if !h.is_empty() {
                                    hosts.write().push(PerHostConfig {
                                        host: h,
                                        max_connections: new_conns(),
                                        speed_limit_bytes_per_sec: None,
                                        user_agent: None,
                                    });
                                    new_host.set("".to_string());
                                }
                            },
                            "Add Rule"
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
                        onclick: move |_| on_close.call(()),
                        "Save Rules"
                    }
                }
            }
        }
    }
}
