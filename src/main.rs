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

    let tray = ResourceMonitor { readings: metrics.read() };
    let handle = tray
        .spawn()
        .expect("failed to start tray - is the AppIndicator extension enabled?");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let readings = metrics.read();
        let _ = handle.update(move |t: &mut ResourceMonitor| t.readings = readings);
    }

}