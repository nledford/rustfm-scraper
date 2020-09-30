#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_no_args() {
        let mut cmd = Command::cargo_bin("rustfm-scraper").unwrap();
        cmd.assert().failure();
    }
}
