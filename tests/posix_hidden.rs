mod fixtures;
mod utils;

use fixtures::{server, Error, TestServer};
use rstest::rstest;

#[rstest]
#[case(server(&[] as &[&str]), true)]
#[case(server(&["--posix-hidden"]), false)]
fn hidden_get_dir(#[case] server: TestServer, #[case] exist: bool) -> Result<(), Error> {
    let resp = reqwest::blocking::get(server.url())?;
    assert_eq!(resp.status(), 200);
    let paths = utils::retrieve_index_paths(&resp.text()?);
    assert!(paths.contains("dir1/"));
    assert_eq!(paths.contains(".git/"), exist);
    Ok(())
}

#[rstest]
#[case(server(&["--allow-search"] as &[&str]), true)]
#[case(server(&["--allow-search", "--posix-hidden"]), false)]
fn hidden_search_dir(#[case] server: TestServer, #[case] exist: bool) -> Result<(), Error> {
    let resp = reqwest::blocking::get(format!("{}?q={}", server.url(), "git"))?;
    assert_eq!(resp.status(), 200);
    let paths = utils::retrieve_index_paths(&resp.text()?);
    for p in paths {
        assert_eq!(p.contains(".git"), exist);
    }
    Ok(())
}
