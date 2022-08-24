// type FnPtr = fn() -> String;
// pub trait Migration {
//     fn execute(&self) -> &str;
//     fn rollback(&self) -> &str;
// }

// pub struct Command {
//     execute: FnPtr,
//     rollback: FnPtr,
// }

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

// struct Schema {
//     commands: Vec<Command>,
// }

// impl Schema {
//     fn new() -> Self {
//         Self { commands: vec![] }
//     }

//     fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
//         self.commands.push(Command { execute, rollback })
//     }

//     fn execute(&self) -> Vec<String> {
//         self.commands.iter().map(|cmd| (cmd.execute)()).collect()
//     }

//     fn rollback(&self) -> Vec<String> {
//         self.commands
//             .iter()
//             .rev()
//             .map(|cmd| (cmd.rollback)())
//             .collect()
//     }
// }

// fn add_field() -> String {
//     "add field".to_string()
// }

// fn remove_field() -> String {
//     "remove field".to_string()
// }

trait Command {
    fn execute(&self);
}

struct MacroCommand {
    stack: Vec<Box<dyn Command>>,
}

impl MacroCommand {
    fn new() -> MacroCommand {
        MacroCommand { stack: Vec::new() }
    }

    fn append(&mut self, cmd: Box<dyn Command>) {
        self.stack.push(cmd);
    }

    fn undo(&mut self) {
        self.stack.pop();
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

impl Command for MacroCommand {
    fn execute(&self) {
        for command in &self.stack {
            command.execute();
        }
    }
}

struct DrawCommand {
    drawable: Box<dyn Drawable>,
    x: u32,
    y: u32,
}

impl DrawCommand {
    fn new(drawable: Box<dyn Drawable>, x: u32, y: u32) -> DrawCommand {
        DrawCommand { drawable, x, y }
    }
}

impl Command for DrawCommand {
    fn execute(&self) {
        self.drawable.draw(self.x, self.y)
    }
}

trait Drawable {
    fn draw(&self, x: u32, y: u32);
}

#[derive(Clone)]
struct DrawCanvas {}

impl DrawCanvas {
    fn new() -> DrawCanvas {
        DrawCanvas {}
    }
}

impl Drawable for DrawCanvas {
    fn draw(&self, x: u32, y: u32) {
        println!("draw(x: {}, y: {})", x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_migration() {
        // let mut schema = Schema::new();

        // // let cmd = Box::new(CreateTable);
        // schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
        // // let cmd = Box::new(AddField);
        // schema.add_migration(add_field, remove_field);

        // assert_eq!(vec!["create table", "add field"], schema.execute());
        // assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }

    #[test]
    fn test_draw() {
        let mut history = MacroCommand::new();
        let canvas = Box::new(DrawCanvas::new());

        let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
        let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));

        history.append(cmd1);
        history.append(cmd2);

        println!("-------------");
        history.execute();
        println!();

        println!("-----undo----");
        history.undo();
        history.execute();

        println!("-----clear---");
        history.clear();
        history.execute();
    }
}
