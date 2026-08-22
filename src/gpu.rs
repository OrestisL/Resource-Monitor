use std::fs;
use std::path::{Path, PathBuf};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;

#[derive(Debug, Clone, Copy, Default)]
pub struct GpuReadings {
    pub temp: f32,      // °C
    pub util: f32,      // %
    pub mem_used: f32,  // bytes
    pub mem_total: f32, // bytes
}

pub enum Gpu {
    Nvidia(Nvml),
    Amd(PathBuf),   // .../drm/cardN/device
    Intel(PathBuf), // best-effort (temp only)
    None,
}

impl Gpu {
    pub fn detect() -> Self {

        if let Ok(nvml) = Nvml::init() {
            return Gpu::Nvidia(nvml);
        }
        // scan DRM cards for an AMD or Intel GPU by PCI vendor id
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                // "card0", "card1"... but not "card0-HDMI-A-1" connector nodes.
                let is_card = name.strip_prefix("card").map_or(false, |rest| {
                    !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())
                });
                if !is_card {
                    continue;
                }
                let dev = entry.path().join("device");
                match read_string(&dev.join("vendor")).as_deref() {
                    Some("0x1002") => return Gpu::Amd(dev),   // AMD
                    Some("0x8086") => return Gpu::Intel(dev), // Intel
                    _ => {}
                }
            }
        }
        Gpu::None
    }

    pub fn read(&self) -> GpuReadings {
        match self {
            Gpu::Nvidia(nvml) => read_nvidia(nvml).unwrap_or_default(),
            Gpu::Amd(dir) => read_amd(dir),
            Gpu::Intel(dir) => read_intel(dir),
            Gpu::None => GpuReadings::default(),
        }
    }
}

fn read_nvidia(nvml: &Nvml) -> Option<GpuReadings> {
    let device = nvml.device_by_index(0).ok()?;
    let temp = device.temperature(TemperatureSensor::Gpu).ok()? as f32;
    let util = device.utilization_rates().ok()?.gpu as f32;
    let mem = device.memory_info().ok()?;
    Some(GpuReadings { temp, util, mem_used: mem.used as f32, mem_total: mem.total as f32 })
}

fn read_amd(dir: &Path) -> GpuReadings {
    GpuReadings {
        temp: read_hwmon_temp(dir).unwrap_or(0.0),
        util: read_f32(&dir.join("gpu_busy_percent")).unwrap_or(0.0),
        mem_used: read_f32(&dir.join("mem_info_vram_used")).unwrap_or(0.0),
        mem_total: read_f32(&dir.join("mem_info_vram_total")).unwrap_or(0.0),
    }
}

fn read_intel(dir: &Path) -> GpuReadings {
    // Temp works on discrete Arc via hwmon; integrated GPUs usually have none.
    // Util/VRAM are left at 0: Intel GPU-busy needs the i915/xe PMU (perf
    // counters + CAP_PERFMON), which is well beyond a sysfs read.
    GpuReadings {
        temp: read_hwmon_temp(dir).unwrap_or(0.0),
        ..Default::default()
    }
}

/// temp1_input (millidegrees C) from the first hwmon dir under `device/hwmon/`.
fn read_hwmon_temp(dir: &Path) -> Option<f32> {
    let sub = fs::read_dir(dir.join("hwmon")).ok()?.flatten().next()?.path();
    Some(read_f32(&sub.join("temp1_input"))? / 1000.0)
}

fn read_string(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_f32(path: &Path) -> Option<f32> {
    read_string(path)?.parse().ok()
}