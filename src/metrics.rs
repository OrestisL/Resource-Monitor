use sysinfo::{ Components, System };

use crate::gpu::Gpu;

#[derive(Debug, Clone, Copy, Default)]
pub struct Readings {
    pub cpu_temp: f32,
    pub cpu_util: f32,
    pub ram_avail: f32,
    pub ram_total: f32,
    pub gpu_temp: f32,
    pub gpu_util: f32,
    pub gpu_mem_avail: f32,
    pub gpu_mem_total: f32,
}

pub struct Metrics {
    components: Components,
    system: System,
    gpu: Gpu,
}

impl Metrics {
     pub fn new() -> Self {
        Metrics {
            components: Components::new_with_refreshed_list(),
            system: System::new(),
            gpu: Gpu::detect(),
        }
    }

    pub fn read(&mut self) -> Readings {
        self.components.refresh(false);
        let (cpu_temp, cpu_util, ram_avail, ram_total) =
            self.read_cpu_ram().unwrap_or((0.0, 0.0, 0.0, 0.0));

        let g = self.gpu.read();
        let gpu_mem_avail = (g.mem_total - g.mem_used).max(0.0); // your menu wants "available"

        Readings {
            cpu_temp, cpu_util, ram_avail, ram_total,
            gpu_temp: g.temp,
            gpu_util: g.util,
            gpu_mem_avail,
            gpu_mem_total: g.mem_total,
        }
    }

    fn read_cpu_ram(&mut self) -> Option<(f32, f32, f32, f32)> {
        let cpu_temp = self.components
            .list()
            .iter()
            .find(|c| {
                let l = c.label();
                l.contains("Package id") || l.contains("Tctl")
            })
            .and_then(|c| c.temperature()) // -> Option<f32>, now types line up
            .or_else(|| {
                self.components
                    .list()
                    .iter()
                    .filter(|c| c.label().starts_with("Core"))
                    .filter_map(|c| c.temperature())
                    .fold(None, |acc, t| Some(acc.map_or(t, |a: f32| a.max(t))))
            })
            .unwrap_or(0.0);

        self.system.refresh_cpu_usage();
        let cpu_util = self.system.global_cpu_usage();

        self.system.refresh_memory();
        let ram_free = self.system.available_memory();
        let ram_total = self.system.total_memory();

        Some((cpu_temp as f32, cpu_util, ram_free as f32, ram_total as f32))
    }
}
