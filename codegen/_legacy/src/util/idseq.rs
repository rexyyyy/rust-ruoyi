/// 雪花算法分布式唯一ID生成器
/// 参考  https://blog.csdn.net/wanghao112956/article/details/102794497
/// 目的 通过调整SEQUENCE_ID_BITS 
///      生成的id可以调整长度
/// 
/// 原生SnowFlake中的空间划分:
/// 1, 高位1bit固定0表示正数
/// 2, 41bit毫秒时间戳
/// 3, 10bit机器编号, 最高可支持1024个节点
/// 4, 12bit自增序列, 单节点最高可支持4096个ID/ms
/// 改为
/// 缩短算法后空间划分
/// 1, 高位32bit作为秒级时间戳, 时间戳减去固定值(2019年时间戳), 最高可支持到2106年(1970 + 2^32 / 3600 / 24 / 365 ≈ 2106)
/// 2, 5bit作为机器标识, 最高可部署32台机器                   下面代码是3  就是只支持到8个机器标识
/// 3, 最后16bit作为自增序列, 单节点最高每秒 2^16 = 65536个ID  下面是14 只到16384个每秒
/// PS: 如果需要部署更多节点, 可以适当调整机器位和自增序列的位长, 如机器位7bit, 自增序列14bit, 这样一来就变成了支持2^7=128个节点, 单节点每秒2^14=16384个自增序列

use chrono::prelude::*;
use std::sync::atomic::{AtomicU64,Ordering};
//** 初始偏移时间戳 */
static OFFSET: u64 = 1546300800;

/** 机器id (0~15 保留 16~31作为备份机器) */
static WORKER_ID: u64 = 1;
/** 机器id所占位数 (5bit, 支持最大机器数 2的3次方 8 只有最大8个*/
static WORKER_ID_BITS: u64 = 3;
/** 自增序列所占位数 (16bit, 2的16次方 65536 14次方是 16384*/
static SEQUENCE_ID_BITS: u64 = 14;
/** 机器id偏移位数 */
static WORKER_SHIFT_BITS: u64 = SEQUENCE_ID_BITS;
/** 自增序列偏移位数 */
static OFFSET_SHIFT_BITS: u64 = SEQUENCE_ID_BITS + WORKER_ID_BITS;
/** 机器标识最大值 (2^5 / 2 - 1 = 15) */
static WORKER_ID_MAX: u64 = ((1 << WORKER_ID_BITS) - 1) >> 1;
/** 备份机器ID开始位置 (2^5 / 2 = 16) */
static BACK_WORKER_ID_BEGIN: u64 = (1 << WORKER_ID_BITS) >> 1;
/** 自增序列最大值 (2^16  */
static SEQUENCE_MAX: u64 = (1 << SEQUENCE_ID_BITS) - 1;
/** 发生时间回拨时容忍的最大回拨时间 (秒) */
static BACK_TIME_MAX: u64 = 1;

/** 上次生成ID的时间戳 (秒) */
static LAST_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
/** 当前秒内序列 (2^16)*/
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
/** 备份机器上次生成ID的时间戳 (秒) */
static LAST_TIMESTAMP_BAK: AtomicU64 = AtomicU64::new(0);
/** 备份机器当前秒内序列 (2^16)*/
static SEQUENCE_BAK: AtomicU64 = AtomicU64::new(0);

pub struct IdSeq {}

impl IdSeq {
    pub fn next_id() -> u64 {
        //获取当前毫秒值
        let now: DateTime<Local> = Local::now();
        let mills: u64 = now.timestamp_millis().try_into().unwrap();
        //println!("获取毫秒的值: {}", mills);
        return Self::next_seid(mills / 1000);
    }

    /**
     * 主机器自增序列
     * @param timestamp 当前Unix时间戳
     * @return long
     */
    pub fn next_seid(timestamp: u64) -> u64 {
        // 时钟回拨检查
        if (timestamp < LAST_TIMESTAMP.load(Ordering::Relaxed)) {
            // 发生时钟回拨
            println!(
                "时钟回拨, 启用备份机器ID: now: [{}] last: [{}]",
                timestamp, LAST_TIMESTAMP.load(Ordering::Relaxed)
            );
            return Self::next_id_backup(timestamp);
        }

        // 开始下一秒
        if (timestamp != LAST_TIMESTAMP.load(Ordering::Relaxed)) {
            let swapped_value = LAST_TIMESTAMP.swap(timestamp, Ordering::Relaxed);
            // = timestamp;
            let swapped_sequence = SEQUENCE.swap(0, Ordering::Relaxed);
            // sequence = 0;
        }
        SEQUENCE.fetch_add(1, Ordering::Relaxed);
        if (0 == (SEQUENCE.load(Ordering::Relaxed) & SEQUENCE_MAX)) {
            // 秒内序列用尽
            //            log.warn("秒内[{}]序列用尽, 启用备份机器ID序列", timestamp);
            SEQUENCE.fetch_sub(1, Ordering::Relaxed);
            return Self::next_id_backup(timestamp);
        }
        println!("timestamp: {} OFFSET:{}",&timestamp,&OFFSET);

        ((timestamp - OFFSET) << OFFSET_SHIFT_BITS) | (WORKER_ID << WORKER_SHIFT_BITS) | SEQUENCE.load(Ordering::Relaxed)
    }

    /**
     * 备份机器自增序列
     * @param timestamp timestamp 当前Unix时间戳
     * @return long
     */
    pub fn next_id_backup(mut timestamp: u64) -> u64 {
        if (timestamp < LAST_TIMESTAMP_BAK.load(Ordering::Relaxed)) {
            //获取当前毫秒值
            let now: DateTime<Local> = Local::now();
            let mills: u64 = now.timestamp_millis().try_into().unwrap();

            if ((LAST_TIMESTAMP_BAK.load(Ordering::Relaxed) - mills / 1000) <= BACK_TIME_MAX) {
                timestamp = LAST_TIMESTAMP_BAK.load(Ordering::Relaxed);
            } else {
                // throw new RuntimeException(String.format("时钟回拨: now: [%d] last: [%d]", timestamp, lastTimestampBak));
            }
        }

        if (timestamp != LAST_TIMESTAMP_BAK.load(Ordering::Relaxed)) {
            let swapped_value = LAST_TIMESTAMP_BAK.swap(timestamp, Ordering::Relaxed);
            // = timestamp;
            let swapped_sequence = SEQUENCE_BAK.swap(0, Ordering::Relaxed);
            // sequence = 0;

            // lastTimestampBak = timestamp;
            // sequenceBak = 0;
        }
        SEQUENCE_BAK.fetch_add(1, Ordering::Relaxed);
        // sequenceBak = sequenceBak + 1;
        if (0 == (SEQUENCE_BAK.load(Ordering::Relaxed) & SEQUENCE_MAX)) {
            // 秒内序列用尽
            //            logger.warn("秒内[{}]序列用尽, 备份机器ID借取下一秒序列", timestamp);
            return Self::next_id_backup(timestamp + 1);
        }

        ((timestamp - OFFSET) << OFFSET_SHIFT_BITS)
            | ((WORKER_ID ^ BACK_WORKER_ID_BEGIN) << WORKER_SHIFT_BITS)
            | SEQUENCE_BAK.load(Ordering::Relaxed)
    }
}
