type FnPtr = fn() -> String;
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

// pub struct CreateTable;
// impl Migration for CreateTable {
//     fn execute(&self) -> &str {
//         "create table"
//     }

//     fn rollback(&self) -> &str {
//         "drop table"
//     }
// }

// pub struct AddField;
// impl Migration for AddField {
//     fn execute(&self) -> &str {
//         "add field"
//     }

//     fn rollback(&self) -> &str {
//         "remove field"
//     }
// }

struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback })
    }

    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_migration() {
        let mut schema = Schema::new();

        // let cmd = Box::new(CreateTable);
        schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
        // let cmd = Box::new(AddField);
        schema.add_migration(add_field, remove_field);

        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
