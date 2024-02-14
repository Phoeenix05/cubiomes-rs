mod cubiomes;

use cubiomes::*;

fn main() {
    let mut g: Generator = Generator::default();
    let g_ptr = &mut g;

    unsafe { setupGenerator(g_ptr, enum_MCVersion_MC_1_20, LARGE_BIOMES) };

    let mut seed: ::std::os::raw::c_uint = 0;
    loop {
        unsafe { applySeed(g_ptr, enum_Dimension_DIM_OVERWORLD, seed.into()) };

        let scale = 1;
        let (x, y, z) = (0, 63, 0);
        let biome_id = unsafe { getBiomeAt(g_ptr, scale, x, y, z) };

        if biome_id == enum_BiomeID_lush_caves {
            println!("seed: {}", seed);
            break;
        }

        seed += 1;
    }
}
