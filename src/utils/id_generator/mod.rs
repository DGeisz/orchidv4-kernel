use mockall::*;
use std::rc::Rc;
use uuid::Uuid;

#[automock]
pub trait IdGenControl {
    fn gen_id(&self) -> String;
}

pub fn mock_id_gen_control() -> MockIdGenControl {
    MockIdGenControl::new()
}

pub struct UuidGenerator;

impl UuidGenerator {
    pub fn new() -> Rc<Box<dyn IdGenControl>> {
        Rc::new(Box::new(UuidGenerator))
    }
}

impl IdGenControl for UuidGenerator {
    fn gen_id(&self) -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}
