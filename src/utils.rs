use crate::watcher::WatcherEvent;
use futures::{stream, Stream, TryStream, TryStreamExt};

/// Flattens each item in the list following the rules of `WatcherEvent::into_iter_added`
pub fn try_flatten_addeds<K, S: TryStream<Ok = WatcherEvent<K>>>(
    stream: S,
) -> impl Stream<Item = Result<K, S::Error>> {
    stream
        .map_ok(|event| stream::iter(event.into_iter_added().map(Ok)))
        .try_flatten()
}
