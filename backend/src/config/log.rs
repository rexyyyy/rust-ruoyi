use crate::modules::system::service::CONTEXT;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{DateType, KeepType, Packer, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;

pub fn init_log() {
    //create log dir
    let _ = std::fs::create_dir_all(&CONTEXT.config.log_dir);
    let path =&CONTEXT.config.log_dir;
    //init fast log
    let mut cfg = Config::new()
    .file_split(path, 
          Rolling::new(RollingType::ByDate(DateType::Day)), 
          KeepType::KeepNum(2), 
          LogPacker {})
        .level(str_to_log_level(&CONTEXT.config.log_level))
       ;
    if CONTEXT.config.debug {
        cfg = cfg.console();
    }
    cfg = cfg.chan_len(CONTEXT.config.log_chan_len);
    let _ = fast_log::init(cfg);
    if CONTEXT.config.debug == false {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
    log::info!("log init with server start..");
}
#[allow(dead_code)]
fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        // "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        // "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        // "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
        _ => Box::new(fast_log::plugin::packer::LogPacker {}),
    }
}
#[allow(dead_code)]
fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}


fn str_to_log_level(arg: &str) -> log::LevelFilter {
    return match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };
}