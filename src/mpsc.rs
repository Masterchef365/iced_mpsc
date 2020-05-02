use futures::channel::mpsc::Receiver;
pub use futures::channel::mpsc::Sender;
use futures::stream::StreamExt;
use iced_futures::futures;
use std::marker::PhantomData;

pub fn channel<T: Send + 'static + Debug>(buf_size: usize, idx: u32) -> iced::Subscription<Message<T>> {
    iced::Subscription::from_recipe(MpscChannel::new(buf_size, idx))
}

pub struct MpscChannel<T> {
    buf_size: usize,
    idx: u32,
    _phantom: PhantomData<T>
}

impl<T> MpscChannel<T> {
    pub fn new(buf_size: usize, idx: u32) -> Self {
        Self { buf_size, idx, _phantom: PhantomData }
    }
}

#[derive(Debug, Clone)]
pub enum Message<T> {
    Sender(Sender<T>),
    Received(T),
}

use std::fmt::Debug;
impl<H, I, T> iced_native::subscription::Recipe<H, I> for MpscChannel<T>
where
    H: std::hash::Hasher,
    T: Send + 'static + Debug,
{
    type Output = Message<T>;
    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.idx.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let (tx, rx) = futures::channel::mpsc::channel(self.buf_size);
        Box::pin(
            futures::stream::once(async move { Message::Sender(tx.clone()) })
                .chain(rx.map(Message::Received)),
        )
    }
}
