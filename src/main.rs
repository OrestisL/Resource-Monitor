mod metrics;
mod render;
mod tray;
mod gpu;
mod config;

use crate::metrics::Metrics;
use ksni::blocking::TrayMethods;

use crate::tray::ResourceMonitor;
use crate::config::config;

// ensure single instance 
use single_instance::SingleInstance;

fn main() {

    // create single instance
    let instance = SingleInstance::new("resource-monitor")
           .expect("failed to init single-instance guard");
    // if an instance exists, exit
    if !instance.is_single()
    {
        eprintln!("another instance is already running; exiting");
        return;
    }
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
