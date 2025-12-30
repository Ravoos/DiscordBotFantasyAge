use serenity::all::*;
use std::cmp;

fn paginate_vec(
    items: &[String],
    page: usize,
    per_page: usize,
) -> (Vec<String>, usize, usize) {
    let total_pages = (items.len() + per_page - 1) / per_page;
    let clamped_page = cmp::min(page, total_pages.saturating_sub(1));

    let start = clamped_page * per_page;
    let end = cmp::min(start + per_page, items.len());

    (items[start..end].to_vec(), total_pages, clamped_page)
}

fn split_stunt(stunt: &str) -> (&str, &str) {
    if let Some((name, rest)) = stunt.split_once(":*** ") {
        (name.trim_matches('*'), rest)
    } else {
        ("Stunt", stunt)
    }
}

pub fn build_stunt_page(
    title: &str,
    stunts: &[String],
    page: usize,
) -> (CreateEmbed, Vec<CreateActionRow>) {
    let per_page = 5;
    let (items, total_pages, _clamped_page) = paginate_vec(stunts, page, per_page);

    let mut embed = CreateEmbed::new()
        .title(format!("{} (Page {}/{})", title, page + 1, total_pages));

    for stunt in items {
        let (name, description) = split_stunt(&stunt);
        embed = embed.field(
            format!("***{name}***"),
            description,
            false);
    }

    let mut buttons = Vec::new();

    if page > 0 {
        buttons.push(
            CreateButton::new(format!("{}:{}", title, page - 1))
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

    let action_rows = if buttons.is_empty() {
        Vec::new()
    } else {
        vec![CreateActionRow::Buttons(buttons)]
    };

    (embed, action_rows)
}
