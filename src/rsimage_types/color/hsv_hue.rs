/**
 * @brief HSV::hue impl
 */

struct Hue { value: f32 }

impl Hue
{
    pub fn new(v: f32) -> Self
    {
        Self { value: Self::normalize(v) }
    }

    pub(crate) fn normalize(v: f32) -> f32
    {
        let mut x = v % 360.0;

        if x < 0.0
        {
            x += 360.0;
        }

        x
    }
}

impl std::ops::Add<f32> for Hue
{
    type Output = Hue;

    fn add(self, rhs: f32) -> Self::Output
    {
        Self::new(self.value + rhs)
    }
}

impl std::ops::Add<Hue> for Hue
{
    type Output = Hue;

    fn add(self, rhs: Hue) -> Self::Output
    {
        self + rhs.value
    }
}

impl std::ops::Sub<f32> for Hue
{
    type Output = Hue;

    fn sub(self, rhs: f32) -> Self::Output
    {
        Self::new(self.value - rhs)
    }
}

impl std::ops::Sub<Hue> for Hue
{
    type Output = Hue;

    fn sub(self, rhs: Hue) -> Self::Output
    {
        self - rhs.value
    }
}

