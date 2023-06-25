use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Fencer {
    firstname: String,
    surname: Option<String>,
    nationality: Option<String>,
}
impl Fencer {
    pub fn new(name: String, surname: Option<String>, nationality: Option<String>) -> Self {
        Fencer {
            firstname: name.trim().to_string(),
            surname: surname.map(|x| x.trim().to_string()),
            nationality: nationality.map(|x| x.trim().to_string()),
        }
    }

    pub fn print(&self) -> String {
        format!(
            "\n{}\n{}\n{}",
            self.firstname,
            self.surname
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            self.nationality
                .clone()
                .unwrap_or_else(|| "Unknown".to_string())
        )
    }
}
