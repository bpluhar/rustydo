use cursive::theme::{BaseColor, BorderStyle, Color, PaletteColor, Style, Theme};
use cursive::traits::Nameable;
use cursive::view::Resizable;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView, Panel};
use cursive::With;

fn custom_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    let border_color = Color::Light(BaseColor::Red);

    theme.palette[PaletteColor::Primary] = border_color;
    theme.palette[PaletteColor::Secondary] = border_color;

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::Red);
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Red);
    theme.palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Magenta);
    theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Red);
    theme.palette[PaletteColor::HighlightText] = Color::Light(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::TerminalDefault;

    theme
}

fn show_main_view(siv: &mut cursive::Cursive) {
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Left)
        .autojump();

    select.add_item("Do Laundry", "0");
    select.add_item("Do Dishes", "1");
    select.add_item("Do Groceries", "2");

    // First, create a reference to the TextView that we can modify later
    let task_info = TextView::new("Select a task to see details.")
        .style(Style::from(Color::Light(BaseColor::White)));

    // Modify the select's on_submit callback
    select.set_on_submit(move |siv: &mut cursive::Cursive, item: &str| {
        // Get different text based on the selected task
        let info_text = match item {
            "0" => "Task 1 Details: This is the first task...",
            "1" => "Task 2 Details: This is the second task...",
            "2" => "Task 3 Details: This is the third task...",
            _ => "Unknown task selected",
        };

        // Find the TextView by name and update its content
        siv.call_on_name("task_info", |view: &mut TextView| {
            view.set_content(info_text);
        });
    });

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                // Left side menu with border
                .child(
                    Panel::new(
                        LinearLayout::horizontal()
                            .child(DummyView.fixed_width(1))
                            .child(
                                LinearLayout::vertical()
                                    .child(DummyView.fixed_width(1))
                                    .child(select)
                                    .fixed_width(32)
                            )
                            .child(DummyView.fixed_width(1))
                    )
                    .title("Tasks")
                )
                // Right side content with border
                .child(
                    Panel::new(
                        LinearLayout::horizontal()
                            .child(DummyView.fixed_width(1))
                            .child(
                                LinearLayout::vertical()
                                    .child(DummyView.fixed_width(1))
                                    .child(task_info.with_name("task_info"))
                                    .child(DummyView.fixed_width(1))
                                    .full_width()
                            )
                            .child(DummyView.fixed_width(1))
                    )
                    .title("Details")
                )
        )
        .title("Rustydo 📋 | 1.0.1")
        // .button("Quit", |s| s.quit())
        .full_width()
        .full_height(),
    );
}

fn main() {
    let mut siv = cursive::default();
    siv.set_theme(custom_theme());

    show_main_view(&mut siv);
    siv.run();
}
