//! 极小 ZIP 打包器（STORE 方式，不压缩）。
//!
//! 只为 `/tool/gen/batchGenCode` 下载代码包服务。
//! 没引入 `zip` crate 的原因见 Cargo.toml 注释：它 2.x / 8.x 的 API 差异过大，
//! 而这里只需要几十行就能把「文件列表 -> zip 字节」做完。
//! STORE 方式浏览器与压缩软件都能正常解压，体积略大但对几十 KB 的源码包无所谓。

/// CRC32 (IEEE 802.3)，ZIP 头里必填。
fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for b in data {
        crc ^= *b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

struct Entry<'a> {
    name: &'a str,
    data: &'a [u8],
    crc: u32,
    offset: u32,
}

fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

/// 把 `路径 -> 内容` 打成 zip 字节流。路径统一用 `/` 分隔。
pub fn zip_files<'a>(files: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut entries: Vec<Entry> = Vec::new();

    for (name, content) in files {
        let data = content.as_bytes();
        let crc = crc32(data);
        entries.push(Entry {
            name,
            data,
            crc,
            offset: out.len() as u32,
        });

        // ---- local file header ----
        put_u32(&mut out, 0x0403_4b50);
        put_u16(&mut out, 20); // version needed
        put_u16(&mut out, 0x0800); // flag: UTF-8 names
        put_u16(&mut out, 0); // method: STORE
        put_u16(&mut out, 0); // mod time
        put_u16(&mut out, 0); // mod date
        put_u32(&mut out, crc);
        put_u32(&mut out, data.len() as u32);
        put_u32(&mut out, data.len() as u32);
        put_u16(&mut out, name.len() as u16);
        put_u16(&mut out, 0); // extra len
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
    }

    // ---- central directory ----
    let cd_start = out.len();
    for e in &entries {
        put_u32(&mut out, 0x0201_4b50);
        put_u16(&mut out, 20); // version made by
        put_u16(&mut out, 20); // version needed
        put_u16(&mut out, 0x0800);
        put_u16(&mut out, 0); // method
        put_u16(&mut out, 0);
        put_u16(&mut out, 0);
        put_u32(&mut out, e.crc);
        put_u32(&mut out, e.data.len() as u32);
        put_u32(&mut out, e.data.len() as u32);
        put_u16(&mut out, e.name.len() as u16);
        put_u16(&mut out, 0); // extra
        put_u16(&mut out, 0); // comment
        put_u16(&mut out, 0); // disk number
        put_u16(&mut out, 0); // internal attrs
        put_u32(&mut out, 0); // external attrs
        put_u32(&mut out, e.offset);
        out.extend_from_slice(e.name.as_bytes());
    }
    let cd_len = out.len() - cd_start;

    // ---- end of central directory ----
    put_u32(&mut out, 0x0605_4b50);
    put_u16(&mut out, 0);
    put_u16(&mut out, 0);
    put_u16(&mut out, entries.len() as u16);
    put_u16(&mut out, entries.len() as u16);
    put_u32(&mut out, cd_len as u32);
    put_u32(&mut out, cd_start as u32);
    put_u16(&mut out, 0); // comment len

    out
}
