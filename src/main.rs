use std::env;

const PATH_LIST_SEPARATOR: &str = ";";

fn list_path(path: &str) -> Vec<&str> {
    path.split(PATH_LIST_SEPARATOR)
        .filter(|&p| p != "")
        .collect()
}

fn main() {
    match env::var("PATH") {
        Ok(path) => list_path(&path).iter().for_each(|p| println!("{}", p)),
        Err(reason) => println!("{}", reason),
    }
}

#[test]
fn test_list_path_returns_none() {
    let expected: Vec<&str> = vec![];
    let actual = list_path("");
    assert_eq!(expected, actual);
}

#[test]
fn test_list_path_returns_one() {
    let expected = vec!["c:\\tmp"];
    let actual = list_path("c:\\tmp");
    assert_eq!(expected, actual);
}

#[test]
fn test_list_path_returns_two() {
    let expected = vec!["c:\\tmp", "c:\\home"];
    let actual = list_path("c:\\tmp;c:\\home");
    assert_eq!(expected, actual);
}

#[test]
fn test_list_path_returns_two_ignore_empty() {
    let expected = vec!["c:\\tmp", "c:\\home"];
    let actual = list_path("c:\\tmp;;c:\\home");
    assert_eq!(expected, actual);
}
