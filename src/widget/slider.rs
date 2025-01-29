use crate::{style, widget::Slider};

use std::ops::RangeInclusive;

pub fn standard<'a, T, F, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: F,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    F: 'a + Fn(T) -> Message,
    Message: Clone,
{
    Slider::new(range, value, on_change).style(style::slider::default)
}
