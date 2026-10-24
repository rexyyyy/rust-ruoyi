use rust_xlsxwriter::{Color, Format, Workbook};
use calamine::{Data, Reader, Xlsx};
use actix_web::HttpResponse;
use futures_util::future::BoxFuture;

/// 生成 xlsx 字节（表头 + 数据行）
pub fn build_xlsx(sheet_name: &str, headers: &[&str], rows: &[Vec<String>]) -> Result<Vec<u8>, String> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name(sheet_name).map_err(|e| e.to_string())?;
    let header_format = Format::new().set_bold().set_background_color(Color::Silver);
    for (i, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, i as u16, *h, &header_format).map_err(|e| e.to_string())?;
    }
    for (r, row) in rows.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            sheet.write_string((r + 1) as u32, c as u16, v).map_err(|e| e.to_string())?;
        }
    }
    workbook.save_to_buffer().map_err(|e| e.to_string())
}

/// 分批导出：fetch_page(page_no)（从 1 开始）逐页拉取数据并即写即弃，
/// 避免一次查询全量数据 + 全量行字符串同时驻留内存。
/// 某页返回行数 < batch_size 视为最后一页。
pub async fn build_xlsx_streaming<F>(
    sheet_name: &str,
    headers: &[&str],
    batch_size: u64,
    mut fetch_page: F,
) -> Result<Vec<u8>, String>
where
    F: FnMut(u64) -> BoxFuture<'static, Result<Vec<Vec<String>>, String>>,
{
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name(sheet_name).map_err(|e| e.to_string())?;
    let header_format = Format::new().set_bold().set_background_color(Color::Silver);
    for (i, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, i as u16, *h, &header_format).map_err(|e| e.to_string())?;
    }
    let mut written: u32 = 0;
    let mut page_no: u64 = 1;
    loop {
        let rows = fetch_page(page_no).await?;
        let n = rows.len();
        for row in rows {
            written += 1;
            for (c, v) in row.iter().enumerate() {
                sheet.write_string(written, c as u16, v).map_err(|e| e.to_string())?;
            }
        }
        if (n as u64) < batch_size {
            break;
        }
        page_no += 1;
    }
    workbook.save_to_buffer().map_err(|e| e.to_string())
}

/// 解析 xlsx 为二维字符串表
pub fn parse_xlsx(data: &[u8]) -> Result<Vec<Vec<String>>, String> {
    let mut wb = Xlsx::new(std::io::Cursor::new(data.to_vec())).map_err(|e| format!("xlsx open: {}", e))?;
    let sheets = wb.sheet_names().to_vec();
    let name = sheets.first().ok_or("xlsx has no sheets")?.clone();
    let range = wb.worksheet_range(&name).map_err(|e| format!("sheet range: {}", e))?;
    let mut out = Vec::new();
    for row in range.rows() {
        let mut cells = Vec::new();
        for c in row {
            cells.push(match c {
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                Data::Bool(b) => b.to_string(),
                _ => String::new(),
            });
        }
        out.push(cells);
    }
    Ok(out)
}

/// 构造 Excel 下载响应
pub fn excel_response(bytes: Vec<u8>, filename: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .insert_header(("Content-Disposition", format!("attachment; filename={}", filename)))
        .body(bytes)
}
