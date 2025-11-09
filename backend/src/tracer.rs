pub fn setup_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let format = tracing_subscriber::fmt::format()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let file_appender = tracing_appender::rolling::daily("./logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .event_format(format)
        .with_writer(non_blocking)
        .init();

    guard
}
