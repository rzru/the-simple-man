use image::{ImageReader, Rgb};
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

const TILE_SIZE: u32 = 8;

#[proc_macro]
pub fn generate_background_map(input: TokenStream) -> TokenStream {
    let processed = parse_macro_input!(input as LitStr);

    let img = ImageReader::open(processed.value())
        .unwrap()
        .decode()
        .unwrap();
    let img_rgb8 = img.to_rgb8();

    let x = img_rgb8.width() / TILE_SIZE;
    let y = img_rgb8.height() / TILE_SIZE;

    let mut uniq_tiles: Vec<Vec<&Rgb<u8>>> = vec![];
    let mut result = vec![];
    for n in 0..y {
        let mut rows = vec![];

        for m in 0..x {
            let mut tile = vec![];

            for i in 0..TILE_SIZE {
                for r in 0..TILE_SIZE {
                    tile.push(img_rgb8.get_pixel(m * TILE_SIZE + i, n * TILE_SIZE + r))
                }
            }
            let index = uniq_tiles
                .iter()
                .position(|existing_tile| existing_tile == &tile);

            match index {
                Some(index) => rows.push(index),
                None => {
                    uniq_tiles.push(tile);
                    rows.push(uniq_tiles.len() - 1);
                }
            }
        }
        result.push(rows);
    }

    let generated = quote! {
        [#([#(#result),*]),*]
    };

    generated.into()
}
