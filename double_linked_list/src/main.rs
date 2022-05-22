use crate::List::*;

enum List {
    Cont(Box<List>, u32, Box<List>),
    Nil,
}

impl List{
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cont(Box::new(List::new()), elem, Box::new(self))
    }

    fn append(self, elem: u32) -> List {
        let mut temp = Cont( Box::new(List::new()) , elem, Box::new(List::new()) );

        temp = temp.change_tail(self);

        temp
    }

    fn change_tail(self, list: List) -> List {
        match self {
            Cont(head , elem, _) => Cont(head, elem, Box::new(list)),
            Nil => List::new()
        }
    }

    fn change_head(self, list: List) -> Box<List> {
        match self {
            Cont(_, elem, tail) => Box::new(Cont(Box::new(list), elem, tail)),
            Nil => Box::new(List::new())
        }
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
