use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn MobileSetupWizard(theme: ThemePalette, on_finish: EventHandler<()>) -> Element {
    let mut step = use_signal(|| 1);
    let mut selected_language = use_signal(|| "System (English)".to_string());
    let mut selected_theme = use_signal(|| "Dark".to_string());
    let mut is_storage_granted = use_signal(|| false);
    let mut is_notification_granted = use_signal(|| false);

    rsx! {
        div {
            style: "
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background-color: {theme.background}; color: {theme.on_background};
                display: flex; flex-direction: column; justify-content: space-between;
                padding: 24px 20px; box-sizing: border-box; z-index: 3000;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            ",
            // Top Bar with Back or Header
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 48px; position: relative;",
                if *step.read() > 1 {
                    button {
                        onclick: move |_| {
                            let curr = *step.read();
                            if curr > 1 {
                                step.set(curr - 1);
                            }
                        },
                        style: "
                            position: absolute; left: 0; top: 8px; width: 40px; height: 40px;
                            border-radius: 50%; border: none; background: transparent;
                            color: {theme.on_surface}; display: flex; align-items: center; justify-content: center;
                            cursor: pointer;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "22", height: "22", stroke: "currentColor",
                            fill: "none", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                            line { x1: "19", y1: "12", x2: "5", y2: "12" }
                            polyline { points: "12 19 5 12 12 5" }
                        }
                    }
                }
                span {
                    style: "font-size: 18px; font-weight: 700; color: {theme.on_surface};",
                    if *step.read() == 1 {
                        "Xeton Download Manager"
                    } else {
                        "Permissions"
                    }
                }
            }

            // Central Content Area
            div {
                style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 12px 0;",
                if *step.read() == 1 {
                    // Step 1: Welcome, Language & Theme
                    div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center;",
                        svg {
                            view_box: "0 0 100 100", width: "72", height: "72",
                            polygon { points: "50,5 95,27.5 95,72.5 50,95 5,72.5 5,27.5", fill: "{theme.primary}" }
                            path { d: "M 32 30 L 48 50 L 32 70 L 42 70 L 52 57 L 62 70 L 72 70 L 56 50 L 72 30 L 62 30 L 52 43 L 42 30 Z", fill: "#FFFFFF" }
                        }
                        h2 { style: "font-size: 24px; font-weight: 700; margin: 20px 0 4px 0;", "Welcome" }
                        p { style: "font-size: 15px; color: {theme.on_surface_muted}; margin: 0 0 32px 0;", "Let's set things up" }

                        // Language Preference Card
                        div {
                            style: "width: 100%; background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 14px 16px; margin-bottom: 12px;",
                            div { style: "font-size: 13px; color: {theme.on_surface_muted}; margin-bottom: 4px;", "Language" }
                            div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface};", "{selected_language}" }
                        }

                        // Theme Preference Card
                        div {
                            style: "width: 100%; background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 14px 16px; margin-bottom: 24px;",
                            div { style: "font-size: 13px; color: {theme.on_surface_muted}; margin-bottom: 4px;", "Theme" }
                            div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface};", "{selected_theme}" }
                        }
                        span { style: "font-size: 12px; color: {theme.on_surface_muted}; text-align: center;", "You can change these settings anytime later" }
                    }
                } else if *step.read() == 2 {
                    // Step 2: Storage Intro
                    div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center; text-align: center;",
                        div {
                            style: "width: 80px; height: 80px; border-radius: 50%; background-color: rgba(71, 145, 191, 0.15); display: flex; align-items: center; justify-content: center; margin-bottom: 36px;",
                            svg {
                                view_box: "0 0 24 24", width: "40", height: "40", stroke: "{theme.primary}", fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
                            }
                        }
                        h3 { style: "font-size: 22px; font-weight: 700; margin: 0 0 12px 0;", "Permissions setup" }
                        p { style: "font-size: 14px; line-height: 1.5; color: {theme.on_surface_muted}; margin: 0; padding: 0 12px;",
                            "To work properly, the app needs a few permissions. On the next screen, you’ll see what each permission is used for and you can decide which ones to allow or skip."
                        }
                    }
                } else if *step.read() == 3 {
                    // Step 3: Manage Storage Access
                    div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center; text-align: center;",
                        div {
                            style: "width: 80px; height: 80px; border-radius: 50%; background-color: rgba(71, 145, 191, 0.15); display: flex; align-items: center; justify-content: center; margin-bottom: 36px;",
                            svg {
                                view_box: "0 0 24 24", width: "40", height: "40", stroke: "{theme.primary}", fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" }
                            }
                        }
                        h3 { style: "font-size: 22px; font-weight: 700; margin: 0 0 12px 0;", "Manage storage access" }
                        p { style: "font-size: 14px; line-height: 1.5; color: {theme.on_surface_muted}; margin: 0; padding: 0 12px;",
                            "This permission lets the app change the download folder, detect duplicate downloads more accurately, and enable some extra features. It’s optional, but recommended for the best experience."
                        }
                    }
                } else if *step.read() == 4 {
                    // Step 4: Post Notification
                    div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center; text-align: center;",
                        div {
                            style: "width: 80px; height: 80px; border-radius: 50%; background-color: rgba(71, 145, 191, 0.15); display: flex; align-items: center; justify-content: center; margin-bottom: 36px;",
                            svg {
                                view_box: "0 0 24 24", width: "40", height: "40", stroke: "{theme.primary}", fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" }
                                path { d: "M13.73 21a2 2 0 0 1-3.46 0" }
                            }
                        }
                        h3 { style: "font-size: 22px; font-weight: 700; margin: 0 0 12px 0;", "Post Notification" }
                        p { style: "font-size: 14px; line-height: 1.5; color: {theme.on_surface_muted}; margin: 0; padding: 0 12px;",
                            "The app needs to run in the background to manage downloads. Notifications are used to keep you informed and allow background operation."
                        }
                    }
                } else if *step.read() == 5 {
                    // Step 5: You're all set / Let's Go
                    div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center; text-align: center;",
                        div {
                            style: "width: 80px; height: 80px; border-radius: 50%; background-color: rgba(56, 142, 60, 0.15); display: flex; align-items: center; justify-content: center; margin-bottom: 36px;",
                            svg {
                                view_box: "0 0 24 24", width: "40", height: "40", stroke: "#4CAF50", fill: "none", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                        h3 { style: "font-size: 22px; font-weight: 700; margin: 0 0 12px 0;", "You’re all set" }
                        p { style: "font-size: 14px; line-height: 1.5; color: {theme.on_surface_muted}; margin: 0; padding: 0 12px;",
                            "Everything is ready. All required permissions have been configured and the app is good to go."
                        }
                    }
                }
            }

            // Bottom Action Button(s)
            div {
                style: "display: flex; flex-direction: column; gap: 10px; width: 100%;",
                if *step.read() == 1 || *step.read() == 2 {
                    button {
                        onclick: move |_| {
                            let curr = *step.peek();
                            step.set(curr + 1);
                        },
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: {theme.primary}; color: {theme.on_primary};
                            font-size: 16px; font-weight: 600; cursor: pointer;
                        ",
                        "Next"
                    }
                } else if *step.read() == 3 {
                    button {
                        onclick: move |_| {
                            is_storage_granted.set(true);
                            step.set(4);
                        },
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: {theme.primary}; color: {theme.on_primary};
                            font-size: 16px; font-weight: 600; cursor: pointer;
                        ",
                        "Allow permission"
                    }
                    button {
                        onclick: move |_| step.set(4),
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: transparent; color: {theme.on_surface_muted};
                            font-size: 15px; font-weight: 600; cursor: pointer;
                        ",
                        "Skip"
                    }
                } else if *step.read() == 4 {
                    button {
                        onclick: move |_| {
                            is_notification_granted.set(true);
                            step.set(5);
                        },
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: {theme.primary}; color: {theme.on_primary};
                            font-size: 16px; font-weight: 600; cursor: pointer;
                        ",
                        "Allow permission"
                    }
                    button {
                        onclick: move |_| step.set(5),
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: transparent; color: {theme.on_surface_muted};
                            font-size: 15px; font-weight: 600; cursor: pointer;
                        ",
                        "Skip"
                    }
                } else if *step.read() == 5 {
                    button {
                        onclick: move |_| on_finish.call(()),
                        style: "
                            width: 100%; height: 50px; border-radius: 12px; border: none;
                            background-color: {theme.primary}; color: {theme.on_primary};
                            font-size: 16px; font-weight: 600; cursor: pointer;
                        ",
                        "Let's Go"
                    }
                }
            }
        }
    }
}
