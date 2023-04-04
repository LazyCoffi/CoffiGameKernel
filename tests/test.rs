use game_server_demo::data::test_data;
use game_server_demo::data::general_vec_data;

#[test]
fn hp_test() {
    let mut state = test_data::TestData::new();

    state.hp_inc();

    assert_eq!(1, state.get_hp());
}

#[test]
fn general_test() {
    let mut vec = general_vec_data::GeneralVec::new();
    vec.push_int(String::from("Hp"), 100);
    vec.push_uint(String::from("Stg"), 20);

    match vec.peek_attr_by_name(String::from("Hp")) {
        general_vec_data::GeneralType::Int(x) => assert_eq!(100i32, *x),
        general_vec_data::GeneralType::UInt(x) => assert_eq!(20u32, *x),
        _ => ()
    }
}
