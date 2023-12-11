mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "mutex";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let expected = vec![
        Expected::FnCall {
            name: "main".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "mut_cell".into(),
            args: vec!["&core::cell::RefCell<u64>::new(1_u64)".to_owned()],
        },
        Expected::FnRet {
            name: "mut_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mut_cell".into(),
            args: vec!["&core::cell::RefCell<u64>::new(2_u64)".to_owned()],
        },
        Expected::FnRet {
            name: "mut_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mut_cell".into(),
            args: vec!["&core::cell::RefCell<u64>::new(3_u64)".to_owned()],
        },
        Expected::FnRet {
            name: "mut_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mut_rc_cell".into(),
            args: vec!["alloc::rc::Rc::new(core::cell::RefCell<u64>::new(4_u64))".to_owned()],
        },
        Expected::FnRet {
            name: "mut_rc_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mut_rc_cell".into(),
            args: vec!["alloc::rc::Rc::new(core::cell::RefCell<u64>::new(5_u64))".to_owned()],
        },
        Expected::FnRet {
            name: "mut_rc_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mut_rc_cell".into(),
            args: vec!["alloc::rc::Rc::new(core::cell::RefCell<u64>::new(6_u64))".to_owned()],
        },
        Expected::FnRet {
            name: "mut_rc_cell".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mutex".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::mutex::Mutex<u64>::new(1_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "mutex".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mutex".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::mutex::Mutex<u64>::new(2_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "mutex".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "mutex".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::mutex::Mutex<u64>::new(3_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "mutex".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "rwlock".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::rwlock::RwLock<u64>::new(1_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "rwlock".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "rwlock".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::rwlock::RwLock<u64>::new(2_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "rwlock".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "rwlock".into(),
            args: vec![
                "alloc::sync::Arc::new(std::sync::rwlock::RwLock<u64>::new(3_u64))".to_owned(),
            ],
        },
        Expected::FnRet {
            name: "rwlock".into(),
            value: "()".into(),
        },
        Expected::FnRet {
            name: "main".into(),
            value: "()".into(),
        },
    ];

    let mut events = Vec::new();
    for i in 0..expected.len() {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);
        events.push(event);
    }

    verify(testcase, events, expected);

    Ok(())
}
