use serenity::all::*;
use std::cmp;

pub fn paginate_vec(
    items: &[String],
    page: usize,
    per_page: usize,
) -> (Vec<String>, usize) {
    let total_pages = (items.len() + per_page - 1) / per_page;
    let clamped_page = cmp::min(page, total_pages.saturating_sub(1));

    let start = clamped_page * per_page;
    let end = cmp::min(start + per_page, items.len());

    (items[start..end].to_vec(), total_pages)
}

pub fn build_stunt_page(
    title: &str,
    stunts: &[String],
    page: usize,
) -> (CreateEmbed, Vec<CreateActionRow>) {
    let per_page = 5;
    let total_pages = (stunts.len() + per_page - 1) / per_page;
    let (_items, page) = paginate_vec(stunts, page, per_page);

    let embed = CreateEmbed::new()
        .title(format!("{} (Page {}/{})", title, page + 1, total_pages));

    let mut action_rows = Vec::new();

    let mut buttons = Vec::new();

    if page > 0 {
        buttons.push(
            CreateButton::new(format!("{}:{}", title, page.saturating_sub(1)))
                .label("Prev")
                .style(ButtonStyle::Secondary),
        );
    }

    if page + 1 < total_pages {
        buttons.push(
            CreateButton::new(format!("{}:{}", title, page + 1))
                .label("Next")
                .style(ButtonStyle::Secondary),
        );
    }

    if !buttons.is_empty() {
        action_rows.push(CreateActionRow::Buttons(buttons));
    }

    (embed, action_rows)
}
