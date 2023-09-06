mod config;
mod slider;
mod wallpaper;

use std::time::Duration;
use std::thread::sleep;

use anyhow::{Result, Context};
use clap::Parser;
use mimalloc::MiMalloc;

use crate::config::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const OUTPUT_NAME: &str = "satpaper_latest.png";
const SLEEP_DURATION: Duration = Duration::from_secs(60);

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    
    update_wallpaper()
        .context("An error occurred in the wallpaper updating loop")?;

    Ok(())
}

fn update_wallpaper() -> Result<()> {
    let config = Config::parse();
    
    let mut timestamp = None;
    
    loop  {
        log::info!("Checking timestamp...");

        let new = slider::fetch_latest_timestamp(&config)
            .unwrap_or_else(|err| {
                log::error!("Failed to fetch latest timestamp: {err}");
                log::error!("Check aborted; waiting until next go round.");
                timestamp.unwrap_or(0)
            });

        if timestamp
            .map(|old| old != new)
            .unwrap_or(true) 
        {
            log::info!("Timestamp has changed!");
            log::debug!("Old timestamp: {timestamp:?}, new timestamp: {new}");
            log::info!("Fetching updated source and compositing new wallpaper...");
            
            if slider::composite_latest_image(&config)? {
                timestamp = Some(new);

                if config.once {
                    return  Ok(());
                }

                wallpaper::set(
                    config.target_path.join(OUTPUT_NAME),
                    config.wallpaper_command.as_deref(),
                )?;

                log::info!("New wallpaper composited and set.");
            }
        }
        
        // Safety: as far as I can tell, this function doesn't have any safety
        // preconditions?
        //
        // Even the official C documentation doesn't document any invariants etc -
        // it only mentions that it's intended for specific niche cases (which we happen to be one of!)
        unsafe {
            // Aggressively return as much memory to the operating system as possible.
            //
            // (Yes, this is necessary.)
            libmimalloc_sys::mi_collect(true);
        }

        log::debug!("Sleeping for {SLEEP_DURATION:?}...");

        sleep(SLEEP_DURATION)
    }

    #[allow(unreachable_code)]
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_wallpaper() -> Result<()> {
        let config = Config {
            satellite: Satellite::GOESEast,
            resolution_x: 2556,
            resolution_y: 1440,
            disk_size: 95,
            target_path: ".".into(),
            wallpaper_command: None,
            once: false
        };

        slider::composite_latest_image(&config)?;

        std::fs::remove_file("./satpaper_latest.png")?;

        Ok(())
    }
}