use crate::tauri_bindings::{invoke, invoke_no_args};
use dioxus::prelude::*;
use xeton_models::{PowerAction, QueueItem};

#[component]
pub fn QueueManagerModal(on_close: EventHandler<()>) -> Element {
    let mut queues = use_signal(|| {
        vec![
            QueueItem {
                id: "default".to_string(),
                name: "Default Queue".to_string(),
                max_concurrent: 3,
                is_active: true,
                scheduled_start: None,
                scheduled_stop: None,
                power_action: PowerAction::None,
                items_count: 0,
            },
            QueueItem {
                id: "night".to_string(),
                name: "Night Queue".to_string(),
                max_concurrent: 5,
                is_active: false,
                scheduled_start: Some("02:00".to_string()),
                scheduled_stop: Some("08:00".to_string()),
                power_action: PowerAction::Sleep,
                items_count: 0,
            },
        ]
    });

    let mut selected_queue_idx = use_signal(|| 0usize);
    let mut is_creating_new = use_signal(|| false);
    let mut new_queue_name = use_signal(|| "".to_string());

    use_future(move || async move {
        if let Ok(list) = invoke_no_args::<Vec<QueueItem>>("get_queues").await {
            if !list.is_empty() {
                queues.set(list);
            }
        }
    });

    let current_queue = queues.read().get(selected_queue_idx()).cloned();

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-2xl overflow-hidden flex flex-col text-on-surface text-xs select-none",
                onclick: move |e| e.stop_propagation(),

                // Dialog Header
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-b border-border bg-surface-variant/30",
                    div {
                        class: "flex items-center gap-2.5",
                        svg {
                            class: "w-5 h-5 text-primary",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                            }
                        }
                        span { class: "font-semibold text-sm", "Queue Manager" }
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

                // Dialog Body (2 Columns: Queue List & Details)
                div {
                    class: "grid grid-cols-12 min-h-[380px] divide-x divide-border",

                    // Left Column: Queue List
                    div {
                        class: "col-span-4 p-3 bg-surface-variant/10 flex flex-col justify-between",
                        div {
                            class: "space-y-1 overflow-y-auto max-h-[320px]",
                            div {
                                class: "text-[11px] font-medium text-on-surface/40 uppercase tracking-wider px-2 py-1",
                                "Queues"
                            }
                            for (idx, q) in queues.read().iter().enumerate() {
                                div {
                                    key: "{q.id}",
                                    class: format!(
                                        "flex items-center justify-between px-2.5 py-2 rounded-lg cursor-pointer transition-colors text-xs {}",
                                        if selected_queue_idx() == idx { "bg-primary text-on-primary font-medium shadow-sm" } else { "hover:bg-surface-variant/50 text-on-surface" }
                                    ),
                                    onclick: move |_| selected_queue_idx.set(idx),
                                    div {
                                        class: "truncate flex-1 pr-1",
                                        "{q.name}"
                                    }
                                    if q.scheduled_start.is_some() {
                                        span {
                                            class: if selected_queue_idx() == idx { "text-on-primary/80" } else { "text-primary" },
                                            title: "Schedule active",
                                            "⏱"
                                        }
                                    }
                                }
                            }
                        }

                        // Add Queue Button
                        div {
                            class: "pt-3 border-t border-border mt-2",
                            if is_creating_new() {
                                div {
                                    class: "space-y-2",
                                    input {
                                        class: "w-full bg-background border border-border rounded-lg px-2.5 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                        placeholder: "Queue Name...",
                                        value: "{new_queue_name}",
                                        oninput: move |e| new_queue_name.set(e.value()),
                                    }
                                    div {
                                        class: "flex gap-2",
                                        button {
                                            class: "flex-1 px-2 py-1 bg-primary text-on-primary rounded text-[11px] font-medium hover:bg-primary-hover",
                                            onclick: move |_| {
                                                let name = new_queue_name().trim().to_string();
                                                if !name.is_empty() {
                                                    let id = format!("queue-{}", queues.read().len() + 1);
                                                    queues.write().push(QueueItem {
                                                        id,
                                                        name,
                                                        max_concurrent: 3,
                                                        is_active: false,
                                                        scheduled_start: None,
                                                        scheduled_stop: None,
                                                        power_action: PowerAction::None,
                                                        items_count: 0,
                                                    });
                                                    selected_queue_idx.set(queues.read().len() - 1);
                                                    new_queue_name.set("".to_string());
                                                    is_creating_new.set(false);
                                                }
                                            },
                                            "Add"
                                        }
                                        button {
                                            class: "px-2 py-1 bg-surface-variant text-on-surface/70 rounded text-[11px] hover:bg-surface-variant/80",
                                            onclick: move |_| is_creating_new.set(false),
                                            "Cancel"
                                        }
                                    }
                                }
                            } else {
                                button {
                                    class: "w-full py-1.5 px-3 border border-dashed border-border rounded-lg text-on-surface/60 hover:text-on-surface hover:border-primary/50 flex items-center justify-center gap-1.5 transition-colors text-xs",
                                    onclick: move |_| is_creating_new.set(true),
                                    span { "+" }
                                    span { "New Queue" }
                                }
                            }
                        }
                    }

                    // Right Column: Queue Settings / Scheduling
                    div {
                        class: "col-span-8 p-5 flex flex-col justify-between space-y-4",
                        if let Some(q) = current_queue {
                            div {
                                class: "space-y-4",
                                // Queue Name & Concurrency
                                div {
                                    class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Queue Name" }
                                        input {
                                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                            value: "{q.name}",
                                            oninput: move |e| {
                                                let idx = selected_queue_idx();
                                                if let Some(item) = queues.write().get_mut(idx) {
                                                    item.name = e.value();
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        label { class: "block text-[11px] font-medium text-on-surface/60 mb-1.5", "Max Concurrent Downloads" }
                                        input {
                                            r#type: "number",
                                            min: "1",
                                            max: "10",
                                            class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                            value: "{q.max_concurrent}",
                                            oninput: move |e| {
                                                if let Ok(val) = e.value().parse::<u32>() {
                                                    let idx = selected_queue_idx();
                                                    if let Some(item) = queues.write().get_mut(idx) {
                                                        item.max_concurrent = val.clamp(1, 10);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Scheduler Card
                                div {
                                    class: "border border-border rounded-xl p-4 bg-surface-variant/20 space-y-3",
                                    div {
                                        class: "flex items-center justify-between",
                                        div {
                                            class: "font-medium text-xs text-on-surface flex items-center gap-2",
                                            span { "Schedule Automation" }
                                        }
                                        label {
                                            class: "relative inline-flex items-center cursor-pointer",
                                            input {
                                                r#type: "checkbox",
                                                class: "sr-only peer",
                                                checked: q.scheduled_start.is_some(),
                                                onchange: move |e| {
                                                    let idx = selected_queue_idx();
                                                    if let Some(item) = queues.write().get_mut(idx) {
                                                        if e.checked() {
                                                            item.scheduled_start = Some("02:00".to_string());
                                                            item.scheduled_stop = Some("07:00".to_string());
                                                        } else {
                                                            item.scheduled_start = None;
                                                            item.scheduled_stop = None;
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "w-9 h-5 bg-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary" }
                                        }
                                    }

                                    if q.scheduled_start.is_some() {
                                        div {
                                            class: "grid grid-cols-2 gap-3 pt-2 animate-in fade-in duration-100",
                                            div {
                                                label { class: "block text-[11px] text-on-surface/60 mb-1", "Start Time" }
                                                input {
                                                    r#type: "time",
                                                    class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface",
                                                    value: q.scheduled_start.as_deref().unwrap_or("02:00"),
                                                    oninput: move |e| {
                                                        let idx = selected_queue_idx();
                                                        if let Some(item) = queues.write().get_mut(idx) {
                                                            item.scheduled_start = Some(e.value());
                                                        }
                                                    }
                                                }
                                            }
                                            div {
                                                label { class: "block text-[11px] text-on-surface/60 mb-1", "Stop Time" }
                                                input {
                                                    r#type: "time",
                                                    class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface",
                                                    value: q.scheduled_stop.as_deref().unwrap_or("07:00"),
                                                    oninput: move |e| {
                                                        let idx = selected_queue_idx();
                                                        if let Some(item) = queues.write().get_mut(idx) {
                                                            item.scheduled_stop = Some(e.value());
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        div {
                                            class: "pt-1",
                                            label { class: "block text-[11px] text-on-surface/60 mb-1", "Action On Complete" }
                                            select {
                                                class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                                value: match q.power_action {
                                                    PowerAction::None => "none",
                                                    PowerAction::Shutdown => "shutdown",
                                                    PowerAction::Sleep => "sleep",
                                                    PowerAction::Hibernate => "hibernate",
                                                },
                                                onchange: move |e| {
                                                    let action = match e.value().as_str() {
                                                        "shutdown" => PowerAction::Shutdown,
                                                        "sleep" => PowerAction::Sleep,
                                                        "hibernate" => PowerAction::Hibernate,
                                                        _ => PowerAction::None,
                                                    };
                                                    let idx = selected_queue_idx();
                                                    if let Some(item) = queues.write().get_mut(idx) {
                                                        item.power_action = action;
                                                    }
                                                },
                                                option { value: "none", "Do Nothing" }
                                                option { value: "sleep", "Put Computer to Sleep" }
                                                option { value: "hibernate", "Hibernate Computer" }
                                                option { value: "shutdown", "Shut Down Computer" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Bottom Actions
                        div {
                            class: "flex items-center justify-between pt-4 border-t border-border",
                            if selected_queue_idx() > 0 {
                                button {
                                    class: "px-3 py-1.5 text-error hover:bg-error/10 rounded-lg text-xs font-medium transition-colors",
                                    onclick: move |_| {
                                        let idx = selected_queue_idx();
                                        if idx < queues.read().len() && idx > 0 {
                                            let removed = queues.write().remove(idx);
                                            selected_queue_idx.set(idx - 1);
                                            spawn(async move {
                                                #[derive(serde::Serialize)]
                                                struct DelReq { id: String }
                                                let _ = invoke::<(), _>("delete_queue", &DelReq { id: removed.id }).await;
                                            });
                                        }
                                    },
                                    "Delete Queue"
                                }
                            } else {
                                div {}
                            }

                            div {
                                class: "flex items-center gap-2.5",
                                button {
                                    class: "px-4 py-1.5 bg-surface-variant hover:bg-surface-variant/80 text-on-surface rounded-lg text-xs font-medium transition-colors",
                                    onclick: move |_| on_close.call(()),
                                    "Cancel"
                                }
                                button {
                                    class: "px-5 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors",
                                    onclick: move |_| on_close.call(()),
                                    "Save & Close"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
