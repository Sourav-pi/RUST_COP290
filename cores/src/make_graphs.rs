use crate::parse::convert_to_index;
use crate::sheet::Sheet;
use charming::{
    Chart,
    component::{Axis, Legend, Title},
    element::{AxisType, Emphasis, ItemStyle, Orient, Symbol, Tooltip, Trigger},
    series::{Bar, Line, pie},
};
use std::{cmp::max, vec};

type Range = ((usize, usize), (usize, usize));

fn parse_range(range: &str) -> Result<Range, String> {
    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid range format".to_string());
    }
    let start = parts[0].trim();
    let end = parts[1].trim();
    if start.is_empty() || end.is_empty() {
        return Err("Range cannot be empty".to_string());
    }
    let start_tuple = convert_to_index(start.to_string());
    let end_tuple = convert_to_index(end.to_string());
    if start_tuple == (0, 0) || end_tuple == (0, 0) {
        return Err("Invalid cell reference".to_string());
    }
    if start_tuple.0 == end_tuple.0 && start_tuple.1 <= end_tuple.1 {
        Ok((start_tuple, end_tuple))
    } else if start_tuple.0 <= end_tuple.0 && start_tuple.1 == end_tuple.1 {
        return Ok((start_tuple, end_tuple));
    } else {
        return Err("Invalid range".to_string());
    }
}

fn parse_lables(labels: &str) -> Vec<String> {
    if labels.is_empty() {
        return Vec::new();
    }
    labels.split(',').map(|s| s.trim().to_string()).collect()
}
impl Sheet {
    pub fn line_graph(
        &self,
        range: &str,
        x_labels: &str,
        y_lable: &str,
        title: &str,
    ) -> Result<String, String> {
        let (start, end) = parse_range(range)?;
        let label_x: Vec<String> = if x_labels.is_empty() {
            let temp_range = max(end.0 - start.0 + 1, end.1 - start.1 + 1);
            let mut temp_vec: Vec<String> = Vec::new();
            for i in 1..(temp_range + 1) {
                temp_vec.push(format!("{}", i));
            }
            temp_vec
        } else {
            parse_lables(x_labels)
        };
        let mut values = Vec::new();
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                values.push(self.grid[i][j].value);
            }
        }
        Ok(Chart::new()
            .x_axis(Axis::new().type_(AxisType::Category).data(label_x))
            .title(Title::new().text(title))
            .y_axis(Axis::new().name(y_lable).type_(AxisType::Value))
            .series(Line::new().data(values))
            .to_string())
    }

    pub fn bar_graph(
        &self,
        range: &str,
        x_labels: &str,
        y_lable: &str,
        title: &str,
    ) -> Result<String, String> {
        println!("{}", x_labels);
        let (start, end) = parse_range(range)?;
        let label_x;
        if x_labels.is_empty() {
            let temp_range = max(end.0 - start.0 + 1, end.1 - start.1 + 1);
            let mut temp_vec: Vec<String> = Vec::new();
            for i in 1..(temp_range + 1) {
                temp_vec.push(format!("{}", i));
            }
            label_x = temp_vec;
        } else {
            label_x = parse_lables(x_labels);
            println!("label_x: {:?}", label_x);
        }
        let mut values = Vec::new();
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                values.push(self.grid[i][j].value);
            }
        }
        Ok(Chart::new()
            .x_axis(Axis::new().type_(AxisType::Category).data(label_x))
            .title(Title::new().text(title))
            .y_axis(Axis::new().name(y_lable).type_(AxisType::Value))
            .series(Bar::new().data(values))
            .to_string())
    }
    pub fn pie_graph(&self, range: &str, x_labels: &str, title: &str) -> Result<String, String> {
        let (start, end) = parse_range(range)?;
        let mut x_labels = parse_lables(x_labels);
        println!("x_labels: {:?}", x_labels);
        let mut values: Vec<i32> = Vec::new();
        let mut cnt = 0;

        if start.0 == end.0 {
            for i in start.1..=end.1 {
                values.push(self.grid[start.0][i].value);
                if x_labels.len() <= cnt {
                    x_labels.push(format!("{}", cnt + 1));
                }
                cnt += 1;
            }
        } else if start.1 == end.1 {
            for i in start.0..=end.0 {
                values.push(self.grid[i][start.1].value);
                if x_labels.len() <= cnt {
                    x_labels.push(format!("{}", cnt + 1));
                }
                cnt += 1;
            }
        } else {
            return Err("Invalid range".to_string());
        }
        let data: Vec<(i32, String)> = values.into_iter().zip(x_labels).collect();
        println!("{:?}", data);
        Ok(Chart::new()
            .tooltip(Tooltip::new().trigger(Trigger::Item))
            .legend(Legend::new().orient(Orient::Vertical).right("right"))
            .series(
                pie::Pie::new()
                    .radius(100)
                    .center(vec!["50%", "50%"])
                    .data(data)
                    .emphasis(
                        Emphasis::new().item_style(
                            ItemStyle::new()
                                .shadow_color("rgba(0, 0, 0, 0.5)")
                                .shadow_offset_x(0)
                                .shadow_offset_y(5)
                                .shadow_blur(10),
                        ),
                    ),
            )
            .title(Title::new().text(title))
            .to_string())
    }

    pub fn scatter_graph(
        &self,
        rangex: &str,
        rangey: &str,
        title: &str,
        x_name: &str,
        y_name: &str,
    ) -> Result<String, String> {
        let (start1, end1) = parse_range(rangex)?;
        let (start2, end2) = parse_range(rangey)?;

        let diff1 = (end1.0 - start1.0) as i32;
        let diff2 = (end1.1 - start1.1) as i32;
        let diff3 = (end2.0 - start2.0) as i32;
        let diff4 = (end2.1 - start2.1) as i32;
        if diff1 != diff3 || diff2 != diff4 {
            return Err("Invalid range".to_string());
        }
        let mut values: Vec<Vec<i32>> = Vec::new();
        for i in 0..diff1 + 1 {
            for j in 0..diff2 + 1 {
                let temp_vec: Vec<i32> = vec![
                    self.grid[start1.0 + i as usize][start1.1 + j as usize].value,
                    self.grid[start2.0 + i as usize][start2.1 + j as usize].value,
                ];
                values.push(temp_vec);
            }
        }
        Ok(Chart::new()
            .title(Title::new().text(title))
            .x_axis(Axis::new().type_(AxisType::Value).name(x_name))
            .y_axis(Axis::new().type_(AxisType::Value).name(y_name))
            .series(
                Line::new()
                    .data(values)
                    .symbol(Symbol::Circle)
                    .symbol_size(10)
                    .item_style(ItemStyle::new().color("blue")),
            )
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_graph() {
        let mut test_sheet = Sheet::new(6, 6);
        test_sheet.update_cell_data(1, 1, String::from("A2+A3"));
        test_sheet.update_cell_data(2, 1, String::from("90"));
        test_sheet.update_cell_data(3, 1, String::from("50"));
        test_sheet.update_cell_data(4, 1, String::from("A1+A2"));
        test_sheet.update_cell_data(5, 1, String::from("-5"));
        test_sheet.update_cell_data(6, 1, String::from("6"));
        let result = test_sheet.line_graph("A1:A6", "A2,A3,A4,A5,A6", "Y Axis", "Line Graph");
        assert!(result.is_ok());
    }
}
#[test]
fn test_scatter_graph() {
    let mut test_sheet = Sheet::new(6, 6);
    test_sheet.update_cell_data(1, 1, String::from("A2+A3"));
    test_sheet.update_cell_data(2, 1, String::from("90"));
    test_sheet.update_cell_data(3, 1, String::from("50"));
    test_sheet.update_cell_data(4, 1, String::from("A1+A2"));
    test_sheet.update_cell_data(5, 1, String::from("-5"));
    test_sheet.update_cell_data(6, 1, String::from("6"));
    let result = test_sheet.scatter_graph("A1:A6", "B1:B6", "Scatter Graph", "X Axis", "Y Axis");
    assert!(result.is_ok());
}
