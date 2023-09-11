use gtest::{Log, Program, System};
use hello_world::InputMessages;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Hello"));
    assert!(!res.main_failed());

    // test `SendHelloTo`
    let hello_recipient: u64 = 4;
    let res = program.send(2, InputMessages::SendHelloTo(hello_recipient.into()));
    let expected_log = Log::builder()
        .dest(hello_recipient)
        .payload(String::from("666Hello123"));
    assert!(res.contains(&expected_log))
}
