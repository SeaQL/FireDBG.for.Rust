use anyhow::Context;
use core::alloc::Layout;
use rustc_hash::FxHashMap;
use std::{
    fmt::Write as FmtWrite,
    fs::File,
    io::Write as IoWrite,
    process::Stdio,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static::lazy_static! {
    static ref CACHE: Mutex<FxHashMap<String, Result<Layout, ()>>> = Mutex::new(Default::default());
}

pub(crate) fn get_layout_of(typename: &str, typedef: &str) -> Result<Layout, ()> {
    let mut cache = CACHE
        .try_lock()
        .expect("There should be no concurrent access");
    if !cache.contains_key(typename) {
        match probe_layout_of(typedef) {
            Ok(layout) => {
                cache.insert(typename.to_owned(), Ok(layout));
            }
            Err(err) => {
                log::debug!("Failed to probe layout of {typename}: {err:?}");
                cache.insert(typename.to_owned(), Err(()));
            }
        }
    }
    *cache.get(typename).expect("Get inserted layout")
}

fn compile_rust_program(content: &str) -> anyhow::Result<String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = format!("/tmp/{}", timestamp);
    let src = format!("{path}.rs");
    let obj = format!("{path}.o");
    let mut file = File::create(&src).with_context(|| format!("Fail to create file: `{src}`"))?;
    file.write_all(content.as_bytes())
        .context("Fail to write")?;
    let result = std::process::Command::new("rustc")
        .arg("--cap-lints=allow")
        .arg("--edition=2021")
        .arg("-g")
        .arg(&src)
        .arg("-o")
        .arg(&obj)
        .spawn()?
        .wait_with_output()?;
    if result.status.code().context("compile")? != 0 {
        anyhow::bail!("Failed to compile {src}");
    }
    Ok(obj)
}

fn execute_rust_program(obj: &str) -> anyhow::Result<String> {
    let result = std::process::Command::new(&obj)
        .stderr(Stdio::inherit())
        .output()?;
    if result.status.code().context("run")? != 0 {
        anyhow::bail!("Failed to run {obj}");
    }
    Ok(String::from_utf8(result.stdout)?)
}

fn probe_layout_of(typedef: &str) -> anyhow::Result<Layout> {
    log::trace!("Probing {typedef} ...");
    let mut src = "extern crate alloc;\n".to_string();
    write!(src, "{}", typedef)?;
    write!(
        src,
        r#"
    fn main() {{
        let layout = core::alloc::Layout::new::<T>();
        println!("size = {{}}", layout.size());
        println!("align = {{}}", layout.align());
    }}
    "#
    )?;
    let obj = compile_rust_program(&src).with_context(|| format!("Fail to compile: `{src}`"))?;
    let result = execute_rust_program(&obj).with_context(|| format!("Fail to run: `{obj}`"))?;
    let mut lines = result.lines();
    let line = lines.next().context("has next")?;
    let (attr, size) = line.split_once(" = ").context("attr")?;
    assert_eq!(attr, "size");
    let line = lines.next().context("has next")?;
    let (attr, align) = line.split_once(" = ").context("attr")?;
    assert_eq!(attr, "align");
    let size: usize = size
        .parse()
        .with_context(|| format!("Not size: `{size}`"))?;
    let align: usize = align
        .parse()
        .with_context(|| format!("Not align: `{align}`"))?;
    let layout = Layout::from_size_align(size, align)?;
    log::trace!("{layout:?}");
    Ok(layout)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_probe() {
        let layout = probe_layout_of("type T<'a> = &'a str;").unwrap();
        assert_eq!(layout, Layout::new::<&str>());
        let layout = probe_layout_of("type T = (i32, i32);").unwrap();
        assert_eq!(layout, Layout::new::<(i32, i32)>());
    }
}
