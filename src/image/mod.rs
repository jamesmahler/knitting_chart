//! The image cache

use knitting_parse::Stitch;
use knitting_parse::Side;

use png::Decoder;

use std::error::Error;
use std::collections::HashMap;

//
// Include all the images so it can just be 1 binary, no paths / resources
//
const IMG_BEAD: &'static [u8] = include_bytes!("bead.png");
const IMG_BO: &'static [u8] = include_bytes!("bo.png");
const IMG_BOBBLE: &'static [u8] = include_bytes!("bobble.png");
const IMG_K_RS: &'static [u8] = include_bytes!("k-rs.png");
const IMG_K_WS: &'static [u8] = include_bytes!("k-ws.png");
// const IMG_K2TOG_RS: &'static [u8] = include_bytes!("k2tog-rs.png");
// const IMG_K2TOG_WS: &'static [u8] = include_bytes!("k2tog-ws.png");
// const IMG_KBF: &'static [u8] = include_bytes!("kbf.png");
// const IMG_KFB: &'static [u8] = include_bytes!("kfb.png");
// const IMG_KTBL_RS: &'static [u8] = include_bytes!("ktbl-rs.png");
// const IMG_M_KWISE_RS: &'static [u8] = include_bytes!("m-kwise-rs.png");
// const IMG_M_KWISE_WS: &'static [u8] = include_bytes!("m-kwise-ws.png");
// const IMG_M_PWISE_RS: &'static [u8] = include_bytes!("m-pwise-rs.png");
// const IMG_M_PWISE_WS: &'static [u8] = include_bytes!("m-kwise-ws.png");
const IMG_ML: &'static [u8] = include_bytes!("ml.png");

const IMG_NO_STITCH: &'static [u8] = include_bytes!("nostitch.png");

fn load_image(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoder = Decoder::new(bytes);

    let (info, mut reader) = decoder.read_info()?;

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok(buf)
}

pub fn load_cache() -> Result<HashMap<(Stitch, Side), Vec<Vec<u8>>>, Box<dyn Error>> {
    let mut image_cache = HashMap::new();

    image_cache.insert((Stitch::Bead, Side::RS), vec![load_image(IMG_BEAD)?]);
    image_cache.insert((Stitch::Bead, Side::WS), vec![load_image(IMG_BEAD)?]);

    image_cache.insert((Stitch::Bo, Side::RS), vec![load_image(IMG_BO)?]);
    image_cache.insert((Stitch::Bo, Side::WS), vec![load_image(IMG_BO)?]);

    image_cache.insert((Stitch::Bobble, Side::RS), vec![load_image(IMG_BOBBLE)?]);
    image_cache.insert((Stitch::Bobble, Side::WS), vec![load_image(IMG_BOBBLE)?]);

    image_cache.insert((Stitch::K, Side::RS), vec![load_image(IMG_K_RS)?]);
    image_cache.insert((Stitch::K, Side::WS), vec![load_image(IMG_K_WS)?]);

    image_cache.insert((Stitch::Ml, Side::RS), vec![load_image(IMG_ML)?]);
    image_cache.insert((Stitch::Ml, Side::WS), vec![load_image(IMG_ML)?]);

    image_cache.insert((Stitch::NoStitch, Side::RS), vec![load_image(IMG_NO_STITCH)?]);
    image_cache.insert((Stitch::NoStitch, Side::WS), vec![load_image(IMG_NO_STITCH)?]);

    Ok(image_cache)
}
