enum ComputerPart {
    Keyboard,
    Monitor,
    Mouse,
    Computer(Parts),
}

impl ComputerPart {
    fn accept(&self, computer_part_visitor: &ComputerPartVisitor) {
        match self {
            ComputerPart::Keyboard | ComputerPart::Monitor | ComputerPart::Mouse => {
                computer_part_visitor.visit(self)
            }
            ComputerPart::Computer(pts) => {
                for part in &pts.parts {
                    part.accept(computer_part_visitor)
                }
                computer_part_visitor.visit(self);
            }
        }
    }
}

struct Parts {
    parts: Vec<ComputerPart>,
}

impl Parts {
    fn new() -> Self {
        use ComputerPart::*;
        Self {
            parts: vec![Mouse, Keyboard, Monitor],
        }
    }
}

enum ComputerPartVisitor {
    ComputerPartDisplayVisitor,
    ComputerPartShutdownVisitor,
}

impl ComputerPartVisitor {
    fn visit(&self, computer_part: &ComputerPart) {
        match self {
	    ComputerPartVisitor::ComputerPartDisplayVisitor => self.display(computer_part),
	    ComputerPartVisitor::ComputerPartShutdownVisitor => self.shutdown(computer_part),
	}
    }
    fn display(&self, computer_part: &ComputerPart) {
	use ComputerPart::*;
	match computer_part {
	    Mouse => println!("Displaying Mouse...!"),
	    Keyboard => println!("Displaying Keyboard...!"),
	    Monitor => println!("Displaying Monitor...!"),
	    Computer(_) => println!("Displaying Computer...!"),
	}
    }
    fn shutdown(&self, computer_part: &ComputerPart) {
	use ComputerPart::*;
	match computer_part {
	    Mouse => println!("Shutting down Mouse...!"),
	    Keyboard => println!("Shutting down Keyboard...!"),
	    Monitor => println!("Shutting down Monitor...!"),
	    Computer(_) => println!("Shutting down Computer...!"),
	}
	
    }
}

fn main() {
    use ComputerPartVisitor::*;
    let computer = ComputerPart::Computer(Parts::new());
    computer.accept(&ComputerPartDisplayVisitor);
    computer.accept(&ComputerPartShutdownVisitor);
}
