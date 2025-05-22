struct RGB{
    r:u32,
    g:u32,
    b:u32
}
impl RGB{
    fn from_3u8(r:u32,g:u32,b:u32) -> RGB{
        RGB{
            r,g,b
        }
    }
    fn from_3percent(a:f32,b:f32,c:f32) -> RGB{
        RGB{
            r:(255.0 * a) as u32,
            g:(255.0 * b) as u32,
            b:(255.0 * c) as u32
        }
    }
}


fn main() {
    let szary1 = RGB::from_3u8(127, 127, 127);
}
