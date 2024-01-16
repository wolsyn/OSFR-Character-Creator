use log::error;
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FacePaint {
    id: usize,
    texture_alias: String
}

#[derive(Debug, Serialize)]
pub struct Hair {
    id: usize,
    addr: String,
    name: String
}
#[derive(Debug, Serialize)]
pub struct PixieWings {
    id: usize,
    addr: String,
}

#[derive(Debug, Serialize)]
pub struct EyeColor {
    name: String,
    color: u8
}
#[derive(Debug, Serialize)]
pub struct HairColor {
    name: String,
    color: u8
}

#[derive(Debug, Serialize)]
pub struct Extras {
    id: usize,
    name: String,
    species: String,
    gender: String,
    addr: String
}

/// Retrieves eye color data from a SQLite database.
///
/// # Arguments
///
/// * `path` - A string representing the path to the SQLite database file.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `EyeColor` structs or a `rusqlite::Error` if an
/// error occurs during the database operation.
///
/// # Example
///
/// ```rust
/// use your_module::get_eye_color;
///
/// // Provide the path to your SQLite database file
/// let path = "path/to/your/database.db";
///
/// match get_eye_color(path) {
///     Ok(eye_colors) => {
///         // Successfully retrieved eye colors
///         for color in eye_colors {
///             println!("Name: {}, Color: {}", color.name, color.color);
///         }
///     }
///     Err(err) => {
///         // Handle the error gracefully
///         eprintln!("Error retrieving eye colors: {:?}", err);
///     }
/// }
/// ```
pub fn get_eye_color(path: &str) -> Result<Vec<EyeColor>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut eye_colors: Vec<EyeColor> = vec![];

    let mut stmt = conn.prepare("SELECT name, color FROM Eye_Color")?;

    let eye_iter = stmt.query_map([], |row| {
        Ok(
            EyeColor {
                name: row.get(0)?,
                color: row.get(1)?
            }
        )
    })?;

    for color in eye_iter {
        let color = color.unwrap();
        eye_colors.push(
            EyeColor { name: color.name, color: color.color }
        );
    }

    Ok(eye_colors)
}

/// Retrieves face paint data from a SQLite database.
///
/// # Arguments
///
/// * `path` - A string representing the path to the SQLite database file.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `FacePaint` structs or a `rusqlite::Error` if an
/// error occurs during the database operation.
///
/// # Example
///
/// ```rust
/// use your_module::get_facepaints;
///
/// // Provide the path to your SQLite database file
/// let path = "path/to/your/database.db";
///
/// match get_facepaints(path) {
///     Ok(facepaints) => {
///         // Successfully retrieved face paints
///         for facepaint in facepaints {
///             println!("ID: {}, Texture Alias: {}", facepaint.id, facepaint.texture_alias);
///         }
///     }
///     Err(err) => {
///         // Handle the error gracefully
///         eprintln!("Error retrieving face paints: {:?}", err);
///     }
/// }
/// ```
pub fn get_facepaints(path: &str) -> Result<Vec<FacePaint>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut facepaints: Vec<FacePaint> = vec![];

    let mut stmt = conn.prepare("SELECT id, texture_alias FROM FacePaint")?;
    let facepaint_iter = stmt.query_map([], |row| {
        Ok(FacePaint {
            id: row.get(0)?,
            texture_alias: row.get(1)?,
        })
    })?;
    for facepaint in facepaint_iter {
        let facepaint = facepaint.unwrap();
        let buff_facepaints = FacePaint {
            id: facepaint.id,
            texture_alias: facepaint.texture_alias
        };
        facepaints.push(buff_facepaints);

    }
    Ok(facepaints)
}

/// Retrieves a filtered list of `Hair` based on the specified criteria.
///
/// This function queries the database located at the given `path` and retrieves a list of `Hair`
/// items that match the specified `target_gender`. The function returns a `Result` containing
/// a `Vec<Hair>` on success, and it may return a `rusqlite::Error` in case of a database error.
///
///
/// # Returns
///
/// A `Result` containing a vector of `Hair` items retrieved from the database on success,
/// or a `rusqlite::Error` in case of a database error.
///
///
/// # Examples
///
/// ```
/// use your_module::get_hairs;
///
/// // Assuming a database path and target gender are properly defined
/// let result = get_hairs("path/to/database.db", "male");
///
/// match result {
///     Ok(hairs) => {
///         // Process the retrieved hair list
///         for hair in hairs {
///             println!("Hair ID: {}, Address: {}, Name: {}", hair.id, hair.addr, hair.name);
///         }
///     },
///     Err(error) => {
///         // Handle the error gracefully
///         eprintln!("Error retrieving hair list: {:?}", error);
///     }
/// }
/// ```
pub fn get_hairs(path: &str, target_gender: &str) -> Result<Vec<Hair>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut hairs: Vec<Hair> = vec![];

    let mut stmt = conn.prepare("SELECT id, addr, name FROM Hair WHERE gender = ?")?;

    let extra_iter = match stmt.query_map([target_gender], |row| {
        Ok(Hair {
            id: row.get(0)?,
            addr: row.get(1)?,
            name: row.get(2)?
        })
    }) {
    Ok(mapped_rows) => {mapped_rows},
    Err(e) => {
        error!("Error iterating Extras due to {:#?}", e);
        panic!();
    },
};

    for hair in extra_iter {
        let hair = hair.unwrap();
        let buff_hair = Hair {
            id: hair.id,
            addr: hair.addr,
            name: hair.name
        };
        hairs.push(buff_hair);

    }
    Ok(hairs)
}


/// Retrieves hair color data from a SQLite database.
///
/// # Arguments
///
/// * `path` - A string representing the path to the SQLite database file.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `HairColor` structs or a `rusqlite::Error` if an
/// error occurs during the database operation.
///
/// # Example
///
/// ```rust
/// use your_module::get_hair_color;
///
/// // Provide the path to your SQLite database file
/// let path = "path/to/your/database.db";
///
/// match get_hair_color(path) {
///     Ok(hair_colors) => {
///         // Successfully retrieved hair colors
///         for color in hair_colors {
///             println!("Name: {}, Color: {}", color.name, color.color);
///         }
///     }
///     Err(err) => {
///         // Handle the error gracefully
///         eprintln!("Error retrieving hair colors: {:?}", err);
///     }
/// }
/// ```
pub fn get_hair_color(path: &str) -> Result<Vec<HairColor>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut hair_colors:Vec<HairColor> = vec![];

    let mut stmt = conn.prepare("SELECT name, color FROM Hair_Color")?;

    let hair_iter = stmt.query_map([], |row| {
        Ok(
            EyeColor {
                name: row.get(0)?,
                color: row.get(1)?
            }
        )
    })?;

    for color in hair_iter {
        let color = color.unwrap();
        hair_colors.push(
            HairColor { name: color.name, color: color.color }
        );
    }

    Ok(hair_colors)
}

/// This function retrieves a filtered list of `Extras` based on the specified gender and species.
///
/// # Arguments
///
/// * `path` - A reference to the path of the SQLite database file.
/// * `target_gender` - A reference to the target gender for filtering.
/// * `target_species` - A reference to the target species for filtering.
///
/// # Returns
///
/// * `Result<Vec<Extras>, rusqlite::Error>` - A `Result` containing a `Vec<Extras>` if the operation is successful,
///   otherwise an `rusqlite::Error` indicating the nature of the failure.
///
/// # Panics
///
/// This function may panic if there are errors during SQLite database operations, such as preparing statements
/// or iterating through the query results.
///
/// # Examples
///
/// ```rust
/// let path = "path/to/database.db";
/// let gender = "Male";
/// let species = "Human";
///
/// match get_wings_by_gender_species(path, gender, species) {
///     Ok(result) => {
///         // Handle the filtered list of Extras
///         println!("Filtered Extras: {:#?}", result);
///     }
///     Err(err) => {
///         // Handle the error
///         eprintln!("Error: {:?}", err);
///     }
/// }
/// ```
pub fn get_wings_by_gender_species(
    path: &str,
    target_gender: &str,
    target_species: &str,
) -> Result<Vec<Extras>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut extras: Vec<Extras> = vec![];

    // Consulta SQL ajustada com cláusulas WHERE para filtrar por gênero e espécie
    let sql_query = format!(
        "SELECT id, name, species, gender, addr FROM extras WHERE gender = ? AND species = ?"
    );

    let mut stmt = match conn.prepare(&sql_query) {
        Ok(stm) => {stm},
        Err(e) => {
            error!("Error ocurred while preparing the statement {}, due to {:?}", &sql_query, e);
            panic!();
        },
    };

    let extra_iter = match stmt.query_map([target_gender, target_species], |row| {
            Ok(Extras {
                id: row.get(0)?,
                name: row.get(1)?,
                species: row.get(2)?,
                gender: row.get(3)?,
                addr: row.get(4)?
            })
        }) {
        Ok(mapped_rows) => {mapped_rows},
        Err(e) => {
            error!("Error iterating Extras due to {:#?}", e);
            panic!();
        },
    };

    for extra in extra_iter {
        let extra = extra.unwrap();
        let buff_extra = Extras {
            id: extra.id,
            name: extra.name,
            species: extra.species,
            gender: extra.gender,
            addr: extra.addr,
        };
        extras.push(buff_extra);
    }

    Ok(extras)
}
