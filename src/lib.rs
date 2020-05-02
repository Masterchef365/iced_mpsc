pub use futures::channel::mpsc::Sender;
use futures::stream::StreamExt;
use iced_futures::futures;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::time::Instant;

/// Mpsc channel subscription manager
pub struct Mpsc<T> {
    unique: Instant,
    buf_size: usize,
    _phantom: PhantomData<T>,
}

impl<T> Mpsc<T> {
    /// Create a new channel, with specified buffer size.
    pub fn new(buf_size: usize) -> Self {
        Self {
            // TODO: Find a more reliably unique value
            unique: Instant::now(),
            buf_size,
            _phantom: PhantomData,
        }
    }

    /// Output a subscription to this channel to allow `Message`s to be fed into the app
    pub fn sub(&self) -> iced::Subscription<Message<T>>
    where
        T: Debug + Send + 'static,
    {
        MpscSubscription::sub(self.buf_size, self.unique)
    }
}

/// MPSC message carrying either received message or a sender
#[derive(Debug, Clone)]
pub enum Message<T> {
    /// A new channel has been created, and this is the handle to it.
    Sender(Sender<T>),
    /// A value has been read from the channel.
    Received(T),
}

/// A subscription to an MPSC channel
pub struct MpscSubscription<T, U> {
    buf_size: usize,
    unique: U,
    _phantom: PhantomData<T>, // TODO: Remove this?
}

impl<T: Send + 'static + Debug, U: Hash + 'static> MpscSubscription<T, U> {
    /// Create a subscription, which will only create a new channel if `unique` is unique to the
    /// iced app it is passed to.
    pub fn sub(buf_size: usize, unique: U) -> iced::Subscription<Message<T>> {
        iced::Subscription::from_recipe(Self {
            buf_size,
            unique,
            _phantom: PhantomData,
        })
    }
}

impl<H, I, T, U> iced_native::subscription::Recipe<H, I> for MpscSubscription<T, U>
where
    U: Hash + 'static,
    H: std::hash::Hasher,
    T: Send + 'static,
{
    type Output = Message<T>;

    fn hash(&self, state: &mut H) {
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
