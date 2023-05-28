use tui::widgets::List;

pub fn create_list(intervals: &[i32], selected_index: usize) -> List<'_> {
    let items: Vec<_> = intervals
        .iter()
        .enumerate()
        .map(|(i, interval)| {
            let content = format!("{} minutes", interval);
            if i == selected_index {
                format!("> {}", content)
            } else {
                format!("  {}", content)
            }
        })
        .collect();
    List::new(items)
}
