use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_stream::StreamExt;
use crate::data;

pub async fn stream_logs() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let last_len: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let stream =
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(2)))
            .map(move |_| {
                let lines = data::read_daemon_log_lines(500);
                let current_len = lines.len();
                let mut prev = last_len.lock().unwrap();
                if current_len > *prev {
                    let new_lines = lines[*prev..].to_vec();
                    *prev = current_len;
                    let payload = new_lines.join("\n");
                    Ok(Event::default().data(payload))
                } else {
                    *prev = current_len;
                    Ok(Event::default().comment("heartbeat"))
                }
            });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
