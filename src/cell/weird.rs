use super::DisplayCell;

/// # WeirdCell
/// Now we're getting serious, a cell that can hold stuff: mobs (currently immobile) and water (TODO)
pub enum WeirdCell {
    Space(Option<Mob>),
    Stone,
}

impl DisplayCell for WeirdCell {
    fn foreground(&self) -> Option<u32> {
        match *self {
            WeirdCell::Stone            => Some('#' as u32),
            WeirdCell::Space(Some(mob)) => mob.foreground(),
            WeirdCell::Space(None)      => None,
        }
    }
    fn floor(&self) -> Option<u32> {
        match *self {
            WeirdCell::Stone    => Some('.' as u32),
            WeirdCell::Space(_) => None,
        }
    }
    fn color_in_background(&self) -> Option<u16> {
        match *self {
            WeirdCell::Stone    => Some(243), //medium grey
            WeirdCell::Space(_) => None,
        }
    }
    fn color_in_foreground(&self) -> u16 {
        match *self {
            WeirdCell::Stone            => 254, //light grey
            WeirdCell::Space(Some(mob)) => mob.color_in_foreground(),
            _                           => 0, //should never be used.
        }
    }
    fn occludes_background(&self) -> bool {
        match *self {
            WeirdCell::Stone    => true,
            WeirdCell::Space(_) => false,
        }
    }
}

/// # Mob
/// A mob is either a bat or an at, right now.
pub enum Mob {
    Bat,
    At,
}

impl Mob {
    fn foreground(&self) -> Option<u32> {
        match *self {
            Mob::Bat => Some('b' as u32),
            Mob::At  => Some('@' as u32),
        }
    }
    fn color_in_foreground(&self) -> u16 {
        match *self {
            Mob::Bat => 130, // medium reddish brown
            Mob::At  => 15, // white
        }
    }
}

