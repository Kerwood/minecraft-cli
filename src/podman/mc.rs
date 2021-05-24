#[derive(Debug)]
pub struct Container {
    name: String,
    port: String,
    status: String,
    created: String,
    level_type: String,
    game_mode: String,
}

impl Container {
    pub fn new(container: Vec<&str>) -> Self {
        Container {
            name: String::from(container[0]),
            port: String::from(container[1]),
            status: String::from(container[2]),
            created: String::from(container[3]),
            level_type: capitalize_first(container[4]),
            game_mode: capitalize_first(container[5]),
        }
    }

    pub fn get(&self, property: &str) -> &str {
        match property {
            "name" => &self.name,
            "port" => &self.port,
            "status" => &self.status,
            "created" => &self.created,
            "level_type" => &self.level_type,
            "game_mode" => &self.game_mode,
            _ => "",
        }
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(chars)
        .collect()
}
