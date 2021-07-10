use orchidv4_kernel::utils::type_utils::Relltion;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let play = Rc::new(Play {
        p: RefCell::new(None),
    });

    // *play.p.borrow_mut() = Some(Rc::clone(&play));

    println!("Strong count: {}", Rc::strong_count(&play))
}

struct Play {
    p: Relltion<Rc<Play>>,
}

impl Drop for Play {
    fn drop(&mut self) {
        match &*self.p.borrow() {
            Some(p) => println!("Main drop: strong count on p: {}", Rc::strong_count(p)),
            None => {
                println!("P is gone");
            }
        }
    }
}
