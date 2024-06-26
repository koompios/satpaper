use std::path::PathBuf;

use fimg::Image;

#[derive(Debug, Clone)]
pub struct Config {
    /// The satellite to source imagery from.
    ///
    /// Options include:
    ///
    /// - GOES East (covers most of North and South America)
    ///
    /// - GOES West (Pacific Ocean and parts of the western US)
    ///
    /// - Himawari (Oceania and East Asia)
    ///
    /// - Meteosat 9 (Africa, Middle East, India, Central Asia)
    ///
    /// - Meteosat 10 (Atlantic Ocean, Africa, Europe)
    pub satellite: Satellite,
    /// The X resolution/width of the generated wallpaper.
    pub resolution_x: u32,
    /// The Y resolution/height of the generated wallpaper.
    pub resolution_y: u32,
    /// The size of the "disk" (Earth) relative to the generated wallpaper's
    /// smaller dimension.
    ///
    /// Values in the 90-95 range are the best if you want maximum detail.
    pub disk_size: u32,
    /// Where generated wallpapers should be saved.
    ///
    /// Satpaper will output to a file called "satpaper_latest.png" at this path.
    pub target_path: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum Satellite {
    GOESEast,
    GOESWest,
    Himawari,
    Meteosat9,
    Meteosat10,
}

impl Config {
    pub fn disk(&self) -> u32 {
        let smaller_dim = self.resolution_x.min(self.resolution_y);

        let disk_dim = smaller_dim as f32 * (self.disk_size as f32 / 100.0);
        disk_dim.floor() as u32
    }
}

impl Satellite {
    pub fn id(self) -> &'static str {
        use Satellite::*;

        match self {
            GOESEast => "goes-16",
            GOESWest => "goes-18",
            Himawari => "himawari",
            Meteosat9 => "meteosat-9",
            Meteosat10 => "meteosat-0deg",
        }
    }

    pub fn max_zoom(self) -> u32 {
        use Satellite::*;

        match self {
            GOESEast | GOESWest | Himawari => 4,
            Meteosat9 | Meteosat10 => 3,
        }
    }

    pub fn image(self) -> Image<Box<[u8]>, 3> {
        Image::alloc(
            self.tile_count() * self.tile_size(),
            self.tile_count() * self.tile_size(),
        )
        .boxed()
    }

    pub fn tile_image(self) -> Image<Box<[u8]>, 3> {
        Image::alloc(self.tile_size(), self.tile_size()).boxed()
    }

    pub fn tile_count(self) -> u32 {
        use Satellite::*;

        match self {
            GOESEast | GOESWest | Himawari => 16,
            Meteosat9 | Meteosat10 => 8,
        }
    }

    pub fn tile_size(self) -> u32 {
        use Satellite::*;

        match self {
            GOESEast | GOESWest => 678,
            Himawari => 688,
            Meteosat9 | Meteosat10 => 464,
        }
    }
}
