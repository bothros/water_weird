pub trait DisplayCell {
    fn foreground(&self) -> Option<u32>;
    fn floor(&self) -> Option<u32>;
    fn color_in_background(&self) -> Option<u16>;
    fn color_in_foreground(&self) -> u16;
    fn occludes_background(&self) -> bool;
}

pub mod stone;
