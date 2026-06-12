use chrono::Utc;
use domain::entities::Worklog;
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet, XlsxError};

use crate::error::{UseCaseError, UseCaseResult};
use crate::export::styles::{tag_color_index, ExportStyles};
use crate::export::worklog_display::{
    format_description, format_duration_secs, jalali_date_string,
};

const HEADERS: [&str; 5] = ["ID", "Date", "Duration", "Description", "Tags"];
const TITLE_ROW: u32 = 0;
const HEADER_ROW: u32 = 1;
const FIRST_DATA_ROW: u32 = 2;

const COL_ID: u16 = 0;
const COL_DATE: u16 = 1;
const COL_DURATION: u16 = 2;
const COL_DESCRIPTION: u16 = 3;
const COL_TAGS: u16 = 4;

const ID_COLOR: Color = Color::RGB(0x78_7C_84);
const DATE_COLOR: Color = Color::RGB(0x2E_7D_52);
const DURATION_COLOR: Color = Color::RGB(0x2B_5C_9A);

pub fn worklogs_to_xlsx(worklogs: &[Worklog]) -> UseCaseResult<Vec<u8>> {
    let mut workbook = Workbook::new();
    let styles = ExportStyles::new();
    let worksheet = workbook.add_worksheet();

    configure_sheet(worksheet, worklogs.len())?;
    write_title_row(worksheet, worklogs.len(), &styles)?;
    write_headers(worksheet, &styles)?;
    write_rows(worksheet, worklogs, &styles)?;

    if !worklogs.is_empty() {
        let last_row = FIRST_DATA_ROW + worklogs.len() as u32 - 1;
        xlsx(worksheet.autofilter(HEADER_ROW, COL_ID, last_row, COL_TAGS))?;
    }

    workbook
        .save_to_buffer()
        .map_err(|e| UseCaseError::Export(e.to_string()))
}

fn xlsx<T>(result: Result<T, XlsxError>) -> UseCaseResult<T> {
    result.map_err(|e| UseCaseError::Export(e.to_string()))
}

fn configure_sheet(worksheet: &mut Worksheet, row_count: usize) -> UseCaseResult<()> {
    xlsx(worksheet.set_name("Worklogs"))?;
    worksheet.set_tab_color(Color::RGB(0x58_C4_8C));

    xlsx(worksheet.set_column_width(COL_ID, 38.0))?;
    xlsx(worksheet.set_column_width(COL_DATE, 14.0))?;
    xlsx(worksheet.set_column_width(COL_DURATION, 12.0))?;
    xlsx(worksheet.set_column_width(COL_DESCRIPTION, 48.0))?;
    xlsx(worksheet.set_column_width(COL_TAGS, 32.0))?;

    xlsx(worksheet.set_row_height(TITLE_ROW, 22.0))?;
    xlsx(worksheet.set_row_height(HEADER_ROW, 20.0))?;

    if row_count > 0 {
        xlsx(worksheet.set_freeze_panes(FIRST_DATA_ROW, COL_ID))?;
    }

    Ok(())
}

fn write_title_row(
    worksheet: &mut Worksheet,
    row_count: usize,
    styles: &ExportStyles,
) -> UseCaseResult<()> {
    let exported_at = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let title = format!("Worklogger — {row_count} worklog(s) · exported {exported_at}");

    xlsx(worksheet.merge_range(
        TITLE_ROW,
        COL_ID,
        TITLE_ROW,
        COL_TAGS,
        &title,
        &styles.title,
    ))?;

    Ok(())
}

fn write_headers(worksheet: &mut Worksheet, styles: &ExportStyles) -> UseCaseResult<()> {
    for (col, header) in HEADERS.iter().enumerate() {
        xlsx(worksheet.write_string_with_format(
            HEADER_ROW,
            col as u16,
            *header,
            &styles.header,
        ))?;
    }
    Ok(())
}

fn write_rows(
    worksheet: &mut Worksheet,
    worklogs: &[Worklog],
    styles: &ExportStyles,
) -> UseCaseResult<()> {
    for (row_idx, worklog) in worklogs.iter().enumerate() {
        let row = FIRST_DATA_ROW + row_idx as u32;
        let row_format = styles.row_format(row_idx).clone();

        write_id_cell(worksheet, row, worklog, &row_format)?;
        write_date_cell(worksheet, row, worklog, &row_format)?;
        write_duration_cell(worksheet, row, worklog, &row_format)?;
        write_description_cell(worksheet, row, worklog, &row_format)?;
        write_tags_cell(worksheet, row, worklog, styles, &row_format)?;
    }
    Ok(())
}

fn write_id_cell(
    worksheet: &mut Worksheet,
    row: u32,
    worklog: &Worklog,
    row_format: &Format,
) -> UseCaseResult<()> {
    let format = row_format
        .clone()
        .set_font_name("Consolas")
        .set_font_color(ID_COLOR);
    xlsx(worksheet.write_string_with_format(
        row,
        COL_ID,
        &worklog.id().as_uuid().to_string(),
        &format,
    ))?;
    Ok(())
}

fn write_date_cell(
    worksheet: &mut Worksheet,
    row: u32,
    worklog: &Worklog,
    row_format: &Format,
) -> UseCaseResult<()> {
    let format = row_format.clone().set_bold().set_font_color(DATE_COLOR);
    let value = jalali_date_string(worklog.datetime().as_datetime());
    xlsx(worksheet.write_string_with_format(row, COL_DATE, &value, &format))?;
    Ok(())
}

fn write_duration_cell(
    worksheet: &mut Worksheet,
    row: u32,
    worklog: &Worklog,
    row_format: &Format,
) -> UseCaseResult<()> {
    let format = row_format.clone().set_bold().set_font_color(DURATION_COLOR);
    let value = format_duration_secs(worklog.duration().as_std().as_secs());
    xlsx(worksheet.write_string_with_format(row, COL_DURATION, &value, &format))?;
    Ok(())
}

fn write_description_cell(
    worksheet: &mut Worksheet,
    row: u32,
    worklog: &Worklog,
    row_format: &Format,
) -> UseCaseResult<()> {
    let format = row_format.clone().set_text_wrap();
    let value = format_description(worklog);
    xlsx(worksheet.write_string_with_format(row, COL_DESCRIPTION, &value, &format))?;
    Ok(())
}

/// Multi-colored tags in a single cell via Excel rich strings.
fn write_tags_cell(
    worksheet: &mut Worksheet,
    row: u32,
    worklog: &Worklog,
    styles: &ExportStyles,
    row_format: &Format,
) -> UseCaseResult<()> {
    let tags: Vec<&str> = worklog.tags().iter().map(|t| t.as_str()).collect();
    if tags.is_empty() {
        xlsx(worksheet.write_string_with_format(row, COL_TAGS, "", row_format))?;
        return Ok(());
    }

    let mut segment_specs: Vec<(&Format, String)> = Vec::new();
    for (i, tag) in tags.iter().enumerate() {
        if i > 0 {
            segment_specs.push((&styles.tag_separator, ", ".to_string()));
        }
        let color_idx = tag_color_index(tag);
        segment_specs.push((&styles.tag_formats[color_idx], (*tag).to_string()));
    }

    let segments: Vec<(&Format, &str)> = segment_specs
        .iter()
        .map(|(format, text)| (*format, text.as_str()))
        .collect();

    xlsx(worksheet.write_rich_string_with_format(
        row,
        COL_TAGS,
        &segments,
        row_format,
    ))?;
    Ok(())
}
