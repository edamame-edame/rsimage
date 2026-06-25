
// Struct
pub struct RGB8
{
    r: u8,
    g: u8,
    b: u8
}

pub struct RGB16
{
    r: u16,
    g: u16,
    b: u16
}


struct Hue { value: f32 }

struct Sat { value: u16 }

struct Val { value: u16 }

pub struct HSV
{
    h: Hue,
    s: Sat,
    v: Val
}

impl std::convert::From<RGB8> for RGB16
{
    fn from(source: RGB8) -> RGB16
    {
        return  RGB16
                {
                    r: (source.r as u16) << 8,
                    g: (source.g as u16) << 8,
                    b: (source.b as u16) << 8
                };
    }
}


impl std::convert::From<RGB16> for RGB8
{
    fn from(source: RGB16) -> RGB8
    {
        return  RGB8
                {
                    r: (source.r >> 8) as u8,
                    g: (source.g >> 8) as u8,
                    b: (source.b >> 8) as u8
                };
    }
}


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


impl std::ops::Add<Hue> for Hue
{
    type Output = Hue;

    fn add(self, rhs: Hue) -> Self::Output
    {
        self + rhs.value
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


impl HSV
{
    fn from_rgb16(source: RGB16) -> HSV
    {
        let max: f32 = 
            std::cmp::max(
                std::cmp::max(
                    source.r, source.g), source.b
            ) as f32;
        
        let min: f32 = 
            std::cmp::min(
                std::cmp::min(
                    source.r, source.g), source.b
            ) as f32;

        let lc1: f32;
        let lc2: f32;
        let ofs: f32;

        if max as u16 == source.r
        {
            lc1 = source.g as f32;
            lc2 = source.b as f32;
            ofs = 0f32;
        }
        else if max as u16 == source.g
        {
            lc1 = source.b as f32;
            lc2 = source.r as f32;
            ofs = 120f32;
        }
        else
        {
            lc1 = source.r as f32;
            lc2 = source.g as f32;
            ofs = 240f32;
        }

        Self
        {
            h: Hue::new((lc1 - lc2) / (max - min) + ofs),
            s: (max as u16) - (min as u16),
            v: max as u16
        }
    }

    fn to_rgb16(self) -> RGB16
    {
        ;
    }
}

