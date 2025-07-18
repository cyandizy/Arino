use rusqlite::Connection;

use crate::{cli::prompt, miscellaneous::calculate_mean};

// Gets dish ID from dish name
pub fn dish_id(dish: &str, conn: &Connection) -> Option<u32> {
    let id: Option<u32> =
        match conn.query_row("SELECT id FROM dishes WHERE name = ?1;", [dish], |row| {
            row.get(0)
        }) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid dish name");
                None
            }
            Err(e) => {
                eprintln!("Error: {e}");
                None
            }
        };

    id
}

// Gets dish name from dish ID
pub fn dish_name(dish_id: u32, conn: &Connection) -> Option<String> {
    let name: Option<String> = match conn.query_row(
        "SELECT name FROM dishes WHERE id = ?1;",
        [&dish_id],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Invalid dish id");
            None
        }
        Err(e) => {
            eprintln!("Error: {e}");
            None
        }
    };

    name
}

// Gets ingredient id from ingredient name
pub fn ingredient_id(ingredient: &str, conn: &Connection) -> Option<u32> {
    let id: Option<u32> = match conn.query_row(
        "SELECT id FROM ingredients WHERE name = ?1;",
        [&ingredient],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Invalid ingredient name");
            None
        }
        Err(e) => {
            eprintln!("Error: {e}");
            None
        }
    };

    id
}

// Gets category name from user, and return its id and itself.
pub fn category_name_and_id(conn: &Connection) -> Option<(String, u32)> {
    let (category_name, category_id) = loop {
        let input_category_name =
            prompt("Category (vegetable, fruit, dairy, meat, condiment, grain)");
        if input_category_name.is_empty() {
            return None;
        }

        let retrieved_category_id: u32 = match conn.query_row(
            "SELECT id FROM categories WHERE name = ?1;",
            [&input_category_name],
            |row| row.get(0),
        ) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid category");
                continue;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        break (input_category_name, retrieved_category_id);
    };

    Some((category_name, category_id))
}

// Gets price from ingredient ID
pub fn price(ingredient_id: u32, conn: &Connection) -> Option<f32> {
    let mut price_query = match conn.prepare("SELECT price FROM prices WHERE ingredient_id = ?1;") {
        Ok(query) => query,
        Err(e) => {
            eprintln!("Error preparing query: {e}");
            return None;
        }
    };

    let prices_iter = match price_query.query_map([ingredient_id], |row| Ok(row.get::<_, f32>(0)?))
    {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Error executing query: {e}");
            return None;
        }
    };

    let mut prices: Vec<f32> = Vec::new();
    for price in prices_iter {
        match price {
            Ok(p) => prices.push(p),
            Err(e) => {
                eprintln!("Error retrieving price: {e}");
                return None;
            }
        }
    }

    Some(calculate_mean(prices))
}
