mod metrics;
mod render;
mod tray;
mod gpu;
mod config;

use crate::metrics::Metrics;
use ksni::blocking::TrayMethods;

use crate::tray::ResourceMonitor;
use crate::config::config;
/// How often to refresh, in seconds.

fn main() {
    let mut metrics = Metrics::new();
    let interval = config().interval_secs;

    //let tray = ResourceMonitor { readings: metrics.read() };
    let handle = {
        let mut attempts = 0;
        loop {
            let tray = ResourceMonitor { readings: metrics.read() };
            match tray.spawn() {
                Ok(h) => {
                    break h;
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= 15 {
                        panic!("tray host never appeared after {attempts} tries: {e}");
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
    };
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let readings = metrics.read();
        let _ = handle.update(move |t: &mut ResourceMonitor| {
            t.readings = readings;
        });
    }
}
