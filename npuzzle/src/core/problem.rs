#[derive(Clone, Debug)]
pub struct Problem {
    table: Vec<Vec<u8>>,
}

impl Problem {
    pub fn new(table: Vec<Vec<u8>>) -> Problem {
        Problem { table }
    }

    pub fn get(&self) -> Vec<Vec<u8>> {
        self.table.clone()
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let mut table = Vec::new();
        let mut empty_count = 0;

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let row: Vec<u8> = line
                .split_whitespace()
                .map(|x| {
                    x.parse::<u8>()
                        .map_err(|_| format!("Invalid value in table: {}", x))
                })
                .collect::<Result<Vec<u8>, String>>()?;

            if row.is_empty() {
                return Err("Pusta linia w pliku".to_string());
            }

            empty_count += row.iter().filter(|&&x| x == 0).count();
            table.push(row);
        }

        if empty_count != 1 {
            return Err("Nieprawidłowa liczba pustych pól".to_string());
        }

        if table.is_empty() {
            return Err("Pusty plik".to_string());
        }

        let rows = table.len();
        let cols = table[0].len();
        if rows == 0 || cols == 0 {
            return Err("Nieprawidłowy format pliku".to_string());
        }

        for row in &table {
            if row.len() != cols {
                return Err("Nierówne wiersze w pliku".to_string());
            }
        }

        Ok(Problem::new(table))
    }
}
