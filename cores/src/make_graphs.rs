use charming::{Chart, component::{Axis,Title,Legend}, element::{AxisType,Emphasis, ItemStyle, Orient, Tooltip, Trigger, Symbol}, series::{Bar, Line,pie}};
use crate::sheet::Sheet;
use crate::parse::convert_to_index;
use charming::datatype::DataPoint;
use charming::datatype::CompositeValue;

fn parse_range(range: &str) -> Result<((usize, usize), (usize, usize)), String> {
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
        return Ok((start_tuple, end_tuple));
    } else if start_tuple.0 <= end_tuple.0 && start_tuple.1 == end_tuple.1 {
        return Ok((start_tuple, end_tuple));
    } else {
        return Err("Invalid range".to_string());
    }
}

fn parse_lables(labels: &str) -> Vec<String> {
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
        let x_labels = parse_lables(x_labels);
        let mut values = Vec::new();
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                values.push(self.grid[i][j].value.clone());
            }
        };
        Ok(Chart::new()
        .x_axis(Axis::new().type_(AxisType::Category).data(x_labels))
        .title(Title::new().text(title))
        .y_axis(Axis::new().name(y_lable).type_(AxisType::Value))
        .series(Line::new().data(values)).to_string())
    }

    pub fn bar_graph(
        &self,
        range: &str,
        x_labels: &str,
        y_lable: &str,
        title: &str,
    ) -> Result<String, String> {
        let (start, end) = parse_range(range)?;
        let x_labels = parse_lables(x_labels);
        let mut values = Vec::new();
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                values.push(self.grid[i][j].value.clone());
            }
        };
        Ok(Chart::new()
        .x_axis(Axis::new().type_(AxisType::Category).data(x_labels))
        .title(Title::new().text(title))
        .y_axis(Axis::new().name(y_lable).type_(AxisType::Value))
        .series(Bar::new().data(values)).to_string())
    }
    pub fn pie_graph(
        &self,
        range: &str,
        x_labels: &str,
        title: &str,
    ) -> Result<String, String> {
        let (start, end) = parse_range(range)?;
        let x_labels = parse_lables(x_labels);
        let mut values:Vec<i32> = Vec::new();
        let mut cnt=0;
        
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                if cnt<x_labels.len(){
                    values.push(self.grid[i][j].value.clone());
                    cnt+=1;
                }
                else{
                    values.push(self.grid[i][j].value.clone());
                }
            }
        };
    let data: Vec<DataPoint> = values.into_iter()
    .zip(x_labels.into_iter())
    .map(|(v, label)| {
        DataPoint::from(
            CompositeValue::from(vec![
                CompositeValue::from(v),
                CompositeValue::from(label),
            ])
        )
    })
    .collect();
        Ok(Chart::new()
        .tooltip(Tooltip::new().trigger(Trigger::Item))
        .legend(Legend::new().orient(Orient::Vertical).left("left"))
        .series(pie::Pie::new()
            .radius(150)
            .center(vec!["50%", "50%"])
            .data(data)
            .emphasis(Emphasis::new().item_style(ItemStyle::new().shadow_color("rgba(0, 0, 0, 0.5)").shadow_offset_x(0).shadow_offset_y(5).shadow_blur(10)))
        )
        .title(Title::new().text(title)).to_string())
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

        let diff1= start1.0 - end1.0;
        let diff2= start1.1 - end1.1;
        let diff3= start2.0 - end2.0;
        let diff4= start2.1 - end2.1;
        if diff1!=diff3 || diff2!=diff4{
            return Err("Invalid range".to_string());
        }
        let mut values:Vec<Vec<i32>> = Vec::new();
        for i in 0..diff1+1 {
            for j in 0..diff2+1 {
                let mut temp_vec:Vec<i32> = Vec::new();
                temp_vec.push(self.grid[start1.0+i][start1.1+j].value.clone());
                temp_vec.push(self.grid[start2.0+i][start2.1+j].value.clone());
                values.push(temp_vec);
            }
        };
        Ok(Chart::new()
        .title(Title::new().text(title))
        .x_axis(Axis::new().type_(AxisType::Value).name(x_name))
        .y_axis(Axis::new().type_(AxisType::Value).name(y_name))
        .series(Line::new().data(values).symbol(Symbol::Circle).symbol_size(10).item_style(ItemStyle::new().color("blue")))
        .to_string())
    }
}
