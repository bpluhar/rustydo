mod task;
mod theme;

use cursive::theme::{BaseColor, Color, Style};
use cursive::traits::Nameable;
use cursive::view::Resizable;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, Panel, SelectView, TextView};

use crate::task::Task;
use crate::theme::set_theme;

fn show_main_view(siv: &mut cursive::Cursive) {
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Left)
        .autojump();

    // Load existing tasks
    if let Ok(tasks) = Task::read_all_tasks() {
        for task in tasks {
            select.add_item(task.title.clone(), task);
        }
    }

    // Create "Add Task" button callback
    let add_task = move |s: &mut cursive::Cursive| {
        s.add_layer(
            Dialog::new()
                .title("Add New Task")
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new("Title:"))
                        .child(EditView::new().with_name("title"))
                        .child(DummyView.fixed_height(1))
                        .child(TextView::new("Details:"))
                        .child(EditView::new().with_name("details")),
                )
                .button("Create", |s| {
                    let title = s
                        .call_on_name("title", |view: &mut EditView| {
                            view.get_content().to_string()
                        })
                        .unwrap();
                    let details = s
                        .call_on_name("details", |view: &mut EditView| {
                            view.get_content().to_string()
                        })
                        .unwrap();

                    if let Ok(task) = Task::new(title.clone(), details) {
                        // Update the select view with the new task
                        s.call_on_name("task_select", |view: &mut SelectView<Task>| {
                            view.add_item(title, task);
                        });
                    }
                    s.pop_layer();
                })
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    };

    let task_info = TextView::new("Select a task to see details.")
        .style(Style::from(Color::Light(BaseColor::White)));

    // Update select's on_submit callback to show task details
    select.set_on_submit(move |siv: &mut cursive::Cursive, task: &Task| {
        let info_text = format!(
            "{}\n{}\n{}\n\n{}",
            task.creation_date.format("%B %e, %Y"),
            if task.completed {
                "Completed"
            } else {
                "Pending"
            },
            task.completed_date.map_or(String::new(), |date| format!(
                "Completed on: {}",
                date.format("%Y-%m-%d %H:%M:%S")
            )),
            task.details,
        );

        siv.call_on_name("task_info", |view: &mut TextView| {
            view.set_content(info_text);
        });
    });

    // Give the select view a name so we can reference it later
    let select = select.with_name("task_select");

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                // Tasks panel
                .child(
                    Panel::new(
                        LinearLayout::horizontal()
                            .child(DummyView.fixed_width(1))
                            .child(
                                LinearLayout::vertical()
                                    .child(DummyView.fixed_width(1))
                                    .child(select)
                                    .child(DummyView.full_height())
                                    .child(Dialog::new().button("Add Task", add_task).max_height(3))
                                    .fixed_width(32),
                            )
                            .child(DummyView.fixed_width(1)),
                    )
                    .title("Tasks"),
                )
                // Details panel
                .child(
                    Panel::new(
                        LinearLayout::horizontal()
                            .child(DummyView.fixed_width(1))
                            .child(
                                LinearLayout::vertical()
                                    .child(DummyView.fixed_width(1))
                                    .child(task_info.with_name("task_info"))
                                    .child(DummyView.fixed_width(1))
                                    .full_width(),
                            )
                            .child(DummyView.fixed_width(1)),
                    )
                    .title("Details"),
                ),
        )
        .title("Rustydo 📋 | 1.0.1")
        .full_width()
        .full_height(),
    );
}

fn main() {
    let mut siv = cursive::default();
    siv.set_theme(set_theme());

    show_main_view(&mut siv);
    siv.run();
}
