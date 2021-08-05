use std::any::Any;
// Create trait interface to represent element
trait ComputerPart {
    fn accept(&self, computer_part_visitor: &dyn ComputerPartVisitor);
    fn as_any(&self) -> &dyn Any;
}

// Create concrete classes to use the trait interface
struct Keyboard;
struct Monitor;
struct Mouse;

impl ComputerPart for Keyboard {
    fn accept(&self, computer_part_visitor: &dyn ComputerPartVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComputerPart for Monitor {
    fn accept(&self, computer_part_visitor: &dyn ComputerPartVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComputerPart for Mouse {
    fn accept(&self, computer_part_visitor: &dyn ComputerPartVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Computer {
    parts: Vec<Box<dyn ComputerPart>>,
}

impl Computer {
    fn new() -> Self {
        Self {
            parts: vec![Box::new(Mouse), Box::new(Keyboard), Box::new(Monitor)],
        }
    }
}

impl ComputerPart for Computer {
    fn accept(&self, computer_part_display_visitor: &dyn ComputerPartVisitor) {
        for part in &self.parts {
            part.accept(computer_part_display_visitor)
        }
        computer_part_display_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Define an interface to represent visitor
trait ComputerPartVisitor {
    fn visit(&self, computer_part: &dyn ComputerPart);
}

struct ComputerPartDisplayVisitor;
impl ComputerPartVisitor for ComputerPartDisplayVisitor {
    fn visit(&self, computer_part: &dyn ComputerPart) {
        match computer_part.as_any() {
            mouse if mouse.is::<Mouse>() => println!("Displaying Mouse."),
            keyboard if keyboard.is::<Keyboard>() => println!("Displaying Keyboard."),
            monitor if monitor.is::<Monitor>() => println!("Displaying Monitor."),
            computer if computer.is::<Computer>() => println!("Displaying Computer"),
            _ => eprintln!("Nothing found..."),
        }
    }
}

struct ComputerPartSausageVisitor;
impl ComputerPartVisitor for ComputerPartSausageVisitor {
    fn visit(&self, computer_part: &dyn ComputerPart) {
        match computer_part.as_any() {
            mouse if mouse.is::<Mouse>() => println!("Displaying Mouse...Sausage!"),
            keyboard if keyboard.is::<Keyboard>() => println!("Displaying Keyboard...Sausage!"),
            monitor if monitor.is::<Monitor>() => println!("Displaying Monitor...Sausage!"),
            computer if computer.is::<Computer>() => println!("Displaying Computer...Sasausage!"),
            _ => eprintln!("Nothing found..."),
        }
    }
}

fn main() {
    let computer = Computer::new();
    computer.accept(&ComputerPartDisplayVisitor);
    computer.accept(&ComputerPartSausageVisitor);
}
