use cursive::traits::*;
use cursive::views::*;
use cursive::view::ViewWrapper;
use crate::tasks::TaskManager;

const MENU_WIDTH: usize = 32;

pub fn create_main_view(siv: &mut cursive::Cursive) {
    let task_manager = TaskManager::new();
    siv.set_user_data(task_manager);

    let select = create_task_select();
    let task_info = create_task_info();

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(create_tasks_panel(select))
                .child(create_details_panel(task_info))
        )
        .title("Rustydo 📋 | 1.0.1")
        .full_width()
        .full_height(),
    );
}

fn create_task_select() -> SelectView<String> {
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Left)
        .autojump();
    
    // Add items from TaskManager
    select.set_on_submit(handle_task_selection);
    select
}

fn handle_task_selection(siv: &mut cursive::Cursive, item: &str) {
    let info_text = {
        let task_manager = siv.user_data::<TaskManager>().unwrap();
        task_manager.get_info()
    };

    siv.call_on_name("task_info", |view: &mut TextView| {
        view.set_content(info_text);
    });
}

fn create_task_info() -> TextView {
    TextView::new("Select a task to see details")
        .with_name("task_info")
}

fn create_tasks_panel(select: SelectView<String>) -> ResizedView<ResizedView<Panel<ScrollView<SelectView<String>>>>> {
    Panel::new(ScrollView::new(select))
        .title("Tasks")
        .fixed_width(MENU_WIDTH)
        .full_width()
}

fn create_details_panel(task_info: NamedView<TextView>) -> Panel<ScrollView<NamedView<TextView>>> {
    Panel::new(ScrollView::new(task_info))
        .title("Details")
}

// ... helper functions for creating panels ...
