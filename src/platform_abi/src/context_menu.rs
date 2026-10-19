use std::num::NonZeroU32;

use parking_lot::Mutex;

use crate::LogicalPoint;
use crate::app::MenuSerial;

pub struct ChannelValue {
    send: Box<dyn Fn() + Send + Sync>,
}

impl ChannelValue {
    pub fn new<T: Send + 'static>(sender: std::sync::mpsc::Sender<T>, value: T) -> ChannelValue {
        let value = Mutex::new(Some(value));
        ChannelValue {
            send: Box::new(move || {
                if let Some(value) = value.lock().take() {
                    drop(sender.send(value));
                }
            }),
        }
    }

    pub(crate) fn send(&self) {
        (self.send)();
    }
}

pub struct Item {
    pub label: String,
    pub enabled: bool,
    pub value: ChannelValue,
}

pub struct MenuBuilder {
    pub(crate) serial: MenuSerial,
    pub(crate) anchor: LogicalPoint,
    pub(crate) items: Vec<Vec<Item>>,
    pub(crate) on_closed: Vec<ChannelValue>,
    pub(crate) current: Option<usize>,
    pub(crate) width: Option<NonZeroU32>,
}

impl MenuBuilder {
    pub fn serial(&self) -> MenuSerial {
        self.serial
    }

    #[must_use]
    pub fn current(mut self, row: Option<usize>) -> MenuBuilder {
        self.current = row;
        self
    }

    #[must_use]
    pub fn width(mut self, width: Option<NonZeroU32>) -> MenuBuilder {
        self.width = width;
        self
    }

    #[must_use]
    pub fn add_items(mut self, items: Vec<Item>) -> MenuBuilder {
        self.items.push(items);
        self
    }

    #[must_use]
    pub fn add_on_closed(mut self, value: ChannelValue) -> MenuBuilder {
        self.on_closed.push(value);
        self
    }
}
