/**
 * RGB color implementation
 */

use crate::types::color::color::{Color};


#[derive(Copy, Clone, Debug)]
pub struct RGB
{
    pub r: u16,
    pub g: u16,
    pub b: u16
}


impl Color for RGB
{
    fn zero() -> Self
    {
        Self { r: 0u16, g: 0u16, b: 0u16 }
    }
}


impl RGB
{
    pub fn new(r: u16, g: u16, b: u16) -> Self
    {
        Self { r: r, g: g, b: b }
    }
}

