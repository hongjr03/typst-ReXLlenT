#![cfg_attr(feature = "typst-plugin", allow(missing_docs))]

use core::num::NonZeroU32;
use getrandom::{register_custom_getrandom, Error};

// Some application-specific error code
const MY_CUSTOM_ERROR_CODE: u32 = Error::CUSTOM_START + 42;
pub fn always_fail(_buf: &mut [u8]) -> Result<(), Error> {
    let code = NonZeroU32::new(MY_CUSTOM_ERROR_CODE).unwrap();
    Err(Error::from(code))
}

register_custom_getrandom!(always_fail);

use std::io::Cursor;
use umya_spreadsheet::{
    reader, BorderStyleValues, Cell, HorizontalAlignmentValues, Spreadsheet, UnderlineValues,
    VerticalAlignmentValues, Worksheet,
};
use wasm_minimal_protocol::*;

wasm_minimal_protocol::initiate_protocol!();

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TableData {
    dimensions: TableDimensions,
    rows: Vec<RowData>,
    merged_cells: Vec<MergedCell>,
}

#[derive(Serialize, Deserialize)]
struct TableDimensions {
    columns: Vec<f64>,
    rows: Vec<f64>,
    max_columns: Option<u32>,
    max_rows: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct RowData {
    row_number: u32,
    cells: Vec<CellData>,
}

#[derive(Serialize, Deserialize)]
struct CellData {
    value: String,
    column: u32,
    style: Option<CellStyle>,
}

#[derive(Serialize, Deserialize)]
struct CellStyle {
    alignment: Option<Alignment>,
    border: Option<Border>,
    color: Option<String>,
    font: Option<FontStyle>,
}

#[derive(Serialize, Deserialize)]
struct Position {
    row: u32,
    column: u32,
}

#[derive(Serialize, Deserialize)]
struct MergedCell {
    range: String,
    start: Position,
    end: Position,
}

#[derive(Serialize, Deserialize)]
struct Alignment {
    horizontal: String,
    vertical: String,
}

#[derive(Serialize, Deserialize)]
struct Border {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

#[derive(Serialize, Deserialize)]
struct FontStyle {
    bold: bool,
    italic: bool,
    size: f64,
    color: Option<String>,
    underline: bool,
    strike: bool,
}

fn column_to_number(column: &str) -> u32 {
    column
        .chars()
        .fold(0, |acc, c| acc * 26 + (c as u32 - 'A' as u32 + 1))
}

fn parse_cell_reference(cell_ref: &str) -> (u32, u32) {
    let col_str: String = cell_ref.chars().take_while(|c| c.is_alphabetic()).collect();
    let row: u32 = cell_ref
        .chars()
        .skip_while(|c| c.is_alphabetic())
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    (column_to_number(&col_str), row)
}

fn parse_merge_range(range: &str) -> (String, String) {
    let parts: Vec<&str> = range.split(':').collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn get_table_dimensions(worksheet: &Worksheet) -> Result<(u32, u32), String> {
    let mut max_col = 0;
    let mut max_row = 0;

    for cell in worksheet.get_cell_collection() {
        let (col_num, row_num) = parse_cell_reference(&cell.get_coordinate().to_string());
        max_col = max_col.max(col_num);
        max_row = max_row.max(row_num);
    }

    if max_col == 0 || max_row == 0 {
        return Err("No data found in the worksheet".to_string());
    }

    Ok((max_col, max_row))
}

fn get_column_widths(worksheet: &Worksheet, max_col: u32, default_width: f64) -> Vec<f64> {
    let mut columns = vec![default_width; max_col as usize];
    for col in worksheet.get_column_dimensions() {
        let col_idx = *col.get_col_num() as usize - 1;
        if col_idx < columns.len() {
            columns[col_idx] = col.get_width().clone();
        }
    }
    columns
}

fn get_row_heights(worksheet: &Worksheet, max_row: u32, default_height: f64) -> Vec<f64> {
    let mut rows = vec![default_height; max_row as usize];
    for row in worksheet.get_row_dimensions() {
        let row_idx = (*row.get_row_num() as usize) - 1;
        if row_idx < rows.len() {
            rows[row_idx] = row.get_height().clone();
        }
    }
    rows
}

fn cell_value(cell: &Cell) -> Result<String, String> {
    if cell.get_raw_value().is_error() {
        return Err(format!(
            "Error in cell {}",
            cell.get_coordinate().to_string()
        ));
    } else {
        Ok(cell.get_value().to_string())
    }
}

#[cfg_attr(feature = "typst-plugin", wasm_func)]
pub fn to_typst(
    bytes: &[u8],
    sheet_index: &[u8],
    parse_alignment: &[u8],
    parse_border: &[u8],
    parse_bg_color: &[u8],
    parse_font_style: &[u8],
) -> Result<Vec<u8>, String> {
    let file = Cursor::new(bytes);
    let book: Spreadsheet = reader::xlsx::read_reader(file, true)
        .map_err(|e| format!("Failed to read Excel file: {}", e))?;
    // parse string -> bytes -> usize
    let sheet_index: usize = String::from_utf8(sheet_index.to_vec())
        .map_err(|e| format!("Failed to parse sheet index: {}", e))?
        .parse()
        .map_err(|e| format!("Failed to parse sheet index: {}", e))?;
    let parse_alignment: bool = String::from_utf8(parse_alignment.to_vec())
        .map_err(|e| format!("Failed to parse parse_alignment: {}", e))?
        .parse()
        .map_err(|e| format!("Failed to parse parse_alignment: {}", e))?;
    let parse_border: bool = String::from_utf8(parse_border.to_vec())
        .map_err(|e| format!("Failed to parse parse_border: {}", e))?
        .parse()
        .map_err(|e| format!("Failed to parse parse_border: {}", e))?;
    let parse_bg_color: bool = String::from_utf8(parse_bg_color.to_vec())
        .map_err(|e| format!("Failed to parse parse_bg_color: {}", e))?
        .parse()
        .map_err(|e| format!("Failed to parse parse_bg_color: {}", e))?;
    let parse_font_style: bool = String::from_utf8(parse_font_style.to_vec())
        .map_err(|e| format!("Failed to parse parse_font_style: {}", e))?
        .parse()
        .map_err(|e| format!("Failed to parse parse_font_style: {}", e))?;
    let worksheet = book
        .get_sheet(&sheet_index)
        .ok_or_else(|| "Failed to get worksheet".to_string())?;

    let (max_col, max_row) = get_table_dimensions(worksheet)?;

    let mut table_data = TableData {
        dimensions: TableDimensions {
            columns: Vec::new(),
            rows: Vec::new(),
            max_columns: Some(max_col),
            max_rows: Some(max_row),
        },
        rows: Vec::new(),
        merged_cells: Vec::new(),
    };

    // 处理表格尺寸

    let properties = worksheet.get_sheet_format_properties();
    table_data.dimensions.columns =
        get_column_widths(worksheet, max_col, *properties.get_default_column_width());
    table_data.dimensions.rows =
        get_row_heights(worksheet, max_row, *properties.get_default_row_height());

    // 处理合并单元格
    for merge_cell in worksheet.get_merge_cells() {
        let range = merge_cell.get_range().to_string();
        let (start, end) = parse_merge_range(&range);
        let (start_col, start_row) = parse_cell_reference(&start);
        let (end_col, end_row) = parse_cell_reference(&end);

        table_data.merged_cells.push(MergedCell {
            range,
            start: Position {
                row: start_row,
                column: start_col,
            },
            end: Position {
                row: end_row,
                column: end_col,
            },
        });
    }
    // 处理行数据
    for row_num in 1..=max_row {
        let row = worksheet.get_collection_by_row(&row_num);
        let mut row_data = RowData {
            row_number: row_num,
            cells: Vec::new(),
        };

        // 创建一个映射来存储每列的单元格
        let mut col_cell_map: Vec<Option<&Cell>> = vec![None; max_col as usize];
        for cell in row {
            let (col_num, _) = parse_cell_reference(&cell.get_coordinate().to_string());
            col_cell_map[(col_num - 1) as usize] = Some(cell);
        }

        // 处理每一列
        for col_num in 1..=max_col {
            // 检查是否是被合并的单元格
            let is_merged = table_data.merged_cells.iter().any(|mc| {
                row_num >= mc.start.row
                    && row_num <= mc.end.row
                    && col_num >= mc.start.column
                    && col_num <= mc.end.column
                    && !(row_num == mc.start.row && col_num == mc.start.column)
            });

            if !is_merged {
                if let Some(Some(cell)) = col_cell_map.get((col_num - 1) as usize) {
                    let cell_style = if parse_alignment || parse_font_style {
                        Some(CellStyle {
                            alignment: if parse_alignment {
                                get_cell_alignment(cell)
                            } else {
                                None
                            },
                            border: if parse_border {
                                get_cell_border(cell)
                            } else {
                                None
                            },
                            color: if parse_bg_color {
                                get_cell_bg_color(cell, &book)
                            } else {
                                None
                            },
                            font: if parse_font_style {
                                get_cell_font_style(cell, &book)
                            } else {
                                None
                            },
                        })
                    } else {
                        None
                    };

                    row_data.cells.push(CellData {
                        value: cell_value(cell)?,
                        column: col_num,
                        style: cell_style,
                    });
                }
            }
        }

        if !row_data.cells.is_empty() {
            table_data.rows.push(row_data);
        }
    }

    // 序列化为 TOML 然后转换为字节
    let toml_string =
        toml::to_string(&table_data).map_err(|e| format!("Failed to serialize to TOML: {}", e))?;

    let buffer = Vec::from(toml_string.as_bytes());
    Ok(buffer)
}

// 新增辅助函数
fn get_cell_alignment(cell: &Cell) -> Option<Alignment> {
    let style = cell.get_style();
    let alignment = match style.get_alignment() {
        Some(alignment) => alignment,
        None => return None,
    };

    Some(Alignment {
        horizontal: match alignment.get_horizontal() {
            HorizontalAlignmentValues::Left => "left",
            HorizontalAlignmentValues::Center => "center",
            HorizontalAlignmentValues::Right => "right",
            _ => "default",
        }
        .to_string(),
        vertical: match alignment.get_vertical() {
            VerticalAlignmentValues::Bottom => "bottom",
            VerticalAlignmentValues::Center => "center",
            VerticalAlignmentValues::Top => "top",
            _ => "default",
        }
        .to_string(),
    })
}

fn get_cell_border(cell: &Cell) -> Option<Border> {
    let style = cell.get_style();
    let border = match style.get_borders() {
        Some(border) => border,
        None => return None,
    };

    Some(Border {
        left: border.get_left().get_style() != &BorderStyleValues::None,
        right: border.get_right().get_style() != &BorderStyleValues::None,
        top: border.get_top().get_style() != &BorderStyleValues::None,
        bottom: border.get_bottom().get_style() != &BorderStyleValues::None,
    })
}

fn get_cell_bg_color(cell: &Cell, book: &Spreadsheet) -> Option<String> {
    let style = cell.get_style();
    let color = style.get_background_color()?;
    let argb = color.get_argb_with_theme(book.get_theme());
    if argb.is_empty() {
        Some("".to_string())
    } else {
        Some(if argb.len() == 8 {
            argb.chars().skip(2).collect::<String>() // skip 的作用是去掉前两位，即 alpha 通道
        } else {
            argb.to_string()
        })
    }
}

fn get_cell_font_style(cell: &Cell, book: &Spreadsheet) -> Option<FontStyle> {
    let font = match cell.get_style().get_font() {
        Some(font) => font,
        None => {
            return None;
        }
    };

    Some(FontStyle {
        bold: *font.get_font_bold().get_val(),
        italic: *font.get_font_italic().get_val(),
        size: *font.get_font_size().get_val(),
        color: {
            let argb = font.get_color().get_argb_with_theme(book.get_theme());
            if argb.is_empty() {
                None
            } else {
                Some(if argb.len() == 8 {
                    argb.chars().skip(2).collect::<String>() // skip 的作用是去掉前两位，即 alpha 通道
                } else {
                    argb.to_string()
                })
            }
        },
        underline: font.get_font_underline().get_val() != &UnderlineValues::None,
        strike: *font.get_font_strike().get_val(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    fn test_from_path(path: &str) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let sheet_index = "0".as_bytes();
        let parse_alignment = "true".as_bytes();
        let parse_border = "true".as_bytes();
        let parse_bg_color = "true".as_bytes();
        let parse_font_style = "true".as_bytes();

        let result = to_typst(
            &buffer,
            sheet_index,
            parse_alignment,
            parse_border,
            parse_bg_color,
            parse_font_style,
        )?;

        let toml_string = String::from_utf8(result).unwrap();
        assert_ne!(toml_string.len(), 0);
        Ok(())
    }

    #[test]
    fn test_default() {
        let path = "tests/data/default.xlsx";
        test_from_path(path).unwrap();
    }

    #[test]
    fn test_cell() {
        let paths: Vec<&str> = vec![
            "tests/data/cell/alignment.xlsx",
            "tests/data/cell/border.xlsx",
            "tests/data/cell/fill.xlsx",
            "tests/data/cell/incontinunity.xlsx",
            "tests/data/cell/merged.xlsx",
        ];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }

    #[test]
    fn test_font() {
        let paths: Vec<&str> = vec![
            "tests/data/font/bold.xlsx",
            "tests/data/font/fill.xlsx",
            "tests/data/font/italic.xlsx",
            "tests/data/font/size.xlsx",
            "tests/data/font/strike.xlsx",
            "tests/data/font/underline.xlsx",
        ];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }

    #[test]
    fn test_index() {
        let paths: Vec<&str> = vec!["tests/data/index/1.xlsx"];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }

    #[test]
    fn test_table() {
        let paths: Vec<&str> = vec![
            "tests/data/table/column_width.xlsx",
            "tests/data/table/row_height.xlsx",
        ];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }

    #[test]
    fn test_not_supported() {
        let paths: Vec<&str> = vec![
            "tests/data/not_supported/lowercase.xlsx",
            "tests/data/not_supported/rotate.xlsx",
            "tests/data/not_supported/uppercase.xlsx",
        ];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }

    #[test]
    fn test_examples() {
        let paths = vec![
            "examples/test.xlsx",
            "examples/typst.xlsx",
            "examples/typst_guy.xlsx",
            "examples/monet.xlsx",
        ];
        for path in paths {
            test_from_path(path).unwrap();
        }
    }
}
