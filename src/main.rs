use std::rc::Rc;

mod db;
use db::*;
mod io_utils;
use io_utils::*;
mod models;
mod navigator;
use navigator::*;
mod ui;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(db.clone());

    loop {
        clearscreen::clear().unwrap();

        match navigator.get_current_page() {
            Some(page) => {
                if let Err(error) = page.draw_page() {
                    println!("Error rendering page: {error}\nPress any key to continue...");
                    wait_for_key_press();
                }

                let user_input = get_user_input();

                match page.handle_input(&user_input) {
                    Ok(action) => {
                        if let Some(action) = action {
                            if let Err(error) = navigator.handle_action(action) {
                                println!(
                                    "Error processing user input: {error}\nPress any key to continue..."
                                );
                                wait_for_key_press();
                            }
                        }
                    }
                    Err(error) => {
                        println!("Error getting user input: {error}\nPress any key to continue...");
                        wait_for_key_press();
                    }
                }
            }
            None => break,
        }
    }
}
