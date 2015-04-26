use lodepng::RGB;
use std::f32;

// as indicated by the spec, this is the energy of a complete standout pixel, and is also used for pixels on the edge.
pub const MAX_PIXEL_ENERGY: f32 = 255f32 * 255.0 * 3.0;

//TODO spec records energy as f32 but I don't yet see why we can't just use i32
pub fn calculate_energy(width: usize, height: usize, pixels: &[RGB<u8>]) -> Vec<f32> {
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
                (x1.r as f32 - x2.r as f32).powi(2) + (x1.g as f32 - x2.g as f32).powi(2) + (x1.b as f32 - x2.b  as f32).powi(2)
            };

            let energy_y = {
                let y1 = pixels[i - width];
                let y2 = pixels[i + width];
                (y1.r as f32 - y2.r as f32).powi(2) + (y1.g as f32 - y2.g as f32).powi(2) + (y1.b as f32 - y2.b as f32).powi(2)
            };

            (energy_x + energy_y)
        };

        pixel_energies.push(energy);
    }

    pixel_energies
}

pub fn find_seam(width: usize, pixel_energies: &Vec<f32>) -> Vec<usize> {
    // We have an implicit graph where we have:
    // - a fake source pixel which has an edge to every pixel in the first row of the image
    // - each pixel in the image has an edge to the pixel below and the pixel to the left and right of that
    //   (except if it's an edge pixel, in which case it's missing the edge to the left or right pixel)
    // - each pixel in the last row has an edge to a fake destination pixel
    let fake_src = pixel_energies.len();
    let fake_dest = pixel_energies.len() + 1;
    let vertex_count = pixel_energies.len() + 2;
    let mut dist_to = vec![f32::INFINITY; vertex_count];
    let mut prev_vertex = vec![0; vertex_count]; // records the path back in terms of vertices rather than edges (edge_to)

    // fake source pixel edges to each pixel in the first row
    for pixel in 0..width {
        dist_to[pixel] = pixel_energies[pixel];
        prev_vertex[pixel] = fake_src;
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
            if dist_to[pixel_option] > dist_to[pixel] + pixel_energies[pixel_option] {
                dist_to[pixel_option] = dist_to[pixel] + pixel_energies[pixel_option];
                prev_vertex[pixel_option] = pixel;
            }
        }
    }

    // each pixel in the image has an edge to the pixel below and the pixel to the left and right of that
    for pixel in (pixel_energies.len() - width)..pixel_energies.len() {
        if dist_to[fake_dest] > dist_to[pixel] {
            dist_to[fake_dest] = dist_to[pixel];
            prev_vertex[fake_dest] = pixel;
        }
    }

    let mut curr = fake_dest;
    let mut path = Vec::with_capacity(pixel_energies.len() / width); // capacity = the height of the image
    while curr != fake_src {
        if curr != fake_dest {
            path.push(curr);
        }
        curr = prev_vertex[curr]
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use lodepng::RGB;
    use super::{calculate_energy, find_seam, MAX_PIXEL_ENERGY};

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
            MAX_PIXEL_ENERGY, 52225.0,          MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 52024.0,          MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
        ));
    }

    #[test]
    fn finds_seam_as_given_in_example_in_spec() {
        let pixel_energies = vec!(
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 23346.0,          51304.0,          31519.0,          55112.0,          MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 47908.0,          61346.0,          35919.0,          38887.0,          MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, 31400.0,          37927.0,          14437.0,          63076.0,          MAX_PIXEL_ENERGY,
            MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY, MAX_PIXEL_ENERGY,
        );

        let seam = find_seam(6, &pixel_energies);

        // expecting a seam of MAX_PIXEL_ENERGY, 31519.0, 35919.0, 14437.0, MAX_PIXEL_ENERGY in the following pattern:
        // --  --  2   --  --  --
        // --  --  --  9   --  --
        // --  --  --  15  --  --
        // --  --  --  21  --  --
        // --  --  26  --  --  --
        assert_eq!(seam, vec!(2, 9, 15, 21, 26));
    }
}
