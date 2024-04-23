#![allow(dead_code)]
#![allow(unused_imports)]

use firedbg_rust_debugger::{
    new_async_breakpoint, new_breakpoint, DebuggerInfo, DebuggerParams, Event, FireDbgForRust,
    InfoMessage, SourceFile, ALLOCATION_STREAM, EVENT_STREAM, INFO_STREAM,
};
use pretty_assertions::assert_eq;
use sea_streamer::{
    file::FileId, Error as SeaStreamerErr, Producer, SeaConnectOptions, SeaConsumer, SeaProducer,
    SeaStreamer, StreamKey, Streamer, Timestamp,
};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::SystemTime,
};

#[derive(Debug)]
pub enum Expected {
    FnCall { name: String, args: Vec<String> },
    FnRet { name: String, value: String },
}

pub fn debugger_params_from_file(testcase: &str) -> DebuggerParams {
    let path = format!("testcases/{testcase}.rs");
    let modified = Path::new(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
        .metadata()
        .unwrap()
        .modified()
        .unwrap();
    let mut files = vec![Default::default()];
    files.push(SourceFile {
        id: 1,
        path: path.clone(),
        crate_name: testcase.into(),
        modified,
    });

    let mut breakpoints = vec![Default::default()];
    for func in firedbg_rust_parser::parse_file(&path).unwrap() {
        breakpoints.push(new_breakpoint(breakpoints.len() as u32, 1, &func));
    }

    rustc(&format!("testcases/{testcase}"));

    DebuggerParams {
        binary: format!("testcases/{testcase}.o"),
        files,
        breakpoints,
        arguments: vec![],
    }
}

pub fn debugger_params_testbench(testcase: &str) -> DebuggerParams {
    let path = format!("../testbench/{testcase}/src/bin/main.rs");
    let mut files = vec![Default::default()];
    files.push(SourceFile {
        id: 1,
        path: path.clone(),
        crate_name: testcase.into(),
        modified: SystemTime::UNIX_EPOCH,
    });

    let mut breakpoints = vec![Default::default()];
    for func in firedbg_rust_parser::parse_file(&path).unwrap() {
        breakpoints.push(new_breakpoint(breakpoints.len() as u32, 1, &func));
        if func.ty.is_async() {
            breakpoints.push(new_async_breakpoint(breakpoints.len() as u32, 1, &func));
        }
    }

    cargo_b(&format!("../testbench/{testcase}/Cargo.toml"), "main");

    DebuggerParams {
        binary: format!("../testbench/{testcase}/target/debug/main"),
        files,
        breakpoints,
        arguments: vec![],
    }
}

pub fn generate_rust_program(testcase: &str, content: &str) -> DebuggerParams {
    let path = format!("testcases/generated/{testcase}");
    let src = format!("{path}.rs");
    let mut file = std::fs::File::create(&src).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    rustc(&path);
    debugger_params_from_file(&format!("generated/{testcase}"))
}

pub async fn setup(testcase: &str) -> Result<(SeaProducer, SeaConsumer), SeaStreamerErr> {
    let (_, producer, consumer) = setup_1(testcase).await?;
    Ok((producer, consumer))
}

pub async fn setup_1(
    testcase: &str,
) -> Result<(SeaStreamer, SeaProducer, SeaConsumer), SeaStreamerErr> {
    create_env_logger();

    let file_id = temp_file(testcase).unwrap();
    println!("{file_id:?}");
    let mut options = SeaConnectOptions::default();
    options.set_file_connect_options(|options| {
        options.set_end_with_eos(true);
    });
    let streamer = SeaStreamer::connect(file_id.to_streamer_uri()?, options).await?;
    let producer = streamer.create_generic_producer(Default::default()).await?;
    let consumer = streamer
        .create_consumer(&[StreamKey::new(EVENT_STREAM)?], Default::default())
        .await?;

    producer.send_to(
        &StreamKey::new(INFO_STREAM)?,
        serde_json::to_string(&InfoMessage::Debugger(DebuggerInfo {
            debugger: FireDbgForRust,
            version: env!("CARGO_PKG_VERSION").to_owned(),
            workspace_root: "".to_owned(),
            package_name: testcase.to_owned(),
            target: testcase.to_owned(),
            arguments: vec![],
        }))
        .unwrap()
        .as_str(),
    )?;

    Ok((streamer, producer, consumer))
}

pub async fn setup_2(
    testcase: &str,
) -> Result<(SeaProducer, SeaConsumer, SeaConsumer), SeaStreamerErr> {
    let (streamer, producer, consumer_1) = setup_1(testcase).await?;
    let consumer_2 = streamer
        .create_consumer(&[StreamKey::new(ALLOCATION_STREAM)?], Default::default())
        .await?;
    Ok((producer, consumer_1, consumer_2))
}

pub fn verify(testcase: &str, events: Vec<Event>, expected: Vec<Expected>) {
    for (i, expect) in expected.into_iter().enumerate() {
        match &events[i] {
            Event::Breakpoint { .. } => unreachable!(),
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => match expect {
                Expected::FnCall { name, args } => {
                    if function_name.starts_with('<') {
                        assert_eq!(function_name, &name);
                    } else {
                        assert_eq!(function_name, &format!("{testcase}::{name}"));
                    }
                    assert_eq!(arguments.len(), args.len());
                    for (j, arg) in args.iter().enumerate() {
                        assert_wildcard(i, arg, &arguments[j].1.to_string());
                    }
                }
                e => panic!("Expected {e:?}"),
            },
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => match expect {
                Expected::FnRet { name, value } => {
                    if function_name.starts_with('<') {
                        assert_eq!(function_name, &name);
                    } else {
                        assert_eq!(function_name, &format!("{testcase}::{name}"));
                    }
                    assert_wildcard(i, &value, &return_value.to_string());
                }
                e => panic!("Expected {e:?}"),
            },
        }
    }
}

fn temp_file(name: &str) -> Result<FileId, std::io::Error> {
    let name = format!("{}-{}", name, millis_of(&Timestamp::now_utc()));
    let path = format!("/tmp/{name}.firedbg.ss");
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)?;

    Ok(FileId::new(path))
}

fn millis_of(ts: &Timestamp) -> i64 {
    (ts.unix_timestamp_nanos() / 1_000_000) as i64
}

pub fn create_env_logger() {
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}

fn assert_wildcard(i: usize, template: &str, against: &str) {
    if !firedbg_rust_debugger::typename::wildcard_match(template, against) {
        print!("[{i}] ");
        assert_eq!(against, template);
    }
}

pub fn rustc(path: &str) {
    let src = format!("{path}.rs");
    let obj = format!("{path}.o");
    let result = rustc_cmd(&src, &obj)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if result.status.code().unwrap() != 0 {
        panic!("Failed to compile {src}");
    }
}

pub fn rustc_optimize(path: &str) {
    let src = format!("{path}.rs");
    let obj = format!("{path}.o");
    let mut cmd = rustc_cmd(&src, &obj);
    cmd.arg("-O");
    let result = cmd.spawn().unwrap().wait_with_output().unwrap();
    if result.status.code().unwrap() != 0 {
        panic!("Failed to compile {src}");
    }
}

fn rustc_cmd(src: &str, obj: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("rustc");
    cmd.arg("--cap-lints=allow")
        .arg("--edition=2021")
        .arg("-g")
        .arg(&src)
        .arg("-o")
        .arg(&obj);
    cmd
}

fn cargo_b(toml: &str, bin: &str) {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("b")
        .arg("--manifest-path")
        .arg(toml)
        .arg("--bin")
        .arg(bin);
    let result = cmd.spawn().unwrap().wait_with_output().unwrap();
    if result.status.code().unwrap() != 0 {
        panic!("Failed to build {toml}");
    }
}
