use crate::config::{AppConfig, APP_NAME};
use crate::journal::{
    create_today_all, open_journal, open_journal_folder, today_str, JournalBlock,
};
use crate::sound::play_reminder_sound;
use crate::startup::set_startup;
use crate::tray::SystemTray;
use chrono::Local;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tray_icon::menu::MenuEvent;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent};
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SetForegroundWindow, ShowWindow, SW_HIDE, SW_RESTORE, SW_SHOW,
};

pub fn get_app_hwnd() -> HWND {
    let title: Vec<u16> = APP_NAME.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) }
}

pub fn show_main_window(ctx: &egui::Context) {
    let hwnd = get_app_hwnd();
    if hwnd != std::ptr::null_mut() {
        unsafe {
            ShowWindow(hwnd, SW_SHOW);
            ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd);
        }
    }
    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
}

pub fn hide_main_window(_ctx: &egui::Context) {
    let hwnd = get_app_hwnd();
    if hwnd != std::ptr::null_mut() {
        unsafe {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

pub struct JournalApp {
    config: AppConfig,
    tray: SystemTray,
    output_folder: Arc<Mutex<String>>,
    active_reminder: Option<JournalBlock>,
    info_message: Option<(String, Instant)>,
    should_exit: bool,
    last_boundary_check: Option<String>,
}

impl JournalApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();

        if config.start_with_windows {
            let _ = set_startup(true);
        }

        let tray = SystemTray::new().expect("Failed to initialize system tray");

        let open_id = tray.open_id.clone();
        let write_id = tray.write_id.clone();
        let folder_id = tray.folder_id.clone();
        let exit_id = tray.exit_id.clone();

        let output_folder = Arc::new(Mutex::new(config.output_folder.clone()));
        let folder_ref = output_folder.clone();

        let ctx1 = cc.egui_ctx.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            if event.id == open_id {
                show_main_window(&ctx1);
            } else if event.id == write_id {
                let folder = folder_ref.lock().unwrap().clone();
                let _ = open_journal(&folder, None, None);
            } else if event.id == folder_id {
                let folder = folder_ref.lock().unwrap().clone();
                let _ = open_journal_folder(&folder);
            } else if event.id == exit_id {
                std::process::exit(0);
            }
        }));

        let ctx2 = cc.egui_ctx.clone();
        TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
                | TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => {
                    show_main_window(&ctx2);
                }
                _ => {}
            }
        }));

        Self {
            config,
            tray,
            output_folder,
            active_reminder: None,
            info_message: None,
            should_exit: false,
            last_boundary_check: None,
        }
    }

    fn check_boundaries(&mut self, ctx: &egui::Context) {
        let now = Local::now();
        let minute_key = now.format("%Y-%m-%d %H:%M").to_string();

        if self.last_boundary_check.as_deref() == Some(&minute_key) {
            return;
        }
        self.last_boundary_check = Some(minute_key);

        let current_time_str = now.format("%H:%M").to_string();

        let target_block = if current_time_str == self.config.reminders.morning_time {
            Some(JournalBlock::Morning)
        } else if current_time_str == self.config.reminders.afternoon_time {
            Some(JournalBlock::Afternoon)
        } else if current_time_str == self.config.reminders.evening_time {
            Some(JournalBlock::Evening)
        } else {
            None
        };

        if let Some(block) = target_block {
            let key = block.key();
            let is_enabled = match block {
                JournalBlock::Morning => self.config.reminders.morning,
                JournalBlock::Afternoon => self.config.reminders.afternoon,
                JournalBlock::Evening => self.config.reminders.evening,
            };

            let today = today_str();
            let already_reminded = self
                .config
                .last_reminded
                .get(key)
                .map(|s| s == &today)
                .unwrap_or(false);

            if is_enabled && !already_reminded {
                self.config.last_reminded.insert(key.to_string(), today);
                self.config.save();

                play_reminder_sound();
                self.active_reminder = Some(block);

                show_main_window(ctx);
            }
        }
    }

    fn show_info_toast(&mut self, ui: &mut egui::Ui) {
        if let Some((msg, created)) = &self.info_message {
            if created.elapsed() < Duration::from_secs(4) {
                ui.colored_label(egui::Color32::LIGHT_BLUE, msg);
            }
        }
    }
}

impl eframe::App for JournalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_boundaries(ctx);

        if ctx.input(|i| i.viewport().close_requested()) {
            if !self.should_exit && self.config.minimize_to_tray {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                hide_main_window(ctx);
            }
        }

        // Custom card frame styling
        let card_frame = egui::Frame::none()
            .fill(egui::Color32::from_rgb(26, 29, 38))
            .rounding(10.0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 49, 64)))
            .inner_margin(egui::Margin::same(16.0));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);

            // Header Title & Badge
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new(APP_NAME)
                        .size(26.0)
                        .strong()
                        .color(egui::Color32::from_rgb(240, 243, 250)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("● Running in Tray")
                            .small()
                            .color(egui::Color32::from_rgb(90, 210, 140)),
                    );
                });
            });

            ui.label(
                egui::RichText::new("Three tiny check-ins. One clean Markdown dataset.")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 170)),
            );
            ui.add_space(16.0);

            // Today's Check-in Blocks Card
            card_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("📅 Today — {}", today_str()))
                        .strong()
                        .size(17.0)
                        .color(egui::Color32::from_rgb(230, 235, 245)),
                );
                ui.add_space(12.0);

                for block in JournalBlock::all() {
                    let (start, end) = block.times_formatted(&self.config.reminders);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(format!("{} Check-in", block.label()))
                                    .strong()
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(220, 225, 235)),
                            );
                            ui.label(
                                egui::RichText::new(format!("Time Window: {} – {}", start, end))
                                    .small()
                                    .color(egui::Color32::from_rgb(140, 150, 170)),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(egui::RichText::new("✏ Write Journal").strong())
                                .clicked()
                            {
                                if let Err(e) = open_journal(
                                    &self.config.output_folder,
                                    Some(block),
                                    Some(&self.config.reminders),
                                ) {
                                    self.info_message = Some((e, Instant::now()));
                                }
                            }
                        });
                    });
                    ui.add_space(10.0);
                }
            });

            ui.add_space(16.0);

            // Action Toolbar Buttons
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("📝 Create Today's Files").strong())
                    .clicked()
                {
                    if let Err(e) =
                        create_today_all(&self.config.output_folder, Some(&self.config.reminders))
                    {
                        self.info_message = Some((e, Instant::now()));
                    } else {
                        self.info_message =
                            Some(("Today's journal files are ready.".to_string(), Instant::now()));
                    }
                }
                ui.add_space(8.0);
                if ui
                    .button(egui::RichText::new("📂 Open Journal Folder").strong())
                    .clicked()
                {
                    if let Err(e) = open_journal_folder(&self.config.output_folder) {
                        self.info_message = Some((e, Instant::now()));
                    }
                }
            });

            ui.add_space(16.0);

            // Settings Card
            card_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("⚙ Reminders & Schedule Settings")
                        .strong()
                        .size(16.0)
                        .color(egui::Color32::from_rgb(230, 235, 245)),
                );
                ui.add_space(8.0);

                let mut startup = self.config.start_with_windows;
                if ui.checkbox(&mut startup, "Start automatically with Windows").changed() {
                    if let Err(e) = set_startup(startup) {
                        self.info_message = Some((e, Instant::now()));
                    } else {
                        self.config.start_with_windows = startup;
                        self.config.save();
                    }
                }

                ui.label(
                    egui::RichText::new(
                        "Closing the window keeps the app running in the system tray.",
                    )
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 170)),
                );
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.label(
                    egui::RichText::new("Configure Block Reminder Times (24h HH:MM):")
                        .strong()
                        .color(egui::Color32::from_rgb(220, 225, 235)),
                );
                ui.add_space(8.0);

                // Morning Time
                ui.horizontal(|ui| {
                    let mut m_rem = self.config.reminders.morning;
                    if ui.checkbox(&mut m_rem, "Morning reminder at:").changed() {
                        self.config.reminders.morning = m_rem;
                        self.config.save();
                    }
                    ui.add_space(6.0);
                    let mut m_time = self.config.reminders.morning_time.clone();
                    if ui
                        .add(egui::TextEdit::singleline(&mut m_time).desired_width(65.0))
                        .changed()
                    {
                        self.config.reminders.morning_time = m_time;
                        self.config.save();
                    }
                });

                ui.add_space(6.0);

                // Afternoon Time
                ui.horizontal(|ui| {
                    let mut a_rem = self.config.reminders.afternoon;
                    if ui.checkbox(&mut a_rem, "Afternoon reminder at:").changed() {
                        self.config.reminders.afternoon = a_rem;
                        self.config.save();
                    }
                    ui.add_space(6.0);
                    let mut a_time = self.config.reminders.afternoon_time.clone();
                    if ui
                        .add(egui::TextEdit::singleline(&mut a_time).desired_width(65.0))
                        .changed()
                    {
                        self.config.reminders.afternoon_time = a_time;
                        self.config.save();
                    }
                });

                ui.add_space(6.0);

                // Evening Time
                ui.horizontal(|ui| {
                    let mut e_rem = self.config.reminders.evening;
                    if ui.checkbox(&mut e_rem, "Evening reminder at:").changed() {
                        self.config.reminders.evening = e_rem;
                        self.config.save();
                    }
                    ui.add_space(6.0);
                    let mut e_time = self.config.reminders.evening_time.clone();
                    if ui
                        .add(egui::TextEdit::singleline(&mut e_time).desired_width(65.0))
                        .changed()
                    {
                        self.config.reminders.evening_time = e_time;
                        self.config.save();
                    }
                });
            });

            ui.add_space(16.0);

            // Journal folder Card
            card_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("📁 Journal Storage Folder")
                        .strong()
                        .size(16.0)
                        .color(egui::Color32::from_rgb(230, 235, 245)),
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let mut folder_str = self.config.output_folder.clone();
                    ui.add(
                        egui::TextEdit::singleline(&mut folder_str)
                            .interactive(false)
                            .desired_width(380.0),
                    );

                    if ui.button("Browse...").clicked() {
                        let res = rfd::FileDialog::new()
                            .set_directory(&self.config.output_folder)
                            .pick_folder();
                        if let Some(folder) = res {
                            let path_str = folder.to_string_lossy().to_string();
                            self.config.output_folder = path_str.clone();
                            if let Ok(mut lock) = self.output_folder.lock() {
                                *lock = path_str;
                            }
                            self.config.save();
                        }
                    }
                });
            });

            ui.add_space(10.0);
            self.show_info_toast(ui);
        });

        // Popup Modal Window for Reminders
        if let Some(block) = self.active_reminder {
            let mut open = true;
            let (start, end) = block.times_formatted(&self.config.reminders);

            egui::Window::new("Journal Reminder")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.heading(format!("{} block is finished", block.label()));
                    ui.add_space(6.0);
                    ui.label(format!("Take a minute to record {}–{}.", start, end));
                    ui.add_space(18.0);

                    ui.horizontal(|ui| {
                        if ui.button("Write Journal").clicked() {
                            let output_folder = self.config.output_folder.clone();
                            let _ = open_journal(
                                &output_folder,
                                Some(block),
                                Some(&self.config.reminders),
                            );
                            self.active_reminder = None;
                            hide_main_window(ctx);
                        }
                        if ui.button("Later").clicked() {
                            self.active_reminder = None;
                            hide_main_window(ctx);
                        }
                    });
                });

            if !open {
                self.active_reminder = None;
                hide_main_window(ctx);
            }
        }

        ctx.request_repaint_after(Duration::from_secs(5));
    }
}
