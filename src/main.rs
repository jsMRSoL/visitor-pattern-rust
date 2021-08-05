use std::any::Any;
// Create trait interface to represent element
trait ComputerPart {
    fn accept(&self, computer_part_visitor: &ComputerPartDisplayVisitor);
    fn as_any(&self) -> &dyn Any;
}

// Create concrete classes to use the trait interface
struct Keyboard;
struct Monitor;
struct Mouse;

impl ComputerPart for Keyboard {
    fn accept(&self, computer_part_visitor: &ComputerPartDisplayVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl ComputerPart for Monitor {
    fn accept(&self, computer_part_visitor: &ComputerPartDisplayVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl ComputerPart for Mouse {
    fn accept(&self, computer_part_visitor: &ComputerPartDisplayVisitor) {
        computer_part_visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any { self }
}

struct Computer {
    parts: Vec<Box<dyn ComputerPart>>,
}

impl Computer {
    fn new() -> Self {
        Self { parts: vec![Box::new(Mouse), Box::new(Keyboard), Box::new(Monitor)] }
    }
}

impl ComputerPart for Computer {
    fn accept(&self, computer_part_display_visitor: &ComputerPartDisplayVisitor) {
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
    fn visit<T: ComputerPart>(&self, computer_part: &T);
}

struct ComputerPartDisplayVisitor;
impl ComputerPartVisitor for ComputerPartDisplayVisitor {
    fn visit<T: ComputerPart>(&self, computer_part: &T) {
	match computer_part.as_any() {
	    mouse if mouse.is::<Mouse>() => println!("Displaying Mouse."),
	    keyboard if keyboard.is::<Keyboard>() => println!("Displaying Keyboard."),
	    monitor if monitor.is::<Monitor>() => println!("Displaying Monitor."),
	    computer if computer.is::<Computer>() => println!("Displaying Computer"),
	    _ => eprintln!("Nothing found..."),
	}
    }
}

fn main() {
    let computer = Computer::new();
    computer.accept(&ComputerPartDisplayVisitor);
}
