/**
 * Kernel process trait
 */
use crate::types::color::color::{Color};
use crate::types::scalar::point::{Rect};


pub trait KernelProcess<T: Color>
{
    fn get(lt: Rect);

    fn fetch(lt: Rect);

    fn handle() -> bool;
}

