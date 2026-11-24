use crate::harness::CliSandbox;

#[tokio::test]
async fn not_logged_in_prints_and_exits() {
    let sandbox = CliSandbox::start().await;

    let outcome = sandbox.run(&[]).await;

    assert_eq!(outcome.code, Some(0), "{outcome:#?}");
    assert!(outcome.stdout.contains("not logged-in."), "{outcome:#?}");
    assert_eq!(outcome.stderr, "");
}
