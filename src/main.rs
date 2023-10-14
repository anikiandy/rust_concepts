fn main() {
    println!("Hello, world!"); 
    let booger = Boogers::new(); //new boogers
    let number = booger.do_closure(|n| n+5); // call function that takes closure
    println!("something: {}", number); //print

    let boogers = booger.inc_x(5).inc_x(3).inc_x(20); // chain the thing returning Boogers struct each time and assign last one to boogers
    let number = boogers.do_closure(|n| n+5); //do the closure thing again
    println!("something: {}", number); //print response from closure thing
}

struct Boogers{
    x: i32,
}

impl Boogers{i
    //constructor
    pub fn new() -> Boogers {
        Boogers{
            x: 10,
        }
    }

    // fn to do closure 
    // I think <F> means that the fn it takes should impliment F then F is defined in the next line as a Fn(i32) that returns an i32
    // we make this a generic type because 
    pub fn do_closure<F>(&self, closure: F) -> i32
    where F: Fn(i32) -> i32 {
        closure(self.x)
    }

    pub fn inc_x(mut self, y:i32) -> Boogers {
        self.x = self.x + y;
        self
    }
}

