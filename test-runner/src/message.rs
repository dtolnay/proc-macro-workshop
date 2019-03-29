use termcolor::Color::*;

use super::{Expected, Test};
use crate::banner;
use crate::error::Error;
use crate::normalize;
use crate::term;

use std::path::Path;
use std::process::Output;

pub(crate) fn prepare_fail(err: Error) {
    if err.already_printed() {
        return;
    }

    term::bold_color(Red);
    print!("ERROR");
    term::reset();
    println!(": {}", err);
    println!();
}

pub(crate) fn test_fail(err: Error) {
    if err.already_printed() {
        return;
    }

    term::bold_color(Red);
    println!("error");
    term::color(Red);
    println!("{}", err);
    term::reset();
    println!();
}

pub(crate) fn no_tests_enabled() {
    term::color(Yellow);
    println!("There are no tests enabled yet.");
    println!();
    println!("Open tests/progress.rs and uncomment to turn on some of the tests.");
    term::reset();
}

pub(crate) fn nice() {
    term::color(Green);
    println!("nice!");
    term::reset();
}

pub(crate) fn begin_test(test: &Test) {
    let display_name = test
        .path
        .file_name()
        .unwrap_or_else(|| test.path.as_os_str())
        .to_string_lossy();

    let expected = match test.expected {
        Expected::Pass => "should pass",
        Expected::CompileFail => "should fail to compile",
    };

    print!("Testing ");
    term::bold();
    print!("{}", display_name);
    term::reset();
    print!(" [{}] ... ", expected);
}

pub(crate) fn failed_to_build(stderr: String) {
    term::bold_color(Red);
    println!("error");
    term::color(Red);
    banner::dotted();
    print!("{}", stderr);
    banner::dotted();
    term::reset();
    println!();
}

pub(crate) fn should_not_have_compiled() {
    term::bold_color(Red);
    println!("error");
    term::color(Red);
    println!("Expected test case to fail to compile, but it succeeded.");
    term::reset();
    println!();
}

pub(crate) fn write_stderr(wip_path: &Path, stderr_path: &Path, stderr: &str) {
    let wip_path = wip_path.to_string_lossy();
    let stderr_path = stderr_path.to_string_lossy();

    term::bold_color(Yellow);
    println!("wip");
    println!();
    print!("NOTE");
    term::reset();
    println!(": writing the following output to `{}`.", wip_path);
    println!(
        "Move this file to `{}` to accept it as correct.",
        stderr_path,
    );
    term::color(Yellow);
    banner::dotted();
    print!("{}", stderr);
    banner::dotted();
    term::reset();
    println!();
}

pub(crate) fn mismatch(expected: &str, actual: &str) {
    term::bold_color(Red);
    println!("mismatch");
    term::reset();
    println!();
    term::bold_color(Blue);
    println!("EXPECTED:");
    term::color(Blue);
    banner::dotted();
    print!("{}", expected);
    banner::dotted();
    term::reset();
    println!();
    term::bold_color(Red);
    println!("ACTUAL OUTPUT:");
    term::color(Red);
    banner::dotted();
    print!("{}", actual);
    banner::dotted();
    term::reset();
    println!();
}

pub(crate) fn output(warnings: String, output: &Output) {
    let success = output.status.success();
    let stdout = normalize::trim(&output.stdout);
    let stderr = normalize::trim(&output.stderr);
    let has_output = !stdout.is_empty() || !stderr.is_empty();

    if success {
        nice();
        if has_output || !warnings.is_empty() {
            println!();
        }
    } else {
        term::bold_color(Red);
        println!("error");
        term::color(Red);
        if has_output {
            println!("Test case failed at runtime.");
        } else {
            println!("Execution of the test case was unsuccessful but there was no output.");
        }
        term::reset();
        println!();
    }

    self::warnings(warnings);

    let color = if success { Yellow } else { Red };

    for (name, content) in &[("STDOUT", stdout), ("STDERR", stderr)] {
        if !content.is_empty() {
            term::bold_color(color);
            println!("{}:", name);
            term::color(color);
            banner::dotted();
            print!("{}", normalize::trim(content));
            banner::dotted();
            term::reset();
            println!();
        }
    }
}

pub(crate) fn warnings(warnings: String) {
    if warnings.is_empty() {
        return;
    }

    term::bold_color(Yellow);
    println!("WARNINGS:");
    term::color(Yellow);
    banner::dotted();
    print!("{}", warnings);
    banner::dotted();
    term::reset();
    println!();
}
