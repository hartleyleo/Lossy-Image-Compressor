use csc411_image::{RgbImage, Rgb};
use csc411_arith::{index_of_chroma, chroma_of_index};
use crate::compress_decompress::Ypbpr;
use crate::compress_decompress::PixelBlockValues;

// Function takes in a vector filled with 4 pixels of component video (ypbpr) type
// then it will convert this block into our custom struct type PixelBlockValues
pub fn discrete_cosine_transfer(pixels: Vec<Ypbpr>) -> PixelBlockValues {
    
    // Use this for simplification of division for averages
    let pixel_total: f32 = pixels.len() as f32;

    // Calculate a, b, c, d
    // -----------------------------------------------------
    // Based on formula provided in assignment description:
    // a = (Y4 + Y3 + Y2 + Y1)/4.0
    // b = (Y4 + Y3 − Y2 − Y1)/4.0
    // c = (Y4 − Y3 + Y2 − Y1)/4.0
    // d = (Y4 − Y3 − Y2 + Y1)/4.0
    // If we imagine these Y's as the pixel in the 2x2 vec's y values, 
    //       ( Y1 Y2 )   as    ( [0].y [1].y )
    //       ( Y3 Y4 )   ->    ( [2].y [3].y )
    // then we can calculate as follows:
    let mut a = (pixels[3].y + pixels[2].y + pixels[1].y + pixels[0].y) / pixel_total;
    let mut b = (pixels[3].y + pixels[2].y - pixels[1].y - pixels[0].y) / pixel_total;
    let mut c = (pixels[3].y - pixels[2].y + pixels[1].y - pixels[0].y) / pixel_total;
    let mut d = (pixels[3].y - pixels[2].y - pixels[1].y + pixels[0].y) / pixel_total;


    // For b, c, d, we clamp it to be between the floating point range of -0.3 and 0.3
    a = (a * (511 as f32)).round();
    b = (b.clamp(-0.3,0.3) * (50 as f32)).round();
    c = (c.clamp(-0.3,0.3) * (50 as f32)).round();
    d = (d.clamp(-0.3,0.3) * (50 as f32)).round();

    // Calculate average pb
    let avg_pb = (pixels[0].pb + pixels[1].pb + pixels[2].pb + pixels[3].pb) / pixel_total;
    let avg_pb = index_of_chroma(avg_pb as f32);

    // Calculate average pr
    let avg_pr = (pixels[0].pr + pixels[1].pr + pixels[2].pr + pixels[3].pr) / pixel_total;
    let avg_pr = index_of_chroma(avg_pr as f32);

    return PixelBlockValues {a, b, c, d, avg_pb, avg_pr};

}

pub fn inverse_discrete_cosine_transfer(pixel: &PixelBlockValues) -> Vec<Ypbpr> {

    let mut pixels = Vec::new();

    // Calculate Y1, Y2, Y3, Y4
    // -----------------------------------------------------
    // Based on formula provided in assignment description:
    // Y1 = a − b − c + d
    // Y2 = a − b + c − d
    // Y3 = a + b − c − d
    // Y4 = a + b + c + d
    // We must first get the a, b, c, and d values:
    let a: f32 = (pixel.a as f32 / (511 as f32)).clamp(0.0,1.0);
    let b: f32 = (pixel.b as f32 / (50 as f32)).clamp(-0.3,0.3);
    let c: f32 = (pixel.c as f32 / (50 as f32)).clamp(-0.3,0.3);
    let d: f32 = (pixel.d as f32 / (50 as f32)).clamp(-0.3,0.3);

    // Then we calculate as follows: 
    let mut y_vec = Vec::new();
    y_vec.push(a - b - c + d);
    y_vec.push(a - b + c - d);
    y_vec.push(a + b - c - d);
    y_vec.push(a + b + c + d);

    let pb = chroma_of_index(pixel.avg_pb as usize);
    let pr = chroma_of_index(pixel.avg_pr as usize);

    for i in 0..y_vec.len() {
        pixels.push(Ypbpr {y: y_vec[i] as f32, pb: pb as f32, pr: pr as f32});
    }
    
    return pixels;
}