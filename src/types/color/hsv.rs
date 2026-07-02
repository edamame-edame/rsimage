/**
 * HSV color implementation.
 */

use crate::types::scalar::angle::{Angle, Degree};
use crate::types::color::color::{Color};
use crate::types::color::rgb::{RGB};


#[derive(Copy, Clone, Debug)]
pub struct HSV
{
    pub hue: Degree,
    pub sat: f64,
    pub val: f64
}


impl HSV
{
    pub fn new(h: Degree, s: f64, v: f64) -> Self
    {
        Self { hue: h, sat: s, val: v }
    }
}


impl Color for HSV
{
    fn zero() -> Self 
    {
        Self { hue: Degree::new(0f64), sat: 0f64, val: 0f64 }
    }
}


impl std::convert::From<RGB> for HSV
{
    fn from(value: RGB) -> Self
    {
        let max: f64 = std::cmp::max(
            std::cmp::max(value.r, value.g),
            value.b
        ) as f64;

        let min: f64 = std::cmp::min(
            std::cmp::min(value.r, value.g),
            value.b
        ) as f64;

        HSV::new(
            if max as u16 == min as u16  {
                Degree::new(0f64)
            }
            else if max as u16 == value.r {
                Degree::new(60f64 * ((value.g - value.b) as f64) / (max - min))
            }
            else if max as u16 == value.g {
                Degree::new(60f64 * ((value.b - value.r) as f64) / (max - min)) + 120f64
            }
            else {
                Degree::new(60f64 * ((value.r - value.g) as f64) / (max - min)) + 240f64
            },
            (max - min) / max,
            max
        )
    }
}

