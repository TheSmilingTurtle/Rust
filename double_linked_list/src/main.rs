use crate::List::*;

enum List {
    Cont(Box<List>, u32, Box<List>),
    Nil,
}

impl List{
    fn new() -> List {
        Nil
    }

    fn append(mut self, item: u32) -> List {
        let new_item = Cont(
            Box::new(self),
            item,
            Box::new(Nil)
        );

        match self {
            Cont(.., ref mut tail) => *tail = Box::new(new_item),
            _ => {}
        }
        
        new_item
    }

    fn to_string(self) -> String {
        match self {
            Cont(head, elem, tail) => format!("{}, {}, {}", head.to_string(), elem.to_string(), tail.to_string()),
            Nil => "Nil".to_string()
        }
    }
}

fn main() {
    let mut l = List::new();

    l = l.append(3);
    l = l.append(4);

    println!("{}", l.to_string());
}
