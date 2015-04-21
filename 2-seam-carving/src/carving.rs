use lodepng::RGB;

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

#[cfg(test)]
mod tests {
    use lodepng::RGB;
    use super::{calculate_energy, MAX_PIXEL_ENERGY};

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
}
