//! 服务器监控信息采集（对标 RuoYi `Server` 接口）。
//!
//! 采集本身是同步阻塞操作（sysinfo 要读 /proc 或 Windows API），调用方
//! （`server_controller`）已把它放进 actix 阻塞线程池执行。这里再做两件事：
//!
//! 1. **结果缓存 5 秒**：前端监控页会定时轮询 `/monitor/server`，每次全量采集
//!    （CPU + 内存 + 全部磁盘 + 主机信息）在同一进程里没有意义，缓存后把实际采集
//!    频率限制在 1/5s。
//! 2. **复用同一个 `System` 实例**：sysinfo 的 CPU 使用率靠「两次采样求差」计算，
//!    每次都 `System::new_all()` 会让差值恒为 0（永远显示 0%）；复用实例才能反映真实负载。

use std::sync::Mutex;
use std::time::{Duration, Instant};

use bytesize::ByteSize;
use once_cell::sync::Lazy;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

use crate::modules::system::domain::vo::{CpuVO, DiskVO, Memory, ServerVO, SysVO};

/// 采集结果缓存时长：监控页轮询间隔通常 ≥5s，缓存不会让用户感觉「数据不动」。
const CACHE_TTL: Duration = Duration::from_secs(5);

/// 长生命周期 `System` 实例（CPU 使用率需要相邻两次采样的差值）。
static SYSINFO: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));
/// 最近一次采集结果（时间戳 + 数据）。
static CACHE: Lazy<Mutex<Option<(Instant, ServerVO)>>> = Lazy::new(|| Mutex::new(None));

/// 取服务器监控信息：5 秒内的重复调用直接复用上次结果。
pub fn get_server_info() -> ServerVO {
    if let Ok(guard) = CACHE.lock() {
        if let Some((at, vo)) = guard.as_ref() {
            if at.elapsed() < CACHE_TTL {
                return vo.clone();
            }
        }
    }
    let vo = collect();
    if let Ok(mut guard) = CACHE.lock() {
        *guard = Some((Instant::now(), vo.clone()));
    }
    vo
}

/// 真正采集一次（CPU / 内存 / 磁盘 / 主机信息）。
fn collect() -> ServerVO {
    let mut sys = match SYSINFO.lock() {
        Ok(g) => g,
        // 锁中毒时退化为「本次新建实例」，保证监控接口不 500
        Err(_) => return collect_with(System::new_all()),
    };
    // 只刷新需要的部分（refresh_all 还会拉全量进程列表，代价大且这里用不到）
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_disks_list();
    sys.refresh_disks();
    collect_with_ref(&mut sys)
}

/// 新建实例的兜底路径（锁中毒时用）。
fn collect_with(mut sys: System) -> ServerVO {
    collect_with_ref(&mut sys)
}

fn collect_with_ref(sys: &mut System) -> ServerVO {
    // ---------- 磁盘 ----------
    let mut sys_files = Vec::with_capacity(sys.disks().len());
    for d in sys.disks() {
        let total = d.total_space();
        let available = d.available_space();
        let used = total.saturating_sub(available);
        let percent = |part: u64, whole: u64| -> f64 {
            if whole == 0 {
                0.0
            } else {
                part as f64 * 100.0 / whole as f64
            }
        };
        sys_files.push(DiskVO {
            dir_name: d.name().to_string_lossy().to_string(),
            free: ByteSize::b(available).to_string(),
            sys_type_name: String::from_utf8_lossy(d.file_system()).to_string(),
            type_name: d.mount_point().to_string_lossy().to_string(),
            total: ByteSize::b(total).to_string(),
            used: ByteSize::b(used).to_string(),
            usage: format!("{:.1}", percent(used, total)),
        });
    }

    // ---------- 内存 ----------
    // 注意字段语义：free = 空闲、used = 已用（历史实现里两者写反了，前端看到的是颠倒的值）。
    // trim_end_matches(" GB") 是为了兼容前端既有展示逻辑，暂不改变输出格式。
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory().min(total_mem);
    let free_mem = total_mem.saturating_sub(used_mem);
    let mem = Memory {
        free: ByteSize::b(free_mem).to_string().trim_end_matches(" GB").to_string(),
        total: ByteSize::b(total_mem).to_string().trim_end_matches(" GB").to_string(),
        used: ByteSize::b(used_mem).to_string().trim_end_matches(" GB").to_string(),
        usage: if total_mem == 0 {
            "0.0".to_string()
        } else {
            format!("{:.1}", used_mem as f64 * 100.0 / total_mem as f64)
        },
    };

    // ---------- CPU ----------
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let cpu = CpuVO {
        cpu_num: sys.physical_core_count().unwrap_or_else(|| sys.cpus().len()) as u8,
        free: (100.0 - cpu_usage).clamp(0.0, 100.0),
        sys: 0.0,
        total: 0,
        used: cpu_usage,
        wait: 0.0,
    };

    // ---------- 主机 ----------
    let sys_vo = SysVO {
        computer_ip: String::new(),
        computer_name: sys.host_name().unwrap_or_default(),
        os_arch: std::env::consts::ARCH.to_string(),
        os_name: format!(
            "{} {}",
            sys.name().unwrap_or_default(),
            sys.os_version().unwrap_or_default()
        )
        .trim()
        .to_string(),
    };

    ServerVO {
        cpu,
        mem,
        sys: sys_vo,
        sys_files,
    }
}
