/**
 * Image object.
 */

use crate::types::color::color::{Color};
use crate::types::scalar::size::{Size};


#[derive(Clone, Debug)]
pub struct ImageObject<T: Color>
{
    pub(crate) data: std::vec::Vec::<T>
}


impl<T: Color> ImageObject<T>
{
    pub fn new(size: Size) -> Self
    {
        Self { data: std::vec![T::zero(); size.height * size.width] }
    }
}


pub trait ImageTrait<T: Color>
where T:  std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::AddAssign<T>
        + std::ops::SubAssign<T>
        + std::ops::MulAssign<T>
        + std::ops::DivAssign<T>
{
    fn average() -> std::vec::Vec<f64>;

    fn variance() -> std::vec::Vec<f64>;
}

