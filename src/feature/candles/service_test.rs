#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 15)]
    async fn test_get_all_candles() {
        let task1 = tokio::spawn(async {
            println!("test");
            println!("test")
        });
        let task2 = tokio::spawn(async {
            println!("hello test");
            println!("hello test");
        });
        let _ = tokio::join!(task1, task2);
    }
}
