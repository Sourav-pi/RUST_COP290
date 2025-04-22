use super::row::Row;
use crate::components::spreadsheet::*;
use dioxus::prelude::*;

const GRID_STYLE: &str = "
    overflow: hidden;
    height: calc(100vh - 120px);
    width: 100%;
    display: block;
    position: relative;
    border: 1px solid #e0e0e0;
    box-shadow: inset 0 0 5px rgba(0,0,0,0.1);
";

const TABLE_STYLE: &str = "
    display: table;
    min-width: max-content;
    border-collapse: collapse;
";

const NAVIGATION_CONTROLS_STYLE: &str = "
    position: absolute;
    bottom: 16px;
    right: 16px;
    display: flex;
    gap: 8px;
    z-index: 100;
";

const NAV_BUTTON_STYLE: &str = "
    width: 40px;
    height: 40px;
    background-color: rgba(255, 255, 255, 0.8);
    border: 1px solid #ccc;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    &:hover {
        background-color: rgba(240, 240, 240, 0.9);
    }
";

const PAGE_INFO_STYLE: &str = "
    background-color: rgba(255, 255, 255, 0.8);
    padding: 5px 10px;
    border-radius: 12px;
    border: 1px solid #ccc;
    font-size: 14px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
";

#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    pub num_rows: i32,
    pub num_cols: i32,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    let mut start_row_ctx = use_signal(|| 1);
    let mut start_col_ctx = use_signal(|| 1);
    let mut selected_cell = use_context::<SelectedCellContext>();

    let min_cell_width = 81; // Minimum width per cell in pixels

    // Visible rows per page
    let rows_per_page = 20;
    let cols_per_page = 16;

    // Read context values once at the beginning of the component
    let start_row = start_row_ctx.cloned();
    let start_col = start_col_ctx.cloned();

    // Calculate max pages once based on these values
    let max_start_row = (props.num_rows - rows_per_page).max(1);
    let max_start_col = (props.num_cols - cols_per_page).max(1);

    // Calculate end positions based on the variables we already have
    let end_row = (start_row + rows_per_page - 1).min(props.num_rows);
    let end_col = (start_col + cols_per_page - 1).min(props.num_cols);

    // Function to check if a cell is visible in the current view
    let is_cell_visible = move |row: i32, col: i32| -> bool {
        row >= start_row && row <= end_row && col >= start_col && col <= end_col
    };

    // Function to scroll to a cell that's not in view
    let mut scroll_to_cell = move |row: i32, col: i32| {
        // If the cell is above current view, scroll up to show it at the top
        if row < start_row {
            start_row_ctx.set((row - 1).max(1));
        }
        // If the cell is below current view, scroll down to show it at the bottom
        else if row > end_row {
            let new_start = (row - rows_per_page + 1).max(1).min(max_start_row);
            start_row_ctx.set(new_start);
        }

        // If the cell is to the left of current view, scroll left to show it
        if col < start_col {
            start_col_ctx.set((col - 1).max(1));
        }
        // If the cell is to the right of current view, scroll right to show it
        else if col > end_col {
            let new_start = (col - cols_per_page + 1).max(1).min(max_start_col);
            start_col_ctx.set(new_start);
        }
    };

    // Scrolling helper functions remain the same
    let mut move_up_help = move || {
        let current_row = start_row_ctx.cloned();
        if current_row > 1 {
            start_row_ctx.set(current_row - 1);
        }
    };

    let mut move_down_help = move || {
        let current_row = start_row_ctx.cloned();
        if current_row < max_start_row {
            start_row_ctx.set(current_row + 1);
        }
    };

    let mut move_left_help = move || {
        let current_col = start_col_ctx.cloned();
        if current_col > 1 {
            start_col_ctx.set(current_col - 1);
        }
    };

    let mut move_right_help = move || {
        let current_col = start_col_ctx.cloned();
        if current_col < max_start_col {
            start_col_ctx.set(current_col + 1);
        }
    };

    // Effect to handle auto-scrolling when selected cell changes
    use_effect(move || {
        let (sel_row, sel_col) = *selected_cell.read();

        // If the currently selected cell is not visible, scroll to it
        if !is_cell_visible(sel_row, sel_col) {
            scroll_to_cell(sel_row, sel_col);
        }
    });

    // Navigation handle
    let move_up = move |_| {
        move_up_help();
    };
    let move_down = move |_| {
        move_down_help();
    };
    let move_left = move |_| {
        move_left_help();
    };
    let move_right = move |_| {
        move_right_help();
    };

    // Calculate end positions based on the variables we already have
    let end_row = (start_row + rows_per_page - 1).min(props.num_rows);
    let end_col = (start_col + cols_per_page - 1).min(props.num_cols);

    // Calculate visible range

    let on_keydown = {
        let row = selected_cell.cloned().0;
        let col = selected_cell.cloned().1;
        move |e: Event<KeyboardData>| {
            let key = e.key();
            if key == Key::Enter
                || key == Key::ArrowDown
                || key == Key::ArrowUp
                || key == Key::Tab
                || key == Key::ArrowLeft
                || key == Key::ArrowRight
            {
                e.prevent_default();
                let mut to_row = row;
                let mut to_col = col;

                // Store the selected cell position once to avoid inconsistencies
                let current_row = selected_cell.cloned().0;
                let current_col = selected_cell.cloned().1;

                // Handle vertical movement - only change row, not column
                if e.key() == Key::ArrowDown || e.key() == Key::Enter {
                    if current_row == end_row && start_row_ctx.cloned() < max_start_row {
                        move_down_help();
                    };
                    to_row = (row + 1).max(1).min(props.num_rows); // Prevent going past the last row
                } else if e.key() == Key::ArrowUp {
                    if current_row == start_row && start_row_ctx.cloned() > 1 {
                        move_up_help();
                    };
                    to_row = (row - 1).max(1).min(props.num_rows); // Prevent going before the first row
                }
                // Handle horizontal movement - only change column, not row
                else if (e.key() == Key::Tab && e.modifiers().shift())
                    || e.key() == Key::ArrowLeft
                {
                    if current_col == start_col && start_col_ctx.cloned() > 1 {
                        move_left_help();
                    };
                    to_col = (col - 1).max(1).min(props.num_cols); // Prevent going before the first column
                } else if e.key() == Key::Tab || e.key() == Key::ArrowRight {
                    if current_col == end_col && start_col_ctx.cloned() < max_start_col {
                        move_right_help();
                    };
                    to_col = (col + 1).max(1).min(props.num_cols); // Prevent going past the last column
                }

                let script = format!(
                    r#"
                    let x = document.getElementById('row-{}-col-{}');
                    if (x) {{
                        x.focus();
                    }}
                "#,
                    to_row, to_col
                );
                selected_cell.set((to_row, to_col));
                document::eval(&script);
            }
        }
    };

    rsx! {
        div {
            style: GRID_STYLE,
            class: "grid-container",
            onkeydown: on_keydown,
            div {
                style: TABLE_STYLE,
                class: "spreadsheet-table",
                // Header row (always visible)
                Row {
                    row: 0,
                    num_cols: cols_per_page,
                    is_header: true,
                    min_width: min_cell_width,
                    start_col: start_col,
                    end_col: end_col,
                }

                // Visible rows
                for i in (start_row)..=(end_row) {
                    Row {
                        row: i,
                        num_cols: cols_per_page,
                        is_header: false,
                        min_width: min_cell_width,
                        start_col: start_col,
                        end_col: end_col,
                    }
                }
            }

            // Navigation controls
            div {
                style: NAVIGATION_CONTROLS_STYLE,

                // Page info display
                div {
                    style: PAGE_INFO_STYLE,
                    "Rows: {start_row}-{end_row} / {props.num_rows -1 }, Cols: {start_col}-{end_col} / {props.num_cols -1}"
                }

                // Navigation buttons
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_up,
                    disabled: start_row <= 1,
                    id: "up-button",
                    "↑"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_down,
                    disabled: start_row >= max_start_row,
                    id: "down-button",
                    "↓"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_left,
                    disabled: start_col <= 1,
                    id: "left-button",
                    "←"
                }
                button {
                    style: NAV_BUTTON_STYLE,
                    onclick: move_right,
                    disabled: start_col >= max_start_col,
                    id: "right-button",
                    "→"
                }
            }
        }
    }
}
