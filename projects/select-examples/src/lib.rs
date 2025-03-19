#[cfg(test)]
mod tests {
    #[tokio::test]
    #[should_panic(expected = "all branches are disabled and there is no else branch")]
    #[tracing_test::traced_test]
    async fn select_nothing() {
        let _nonono = tokio::select! {
            Some(n) = async { None } => { n },
        };
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn select_nothing_break() {
        let zero = tokio::select! {
            Some(n) = async { None } => { n },
            else => 0,
        };
        assert_eq!(zero, 0);
    }
}
