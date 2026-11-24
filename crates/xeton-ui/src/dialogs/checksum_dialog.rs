use dioxus::prelude::*;
use crate::tauri_bindings::invoke;
use xeton_models::{ChecksumAlgorithm, ChecksumRequest, ChecksumResult};

#[component]
pub fn ChecksumModal(
    file_path: Option<String>,
    on_close: EventHandler<()>,
) -> Element {
    let mut path = use_signal(|| file_path.unwrap_or_default());
    let mut algorithm = use_signal(|| ChecksumAlgorithm::Sha256);
    let mut calculated_hash = use_signal(|| "".to_string());
    let mut compare_hash = use_signal(|| "".to_string());
    let mut is_calculating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let is_matched = use_memo(move || {
        let calc = calculated_hash().trim().to_lowercase();
        let comp = compare_hash().trim().to_lowercase();
        !calc.is_empty() && !comp.is_empty() && calc == comp
    });

    let is_mismatched = use_memo(move || {
        let calc = calculated_hash().trim().to_lowercase();
        let comp = compare_hash().trim().to_lowercase();
        !calc.is_empty() && !comp.is_empty() && calc != comp
    });

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
                                d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                            }
                        }
                        span { class: "font-semibold text-sm", "Verify File Integrity (Checksum)" }
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

                    // File Path
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Target File" }
                        div {
                            class: "flex gap-2",
                            input {
                                class: "flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                placeholder: "/path/to/downloaded/file",
                                value: "{path}",
                                oninput: move |e| path.set(e.value()),
                            }
                            button {
                                class: "px-3 py-1.5 bg-surface-variant text-on-surface hover:bg-surface-variant/80 rounded-lg text-xs font-medium border border-border",
                                "Browse..."
                            }
                        }
                    }

                    // Algorithm Selector & Calculate
                    div {
                        class: "grid grid-cols-12 gap-3 items-end",
                        div {
                            class: "col-span-8",
                            label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Hash Algorithm" }
                            select {
                                class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                value: match algorithm() {
                                    ChecksumAlgorithm::Md5 => "md5",
                                    ChecksumAlgorithm::Sha1 => "sha1",
                                    ChecksumAlgorithm::Sha256 => "sha256",
                                    ChecksumAlgorithm::Sha512 => "sha512",
                                },
                                onchange: move |e| {
                                    let algo = match e.value().as_str() {
                                        "md5" => ChecksumAlgorithm::Md5,
                                        "sha1" => ChecksumAlgorithm::Sha1,
                                        "sha512" => ChecksumAlgorithm::Sha512,
                                        _ => ChecksumAlgorithm::Sha256,
                                    };
                                    algorithm.set(algo);
                                },
                                option { value: "sha256", "SHA-256 (Recommended)" }
                                option { value: "sha512", "SHA-512" }
                                option { value: "sha1", "SHA-1" }
                                option { value: "md5", "MD5" }
                            }
                        }
                        div {
                            class: "col-span-4",
                            button {
                                class: "w-full py-1.5 px-3 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors flex items-center justify-center gap-1.5 disabled:opacity-50",
                                disabled: path().trim().is_empty() || is_calculating(),
                                onclick: move |_| {
                                    is_calculating.set(true);
                                    error_msg.set(None);
                                    let p = path();
                                    let algo = algorithm();
                                    let exp = compare_hash();
                                    spawn(async move {
                                        let req = ChecksumRequest {
                                            file_path: p,
                                            algorithm: algo,
                                            expected_hash: if exp.trim().is_empty() { None } else { Some(exp) },
                                        };
                                        match invoke::<ChecksumResult, _>("calculate_checksum", &req).await {
                                            Ok(res) => calculated_hash.set(res.hash),
                                            Err(err) => error_msg.set(Some(err)),
                                        }
                                        is_calculating.set(false);
                                    });
                                },
                                if is_calculating() {
                                    span { class: "animate-spin", "⏳" }
                                    span { "Hashing..." }
                                } else {
                                    span { "Calculate" }
                                }
                            }
                        }
                    }

                    // Calculated Hash Result
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Calculated Hash" }
                        div {
                            class: "relative",
                            input {
                                readonly: true,
                                class: "w-full bg-surface-variant/30 border border-border rounded-lg px-3 py-1.5 text-xs font-mono text-on-surface focus:outline-none",
                                placeholder: "Calculated hash will appear here...",
                                value: "{calculated_hash}",
                            }
                            if !calculated_hash().is_empty() {
                                button {
                                    class: "absolute right-2 top-1.5 text-xs text-primary hover:text-primary-hover font-medium",
                                    onclick: move |_| {
                                        // Copy to clipboard
                                    },
                                    "Copy"
                                }
                            }
                        }
                    }

                    // Compare with Source
                    div {
                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Compare with Expected Hash" }
                        input {
                            class: format!(
                                "w-full bg-background border rounded-lg px-3 py-1.5 text-xs font-mono text-on-surface focus:outline-none {}",
                                if is_matched() {
                                    "border-emerald-500 bg-emerald-500/10 text-emerald-400"
                                } else if is_mismatched() {
                                    "border-error bg-error/10 text-error"
                                } else {
                                    "border-border focus:border-primary"
                                }
                            ),
                            placeholder: "Paste expected checksum to compare...",
                            value: "{compare_hash}",
                            oninput: move |e| compare_hash.set(e.value()),
                        }
                        if is_matched() {
                            div {
                                class: "flex items-center gap-1.5 text-xs text-emerald-400 mt-1.5 font-medium animate-in fade-in duration-150",
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M5 13l4 4L19 7"
                                    }
                                }
                                span { "Checksums match perfectly! File is authentic and uncorrupted." }
                            }
                        } else if is_mismatched() {
                            div {
                                class: "flex items-center gap-1.5 text-xs text-error mt-1.5 font-medium animate-in fade-in duration-150",
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
                                span { "Checksum mismatch! The file differs from expected hash." }
                            }
                        }
                    }
                }

                // Footer
                div {
                    class: "flex items-center justify-end px-5 py-3.5 border-t border-border bg-surface-variant/20",
                    button {
                        class: "px-5 py-1.5 bg-surface-variant hover:bg-surface-variant/80 text-on-surface rounded-lg text-xs font-medium transition-colors",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
