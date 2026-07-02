/**
 * Angle implementation.
 */
use std::f64::consts::PI;


#[derive(Copy, Clone, Debug)]
pub struct Degree { pub value: f64 }

#[derive(Copy, Clone, Debug)]
pub struct Radian { pub value: f64 }


pub trait Angle
{
    fn new(value: f64) -> Self;

    fn min() -> f64;
    fn max() -> f64;

    fn normalize(self: Self);
}

impl Angle for Degree
{
    fn new(value: f64) -> Self
    {
        Self { value: value }
    }

    fn min() -> f64
    {   0f64}

    fn max() -> f64
    { 360f64}

    fn normalize(mut self: Self)
    {
        let interval: f64 = Self::max() - Self::min();

        while self.value < Self::min()
        {
            self.value += interval;
        }

        while self.value > Self::max()
        {
            self.value -= interval;
        }
    }
}


impl Angle for Radian
{
    fn new(value: f64) -> Self
    {
        Self { value: value }
    }

    fn min() -> f64
    {   0f64}

    fn max() -> f64
    {2f64*PI}

    fn normalize(mut self: Self)
    {
        let interval: f64 = Self::max() - Self::min();

        while self.value < Self::min()
        {
            self.value += interval;
        }

        while self.value > Self::max()
        {
            self.value -= interval;
        }
    }
}


impl std::convert::From<Radian> for Degree
{
    fn from(value: Radian) -> Self
    {
        Self {
            value: value.value
                    * (Degree::max() - Degree::min())
                    / (Radian::max() - Radian::min())
        }
    }
}


impl std::convert::From<Degree> for Radian
{
    fn from(value: Degree) -> Self
    {
        Self {
            value: value.value
                    * (Radian::max() - Radian::min())
                    / (Degree::max() - Degree::min())
        }
    }
}


impl std::ops::Add<f64> for Degree
{
    type Output = Degree;

    fn add(self, rhs: f64) -> Self::Output
    {
        Degree::new(self.value + rhs)
    }
}

impl std::ops::Add<Degree> for Degree
{
    type Output = Degree;

    fn add(self, rhs: Degree) -> Self::Output
    {
        self + rhs.value
    }
}

impl std::ops::Add<Radian> for Degree
{
    type Output = Degree;

    fn add(self, rhs: Radian) -> Self::Output
    {
        self + Degree::from(rhs)
    }
}


impl std::ops::Add<f64> for Radian
{
    type Output = Radian;

    fn add(self, rhs: f64) -> Self::Output
    {
        Radian::new(self.value + rhs)
    }
}

impl std::ops::Add<Degree> for Radian
{
    type Output = Radian;

    fn add(self, rhs: Degree) -> Self::Output
    {
        self + Radian::from(rhs)
    }
}

impl std::ops::Add<Radian> for Radian
{
    type Output = Radian;

    fn add(self, rhs: Radian) -> Self::Output
    {
        self + rhs.value
    }
}


impl std::ops::Sub<f64> for Degree
{
    type Output = Degree;

    fn sub(self, rhs: f64) -> Self::Output
    {
        Degree::new(self.value - rhs)
    }
}

impl std::ops::Sub<Degree> for Degree
{
    type Output = Degree;

    fn sub(self, rhs: Degree) -> Self::Output
    {
        self - rhs.value
    }
}

impl std::ops::Sub<Radian> for Degree
{
    type Output = Degree;

    fn sub(self, rhs: Radian) -> Self::Output
    {
        self - Degree::from(rhs)
    }
}


impl std::ops::Sub<f64> for Radian
{
    type Output = Radian;

    fn sub(self, rhs: f64) -> Self::Output
    {
        Radian::new(self.value - rhs)
    }
}

impl std::ops::Sub<Degree> for Radian
{
    type Output = Radian;

    fn sub(self, rhs: Degree) -> Self::Output
    {
        self - Radian::from(rhs)
    }
}

impl std::ops::Sub<Radian> for Radian
{
    type Output = Radian;

    fn sub(self, rhs: Radian) -> Self::Output
    {
        self - rhs.value
    }
}


impl std::ops::Mul<f64> for Degree
{
    type Output = Degree;

    fn mul(self, rhs: f64) -> Self::Output
    {
        Degree::new(self.value * rhs)
    }
}

impl std::ops::Mul<Degree> for Degree
{
    type Output = Degree;

    fn mul(self, rhs: Degree) -> Self::Output
    {
        self * rhs.value
    }
}

impl std::ops::Mul<Radian> for Degree
{
    type Output = Degree;

    fn mul(self, rhs: Radian) -> Self::Output
    {
        self * Degree::from(rhs)
    }
}


impl std::ops::Mul<f64> for Radian
{
    type Output = Radian;

    fn mul(self, rhs: f64) -> Self::Output
    {
        Radian::new(self.value * rhs)
    }
}

impl std::ops::Mul<Degree> for Radian
{
    type Output = Radian;

    fn mul(self, rhs: Degree) -> Self::Output
    {
        self * Radian::from(rhs)
    }
}

impl std::ops::Mul<Radian> for Radian
{
    type Output = Radian;

    fn mul(self, rhs: Radian) -> Self::Output
    {
        self * rhs.value
    }
}


impl std::ops::Div<f64> for Degree
{
    type Output = Degree;

    fn div(self, rhs: f64) -> Self::Output
    {
        Degree::new(self.value / rhs)
    }
}

impl std::ops::Div<Degree> for Degree
{
    type Output = Degree;

    fn div(self, rhs: Degree) -> Self::Output
    {
        self / rhs.value
    }
}

impl std::ops::Div<Radian> for Degree
{
    type Output = Degree;

    fn div(self, rhs: Radian) -> Self::Output
    {
        self / Degree::from(rhs)
    }
}


impl std::ops::Div<f64> for Radian
{
    type Output = Radian;

    fn div(self, rhs: f64) -> Self::Output
    {
        Radian::new(self.value / rhs)
    }
}

impl std::ops::Div<Degree> for Radian
{
    type Output = Radian;

    fn div(self, rhs: Degree) -> Self::Output
    {
        self / Radian::from(rhs)
    }
}

impl std::ops::Div<Radian> for Radian
{
    type Output = Radian;

    fn div(self, rhs: Radian) -> Self::Output
    {
        self / rhs.value
    }
}

