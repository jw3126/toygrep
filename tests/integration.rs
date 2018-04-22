extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;
    use std::path::{Path,PathBuf};

    #[test]
    fn no_arguments() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr().contains("<PATTERN>")
            .and()
            .stderr().contains("<FILE>")
            .unwrap();
    }

    #[test]
    fn non_existing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["pattern"])
            .fails().unwrap();
    }

    fn asset_path()->PathBuf {
        let filepath = Path::new(file!())
            .parent().unwrap()
            .join("assets")
            .canonicalize().unwrap();
        return filepath
    }

    #[test]
    fn numbers() {
        let path = asset_path().join("numbers.txt");
        let spath = path.to_str().unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&["one", &spath])
            .stdout().contains("1")
            .and()
            .stdout().contains("one")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["two", spath])
            .stdout().contains("2")
            .and().stdout().contains("two")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["three", spath])
            .stdout().contains("3")
            .and()
            .stdout().contains("three")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["four", spath])
            .stdout().is("4 four four four")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["five", spath, "-i"])
            .stdout().is("5 FiVe")
            .unwrap();
    }
}
