use anyhow::Result;
use std::rc::Rc;

mod db;
use db::*;
mod io_utils;
use io_utils::*;
mod models;
mod navigator;
use navigator::*;
mod ui;

fn main() -> Result<()> {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(db.clone());

    loop {
        clearscreen::clear().unwrap();

        match navigator.get_current_page() {
            Some(page) => {
                page.draw_page()?;
                let user_input = get_user_input();
                if let Some(action) = page.handle_input(&user_input)? {
                    navigator.handle_action(action)?;
                }
            }
            None => break,
        }
    }

    Ok(())
}
