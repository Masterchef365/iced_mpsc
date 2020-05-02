pub use futures::channel::mpsc::Sender;
use futures::stream::StreamExt;
use iced_futures::futures;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::time::Instant;

pub struct Mpsc<T> {
    unique: Instant,
    buf_size: usize,
    _phantom: PhantomData<T>,
}

impl<T> Mpsc<T> {
    pub fn new(buf_size: usize) -> Self {
        Self {
            unique: Instant::now(),
            buf_size,
            _phantom: PhantomData,
        }
    }

    pub fn sub(&self) -> iced::Subscription<Message<T>>
    where
        T: Debug + Send + 'static,
    {
        MpscSubscription::sub(self.buf_size, self.unique)
    }
}

pub struct MpscSubscription<T, U> {
    buf_size: usize,
    unique: U,
    _phantom: PhantomData<T>, // TODO: Remove this?
}

impl<T: Send + 'static + Debug, U: Hash + 'static> MpscSubscription<T, U> {
    pub fn sub(buf_size: usize, unique: U) -> iced::Subscription<Message<T>> {
        iced::Subscription::from_recipe(Self {
            buf_size,
            unique,
            _phantom: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message<T> {
    Sender(Sender<T>),
    Received(T),
}

impl<H, I, T, U> iced_native::subscription::Recipe<H, I> for MpscSubscription<T, U>
where
    U: Hash + 'static,
    H: std::hash::Hasher,
    T: Send + 'static,
{
    type Output = Message<T>;
    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.unique.hash(state);
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
