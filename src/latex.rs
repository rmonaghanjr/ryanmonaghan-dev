use std::process::Stdio;
use std::io::Write;

pub fn render_latex(tex: &String) -> String {
    // use pandoc to render latex
    let mut cmd = std::process::Command::new("pandoc");
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-f")
        .arg("latex")
        .arg("-t")
        .arg("html")
        .spawn()
        .expect("failed to execute process");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(tex.as_bytes())
        .unwrap();
    
    child.stdin.as_mut().unwrap().flush().unwrap();
    
    let output = child.wait_with_output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
