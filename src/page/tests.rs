use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::page_serialization::PageSerialization;
use crate::page::Page;
use crate::utils::id_generator::mock_id_gen_control;
use mockall::Sequence;
use std::rc::Rc;
use uuid::Uuid;

#[test]
fn test_serialize_and_get_id() {
    /* Create a mock of the id generator */
    let mut mock_id_gen = mock_id_gen_control();
    let page_id = Uuid::new_v4().to_hyphenated().to_string();
    let dec_id = Uuid::new_v4().to_hyphenated().to_string();

    let mut seq = Sequence::new();

    mock_id_gen
        .expect_gen_id()
        .times(1)
        .in_sequence(&mut seq)
        .return_const(dec_id.clone());

    mock_id_gen
        .expect_gen_id()
        .times(1)
        .in_sequence(&mut seq)
        .return_const(page_id.clone());

    /* Create the actual page control */
    let page = Page::new(Rc::new(Box::new(mock_id_gen)));

    assert_eq!(
        page.serialize(),
        PageSerialization::new(page_id.clone(), vec![DecSocketSer::new(dec_id, None)])
    );

    assert_eq!(page.get_id(), &page_id)
}
