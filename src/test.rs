use gag::BufferRedirect;
use std::io::{stdout, Read, Write};

use crate::*;

#[macro_export]
macro_rules! assert_stdout_eq {
    ($test:expr, $expect:literal) => {
        let mut _stdout_lock = stdout().lock();
        _stdout_lock.flush().unwrap();
        let mut buffer = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buffer.read_to_string(&mut output).unwrap();
        drop(buffer);

        assert_eq!(&output[..], $expect);
    };
}

#[test]
fn test_normal_run() {
    assert_stdout_eq!(
        normal_run(
            vec!["hello world"].into_iter().map(String::from).collect(),
            vec![Flags::Echo].into_iter().collect(),
        ),
        "2\n10\n"
    );
}

#[test]
fn test_line_run() {
    assert_stdout_eq!(
        line_run(
            vec!["hello\nworld"].into_iter().map(String::from).collect(),
            vec![Flags::Echo].into_iter().collect(),
        ),
        "2\n"
    );
}
