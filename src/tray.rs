use tray_icon::{
    menu::{Menu, MenuId, MenuItem, PredefinedMenuItem},
    BadIcon, Icon, TrayIcon, TrayIconBuilder,
};

pub struct SystemTray {
    _icon: TrayIcon,
    pub open_id: MenuId,
    pub write_id: MenuId,
    pub folder_id: MenuId,
    pub exit_id: MenuId,
}

pub fn generate_tray_icon() -> Result<Icon, BadIcon> {
    let width = 32u32;
    let height = 32u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            // Draw notebook cover (dark grey with rounded corners)
            if x >= 4 && x <= 27 && y >= 3 && y <= 28 {
                let is_corner = (x <= 5 || x >= 26) && (y <= 4 || y >= 27);
                if !is_corner {
                    rgba[idx] = 45;
                    rgba[idx + 1] = 45;
                    rgba[idx + 2] = 50;
                    rgba[idx + 3] = 255;
                }
            }
            // Draw page #F5F5F5
            if x >= 8 && x <= 23 && y >= 6 && y <= 25 {
                rgba[idx] = 245;
                rgba[idx + 1] = 245;
                rgba[idx + 2] = 245;
                rgba[idx + 3] = 255;
            }
            // Draw notebook lines
            if (y == 10 && x >= 10 && x <= 21)
                || (y == 15 && x >= 10 && x <= 19)
                || (y == 20 && x >= 10 && x <= 20)
            {
                rgba[idx] = 45;
                rgba[idx + 1] = 45;
                rgba[idx + 2] = 50;
                rgba[idx + 3] = 255;
            }
        }
    }

    Icon::from_rgba(rgba, width, height)
}

impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let icon = generate_tray_icon()?;
        let menu = Menu::new();

        let open_item = MenuItem::new("Open Journal", true, None);
        let write_item = MenuItem::new("Write Current Block", true, None);
        let folder_item = MenuItem::new("Open Journal Folder", true, None);
        let separator = PredefinedMenuItem::separator();
        let exit_item = MenuItem::new("Exit", true, None);

        let open_id = open_item.id().clone();
        let write_id = write_item.id().clone();
        let folder_id = folder_item.id().clone();
        let exit_id = exit_item.id().clone();

        let _ = menu.append(&open_item);
        let _ = menu.append(&write_item);
        let _ = menu.append(&folder_item);
        let _ = menu.append(&separator);
        let _ = menu.append(&exit_item);

        let tray = TrayIconBuilder::new()
            .with_tooltip("Daily Journal")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .build()?;

        Ok(Self {
            _icon: tray,
            open_id,
            write_id,
            folder_id,
            exit_id,
        })
    }
}
