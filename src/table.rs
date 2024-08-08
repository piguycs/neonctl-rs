use prettytable::{format, Cell, Row, Table};

pub fn print_table(titles: Row, data: Vec<Row>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    let titles = titles
        .iter()
        .map(|title| {
            let title_cell = Cell::from(title);
            title_cell.style_spec("bFg")
        })
        .collect();

    table.set_titles(Row::new(titles));

    for row in data {
        table.add_row(row);
    }

    table.printstd();
}
