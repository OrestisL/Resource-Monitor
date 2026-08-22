// tray item

use ksni::menu::StandardItem;
use ksni::{ Icon, MenuItem, Tray };

use crate::metrics::Readings;
use crate::render::render_icon;

pub struct ResourceMonitor {
    pub readings: Readings,
}

impl Tray for ResourceMonitor {
    const MENU_ON_ACTIVATE: bool = true;

    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        "CPU/GPU Temps".into()
    }

    // icon
    fn icon_pixmap(&self) -> Vec<Icon> {
        let text = format!("{}", self.readings.cpu_temp.round() as i32);
        let color = temp_color(self.readings.cpu_temp);
        vec![render_icon(&text, color)]
    }

    // show everything when clicking the icon
    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            (StandardItem {
                label: format!("CPU:  {:.0} \u{00b0}C", self.readings.cpu_temp),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!("CPU Utilization:  {:.0}%", self.readings.cpu_util),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!("Available RAM:  {:.2}GB", self.readings.ram_avail / 1000000000.0),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!("Total RAM:  {:.2}GB", self.readings.ram_total / 1000000000.0),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!(
                    "RAM usage:  {:.2}%",
                    (1.0 - self.readings.ram_avail / self.readings.ram_total) * 100.0
                ),
                enabled: false,
                ..Default::default()
            }).into(),
            MenuItem::Separator,
            (StandardItem {
                label: format!("GPU:  {:.0} \u{00b0}C", self.readings.gpu_temp),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!("GPU Utilization:  {:.0}%", self.readings.gpu_util),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!(
                    "Available GPU Memory:  {:.2}GB",
                    self.readings.gpu_mem_avail / 1000000000.0
                ),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!(
                    "Total GPU Memory: {:.2}GB",
                    self.readings.gpu_mem_total / 1000000000.0
                ),
                enabled: false,
                ..Default::default()
            }).into(),
            (StandardItem {
                label: format!(
                    "GPU Memory Usage: {:.2}%",
                    (1.0 - self.readings.gpu_mem_avail / self.readings.gpu_mem_total) * 100.0
                ),
                enabled: false,
                ..Default::default()
            }).into(),
            MenuItem::Separator,
            (StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_this: &mut Self| std::process::exit(0)),
                ..Default::default()
            }).into()
        ]
    }
}

/// icon colors depending on temps
fn temp_color(temp: f32) -> (u8, u8, u8) {
    if temp >= 80.0 {
        (220, 60, 60) // hot  -> red
    } else if temp >= 65.0 {
        (225, 180, 50) // warm -> amber
    } else {
        (90, 200, 130) // cool -> green
    }
}
