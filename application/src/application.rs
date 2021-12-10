use failure::Error;
use graph::{Delaunay, Voronoi};
use image::ToImage;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::iter::FromIterator;
use std::path::Path;

pub struct Application {
    seed: u64,
    number: u64,
}

impl Application {
    pub fn from_settings(settings: &impl crate::settings::Settings) -> Result<Self, Error> {
        Ok(Application {
            seed: settings.seed(),
            number: settings.number(),
        })
    }

    pub fn run(self) -> Result<(), Error> {
        log::debug!("seed: {}", self.seed);
        log::debug!("number: {}", self.number);

        let mut rng = StdRng::seed_from_u64(self.seed);
        let delaunay = loop {
            match Result::<Delaunay, _>::from_iter((0..self.number).map(|_| rng.gen())) {
                Ok(g) => break g,
                Err(e) => log::warn!("Triangulation failed ({:?}). Trying again.", e),
            }
        };
        let voronoi = Voronoi::from(&delaunay);

        log::info!("Writing to file");
        let path = Path::new("./output.png");
        (&*delaunay, &*voronoi).to_image(1024, 1024).save(path)?;

        //(&*delaunay, &*voronoi).to_image(16384, 16384).save(path)?;

        Ok(())
    }
}
