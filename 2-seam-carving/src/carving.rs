use lodepng::RGB;
use std::i32;

// as indicated by the spec, this is the energy of a complete standout pixel, and is also used for pixels on the edge.
pub const MAX_PIXEL_ENERGY: i32 = 255 * 255 * 3;

pub fn calculate_energy(width: usize, height: usize, pixels: &[RGB<u8>]) -> Vec<i32> {
    let mut pixel_energies = Vec::with_capacity(width * height);
    for i in 0 .. (width * height) {
        let energy = if i < width { // first row
            MAX_PIXEL_ENERGY
        } else if i > width * (height - 1) { // last row
            MAX_PIXEL_ENERGY
        } else if i % width == 0 { // first column
            MAX_PIXEL_ENERGY
        } else if (i + 1) % width == 0 { // last column
            MAX_PIXEL_ENERGY
        } else {
            let energy_x = {
                let x1 = pixels[i - 1];
                let x2 = pixels[i + 1];
                (x1.r as i32 - x2.r as i32).pow(2) + (x1.g as i32 - x2.g as i32).pow(2) + (x1.b as i32 - x2.b  as i32).pow(2)
            };

            let energy_y = {
                let y1 = pixels[i - width];
                let y2 = pixels[i + width];
                (y1.r as i32 - y2.r as i32).pow(2) + (y1.g as i32 - y2.g as i32).pow(2) + (y1.b as i32 - y2.b as i32).pow(2)
            };

            (energy_x + energy_y)
        };

        pixel_energies.push(energy);
    }

    pixel_energies
}

/// To avoid repeated allocations, 1 carver can be created and reused indefinitely for the same image.
pub struct Carver {
    dist_to: Vec<i32>, // should be ok recording distances as i32 as long as path is less than 20,000 pixels long
    prev_vertex: Vec<usize>, // records the path back in terms of vertices rather than edges (edge_to)
}

impl Carver {
    pub fn new(num_pixels: usize) -> Carver {
        // We have an implicit graph where we have:
        // - a fake source pixel which has an edge to every pixel in the first row of the image
        // - each pixel in the image has an edge to the pixel below and the pixel to the left and right of that
        //   (except if it's an edge pixel, in which case it's missing the edge to the left or right pixel)
        // - each pixel in the last row has an edge to a fake destination pixel
        let vertex_count = num_pixels + 2;
        Carver {
            dist_to: vec![i32::max_value(); vertex_count],
            prev_vertex: vec![0; vertex_count],
        }
    }

    pub fn find_seam(&mut self, width: usize, pixel_energies: &Vec<i32>) -> Vec<usize> {
        let fake_src = pixel_energies.len();
        let fake_dest = pixel_energies.len() + 1;

        for i in 0..(pixel_energies.len() + 2) {
            self.dist_to[i] = i32::max_value();
            self.prev_vertex[i] = 0;
        }

        // fake source pixel edges to each pixel in the first row
        for pixel in 0..width {
            self.dist_to[pixel] = pixel_energies[pixel];
            self.prev_vertex[pixel] = fake_src;
        }

        // each pixel in the image has an edge to the pixel below and the pixel to the left and right of that
        for pixel in 0..(pixel_energies.len() - width) {
            let next_pixel_options = if pixel % width == 0 { // first column
                vec![pixel + width, pixel + width + 1]
            } else if (pixel + 1) % width == 0 { // last column
                vec![pixel + width - 1, pixel + width]
            } else {
                vec![pixel + width - 1, pixel + width, pixel + width + 1]
            };

            for pixel_option in next_pixel_options {
                if self.dist_to[pixel_option] > self.dist_to[pixel] + pixel_energies[pixel_option] {
                    self.dist_to[pixel_option] = self.dist_to[pixel] + pixel_energies[pixel_option];
                    self.prev_vertex[pixel_option] = pixel;
                }
            }
        }

        // each pixel in the image has an edge to the pixel below and the pixel to the left and right of that
        for pixel in (pixel_energies.len() - width)..pixel_energies.len() {
            if self.dist_to[fake_dest] > self.dist_to[pixel] {
                self.dist_to[fake_dest] = self.dist_to[pixel];
                self.prev_vertex[fake_dest] = pixel;
            }
        }

        let mut curr = fake_dest;
        let mut path = Vec::with_capacity(pixel_energies.len() / width); // capacity = the height of the image
        while curr != fake_src {
            if curr != fake_dest {
                path.push(curr);
            }
            curr = self.prev_vertex[curr]
        }
        path.reverse();
        path
    }
}


#[cfg(test)]
mod tests {
    use lodepng::RGB;
    use super::{calculate_energy, Carver, MAX_PIXEL_ENERGY};

    fn rgb(r: u8, g: u8, b: u8) -> RGB<u8> {
        RGB { r: r, g: g, b: b }
    }

    #[test]
    fn calculates_energy_as_given_in_example_in_spec() {
        let pixel_energies = calculate_energy(3, 4, &vec!(
            rgb(255, 101, 51), rgb(255, 101, 153), rgb(255, 101, 255),
            rgb(255, 153, 51), rgb(255, 153, 153), rgb(255, 153, 255),
            rgb(255, 203, 51), rgb(255, 204, 153), rgb(255, 205, 255),
            rgb(255, 255, 51), rgb(255, 255, 153), rgb(255, 255, 255),
        )[..]);

        assert_eq!(pixel_energies, vec!(
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 52225,            MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 52024,            MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
        ));
    }

    #[test]
    fn finds_seam_as_given_in_example_in_spec() {
        let pixel_energies = vec!(
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 23346,            51304,            31519,            55112,            MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 47908,            61346,            35919,            38887,            MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 31400,            37927,            14437,            63076,            MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
        );

        let seam = Carver::new(pixel_energies.len()).find_seam(6, &pixel_energies);

        // expecting a seam of MAX_PIXEL_ENERGY, 31519, 35919, 14437, MAX_PIXEL_ENERGY in the following pattern:
        // --  --  2   --  --  --
        // --  --  --  9   --  --
        // --  --  --  15  --  --
        // --  --  --  21  --  --
        // --  --  26  --  --  --
        assert_eq!(seam, vec!(2, 9, 15, 21, 26));
    }
}
