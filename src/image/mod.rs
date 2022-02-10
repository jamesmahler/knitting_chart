//! The image cache

use knitting_parse::Stitch;
use knitting_parse::Side;

use png::Decoder;

use std::error::Error;
use std::collections::HashMap;

//
// Include all the images so it can just be 1 binary, no paths / resources
//
const IMG_LCF1_01: &'static [u8] = include_bytes!("1lcf-01.png");
const IMG_LCF1_02: &'static [u8] = include_bytes!("1lcf-02.png");
const IMG_RCB1_01: &'static [u8] = include_bytes!("1rcb-01.png");
const IMG_RCB1_02: &'static [u8] = include_bytes!("1rcb-02.png");
const IMG_LCF2_01: &'static [u8] = include_bytes!("2lcf-01.png");
const IMG_LCF2_02: &'static [u8] = include_bytes!("2lcf-02.png");
const IMG_LCF2_03: &'static [u8] = include_bytes!("2lcf-03.png");
const IMG_LCF2_04: &'static [u8] = include_bytes!("2lcf-04.png");
const IMG_RCB2_01: &'static [u8] = include_bytes!("2rcb-01.png");
const IMG_RCB2_02: &'static [u8] = include_bytes!("2rcb-02.png");
const IMG_RCB2_03: &'static [u8] = include_bytes!("2rcb-03.png");
const IMG_RCB2_04: &'static [u8] = include_bytes!("2rcb-04.png");
const IMG_LCF3_01: &'static [u8] = include_bytes!("3lcf-01.png");
const IMG_LCF3_02: &'static [u8] = include_bytes!("3lcf-02.png");
const IMG_LCF3_03: &'static [u8] = include_bytes!("3lcf-03.png");
const IMG_LCF3_04: &'static [u8] = include_bytes!("3lcf-04.png");
const IMG_LCF3_05: &'static [u8] = include_bytes!("3lcf-05.png");
const IMG_LCF3_06: &'static [u8] = include_bytes!("3lcf-06.png");
const IMG_RCB3_01: &'static [u8] = include_bytes!("3rcb-01.png");
const IMG_RCB3_02: &'static [u8] = include_bytes!("3rcb-02.png");
const IMG_RCB3_03: &'static [u8] = include_bytes!("3rcb-03.png");
const IMG_RCB3_04: &'static [u8] = include_bytes!("3rcb-04.png");
const IMG_RCB3_05: &'static [u8] = include_bytes!("3rcb-05.png");
const IMG_RCB3_06: &'static [u8] = include_bytes!("3rcb-06.png");
const IMG_LCF4_01: &'static [u8] = include_bytes!("4lcf-01.png");
const IMG_LCF4_02: &'static [u8] = include_bytes!("4lcf-02.png");
const IMG_LCF4_03: &'static [u8] = include_bytes!("4lcf-03.png");
const IMG_LCF4_04: &'static [u8] = include_bytes!("4lcf-04.png");
const IMG_LCF4_05: &'static [u8] = include_bytes!("4lcf-05.png");
const IMG_LCF4_06: &'static [u8] = include_bytes!("4lcf-06.png");
const IMG_LCF4_07: &'static [u8] = include_bytes!("4lcf-07.png");
const IMG_LCF4_08: &'static [u8] = include_bytes!("4lcf-08.png");
const IMG_RCB4_01: &'static [u8] = include_bytes!("4rcb-01.png");
const IMG_RCB4_02: &'static [u8] = include_bytes!("4rcb-02.png");
const IMG_RCB4_03: &'static [u8] = include_bytes!("4rcb-03.png");
const IMG_RCB4_04: &'static [u8] = include_bytes!("4rcb-04.png");
const IMG_RCB4_05: &'static [u8] = include_bytes!("4rcb-05.png");
const IMG_RCB4_06: &'static [u8] = include_bytes!("4rcb-06.png");
const IMG_RCB4_07: &'static [u8] = include_bytes!("4rcb-07.png");
const IMG_RCB4_08: &'static [u8] = include_bytes!("4rcb-08.png");
const IMG_BEAD: &'static [u8] = include_bytes!("bead.png");
const IMG_BO: &'static [u8] = include_bytes!("bo.png");
const IMG_BOBBLE: &'static [u8] = include_bytes!("bobble.png");
const IMG_K_RS: &'static [u8] = include_bytes!("k-rs.png");
const IMG_K_WS: &'static [u8] = include_bytes!("k-ws.png");
const IMG_K2TOG_RS: &'static [u8] = include_bytes!("k2tog-rs.png");
const IMG_K2TOG_WS: &'static [u8] = include_bytes!("k2tog-ws.png");
const IMG_KBF: &'static [u8] = include_bytes!("kbf.png");
const IMG_KFB: &'static [u8] = include_bytes!("kfb.png");
const IMG_KTBL_RS: &'static [u8] = include_bytes!("ktbl-rs.png");
const IMG_KTBL_WS: &'static [u8] = include_bytes!("ktbl-ws.png");
const IMG_M_KWISE_RS: &'static [u8] = include_bytes!("m-kwise-rs.png");
const IMG_M_KWISE_WS: &'static [u8] = include_bytes!("m-kwise-ws.png");
const IMG_M_PWISE_RS: &'static [u8] = include_bytes!("m-pwise-rs.png");
const IMG_M_PWISE_WS: &'static [u8] = include_bytes!("m-pwise-ws.png");
const IMG_ML: &'static [u8] = include_bytes!("ml.png");
const IMG_MR: &'static [u8] = include_bytes!("mr.png");
const IMG_NO_STITCH: &'static [u8] = include_bytes!("nostitch.png");
const IMG_P_RS: &'static [u8] = include_bytes!("p-rs.png");
const IMG_P_WS: &'static [u8] = include_bytes!("p-ws.png");
const IMG_P2TOG_RS: &'static [u8] = include_bytes!("p2tog-rs.png");
const IMG_P2TOG_WS: &'static [u8] = include_bytes!("p2tog-ws.png");
const IMG_PBF: &'static [u8] = include_bytes!("pbf.png");
const IMG_PFB: &'static [u8] = include_bytes!("pfb.png");
const IMG_PTBL_RS: &'static [u8] = include_bytes!("ptbl-rs.png");
const IMG_PTBL_WS: &'static [u8] = include_bytes!("ptbl-ws.png");
const IMG_SL_KWISE: &'static [u8] = include_bytes!("sl-kwise.png");
const IMG_SL_PWISE: &'static [u8] = include_bytes!("sl-pwise.png");
const IMG_SSK_RS: &'static [u8] = include_bytes!("ssk-rs.png");
const IMG_SSK_WS: &'static [u8] = include_bytes!("ssk-ws.png");
const IMG_SSP_RS: &'static [u8] = include_bytes!("ssp-rs.png");
const IMG_SSP_WS: &'static [u8] = include_bytes!("ssp-ws.png");
const IMG_YO: &'static [u8] = include_bytes!("yo.png");


fn load_image(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoder = Decoder::new(bytes);

    let mut reader = decoder.read_info()?;

    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok(buf)
}

pub fn load_cache() -> Result<HashMap<(Stitch, Side), Vec<Vec<u8>>>, Box<dyn Error>> {
    let mut image_cache = HashMap::new();

    image_cache.insert((Stitch::Lcf1, Side::RS), vec![load_image(IMG_LCF1_01)?, load_image(IMG_LCF1_02)?]);
    image_cache.insert((Stitch::Lcf1, Side::WS), vec![load_image(IMG_LCF1_01)?, load_image(IMG_LCF1_02)?]);

    image_cache.insert((Stitch::Rcb1, Side::RS), vec![load_image(IMG_RCB1_01)?, load_image(IMG_RCB1_02)?]);
    image_cache.insert((Stitch::Rcb1, Side::WS), vec![load_image(IMG_RCB1_01)?, load_image(IMG_RCB1_02)?]);

    image_cache.insert((Stitch::Lcf2, Side::RS), vec![load_image(IMG_LCF2_01)?, load_image(IMG_LCF2_02)?, load_image(IMG_LCF2_03)?, load_image(IMG_LCF2_04)?]);
    image_cache.insert((Stitch::Lcf2, Side::WS), vec![load_image(IMG_LCF2_01)?, load_image(IMG_LCF2_02)?, load_image(IMG_LCF2_03)?, load_image(IMG_LCF2_04)?]);

    image_cache.insert((Stitch::Rcb2, Side::RS), vec![load_image(IMG_RCB2_01)?, load_image(IMG_RCB2_02)?, load_image(IMG_RCB2_03)?, load_image(IMG_RCB2_04)?]);
    image_cache.insert((Stitch::Rcb2, Side::WS), vec![load_image(IMG_RCB2_01)?, load_image(IMG_RCB2_02)?, load_image(IMG_RCB2_03)?, load_image(IMG_RCB2_04)?]);

    image_cache.insert((Stitch::Lcf3, Side::RS), vec![load_image(IMG_LCF3_01)?, load_image(IMG_LCF3_02)?, load_image(IMG_LCF3_03)?, load_image(IMG_LCF3_04)?, load_image(IMG_LCF3_05)?, load_image(IMG_LCF3_06)?]);
    image_cache.insert((Stitch::Lcf3, Side::WS), vec![load_image(IMG_LCF3_01)?, load_image(IMG_LCF3_02)?, load_image(IMG_LCF3_03)?, load_image(IMG_LCF3_04)?, load_image(IMG_LCF3_05)?, load_image(IMG_LCF3_06)?]);

    image_cache.insert((Stitch::Rcb3, Side::RS), vec![load_image(IMG_RCB3_01)?, load_image(IMG_RCB3_02)?, load_image(IMG_RCB3_03)?, load_image(IMG_RCB3_04)?, load_image(IMG_RCB3_05)?, load_image(IMG_RCB3_06)?]);
    image_cache.insert((Stitch::Rcb3, Side::WS), vec![load_image(IMG_RCB3_01)?, load_image(IMG_RCB3_02)?, load_image(IMG_RCB3_03)?, load_image(IMG_RCB3_04)?, load_image(IMG_RCB3_05)?, load_image(IMG_RCB3_06)?]);

    image_cache.insert((Stitch::Lcf4, Side::RS), vec![load_image(IMG_LCF4_01)?, load_image(IMG_LCF4_02)?, load_image(IMG_LCF4_03)?, load_image(IMG_LCF4_04)?, load_image(IMG_LCF4_05)?, load_image(IMG_LCF4_06)?, load_image(IMG_LCF4_07)?, load_image(IMG_LCF4_08)?]);
    image_cache.insert((Stitch::Lcf4, Side::WS), vec![load_image(IMG_LCF4_01)?, load_image(IMG_LCF4_02)?, load_image(IMG_LCF4_03)?, load_image(IMG_LCF4_04)?, load_image(IMG_LCF4_05)?, load_image(IMG_LCF4_06)?, load_image(IMG_LCF4_07)?, load_image(IMG_LCF4_08)?]);

    image_cache.insert((Stitch::Rcb4, Side::RS), vec![load_image(IMG_RCB4_01)?, load_image(IMG_RCB4_02)?, load_image(IMG_RCB4_03)?, load_image(IMG_RCB4_04)?, load_image(IMG_RCB4_05)?, load_image(IMG_RCB4_06)?, load_image(IMG_RCB4_07)?, load_image(IMG_RCB4_08)?]);
    image_cache.insert((Stitch::Rcb4, Side::WS), vec![load_image(IMG_RCB4_01)?, load_image(IMG_RCB4_02)?, load_image(IMG_RCB4_03)?, load_image(IMG_RCB4_04)?, load_image(IMG_RCB4_05)?, load_image(IMG_RCB4_06)?, load_image(IMG_RCB4_07)?, load_image(IMG_RCB4_08)?]);

    image_cache.insert((Stitch::Bead, Side::RS), vec![load_image(IMG_BEAD)?]);
    image_cache.insert((Stitch::Bead, Side::WS), vec![load_image(IMG_BEAD)?]);

    image_cache.insert((Stitch::Bo, Side::RS), vec![load_image(IMG_BO)?]);
    image_cache.insert((Stitch::Bo, Side::WS), vec![load_image(IMG_BO)?]);

    image_cache.insert((Stitch::Bobble, Side::RS), vec![load_image(IMG_BOBBLE)?]);
    image_cache.insert((Stitch::Bobble, Side::WS), vec![load_image(IMG_BOBBLE)?]);

    image_cache.insert((Stitch::K, Side::RS), vec![load_image(IMG_K_RS)?]);
    image_cache.insert((Stitch::K, Side::WS), vec![load_image(IMG_K_WS)?]);

    image_cache.insert((Stitch::K2Tog, Side::RS), vec![load_image(IMG_K2TOG_RS)?]);
    image_cache.insert((Stitch::K2Tog, Side::WS), vec![load_image(IMG_K2TOG_WS)?]);

    image_cache.insert((Stitch::Kbf, Side::RS), vec![load_image(IMG_KBF)?]);
    image_cache.insert((Stitch::Kbf, Side::WS), vec![load_image(IMG_KBF)?]);

    image_cache.insert((Stitch::Kfb, Side::RS), vec![load_image(IMG_KFB)?]);
    image_cache.insert((Stitch::Kfb, Side::WS), vec![load_image(IMG_KFB)?]);

    image_cache.insert((Stitch::Ktbl, Side::RS), vec![load_image(IMG_KTBL_RS)?]);
    image_cache.insert((Stitch::Ktbl, Side::WS), vec![load_image(IMG_KTBL_WS)?]);

    image_cache.insert((Stitch::MKwise, Side::RS), vec![load_image(IMG_M_KWISE_RS)?]);
    image_cache.insert((Stitch::MKwise, Side::WS), vec![load_image(IMG_M_KWISE_WS)?]);

    image_cache.insert((Stitch::MPwise, Side::RS), vec![load_image(IMG_M_PWISE_RS)?]);
    image_cache.insert((Stitch::MPwise, Side::WS), vec![load_image(IMG_M_PWISE_WS)?]);

    image_cache.insert((Stitch::Ml, Side::RS), vec![load_image(IMG_ML)?]);
    image_cache.insert((Stitch::Ml, Side::WS), vec![load_image(IMG_ML)?]);

    image_cache.insert((Stitch::Mr, Side::RS), vec![load_image(IMG_MR)?]);
    image_cache.insert((Stitch::Mr, Side::WS), vec![load_image(IMG_MR)?]);

    image_cache.insert((Stitch::NoStitch, Side::RS), vec![load_image(IMG_NO_STITCH)?]);
    image_cache.insert((Stitch::NoStitch, Side::WS), vec![load_image(IMG_NO_STITCH)?]);

    image_cache.insert((Stitch::P, Side::RS), vec![load_image(IMG_P_RS)?]);
    image_cache.insert((Stitch::P, Side::WS), vec![load_image(IMG_P_WS)?]);

    image_cache.insert((Stitch::P2Tog, Side::RS), vec![load_image(IMG_P2TOG_RS)?]);
    image_cache.insert((Stitch::P2Tog, Side::WS), vec![load_image(IMG_P2TOG_WS)?]);

    image_cache.insert((Stitch::Pbf, Side::RS), vec![load_image(IMG_PBF)?]);
    image_cache.insert((Stitch::Pbf, Side::WS), vec![load_image(IMG_PBF)?]);

    image_cache.insert((Stitch::Pfb, Side::RS), vec![load_image(IMG_PFB)?]);
    image_cache.insert((Stitch::Pfb, Side::WS), vec![load_image(IMG_PFB)?]);

    image_cache.insert((Stitch::Ptbl, Side::RS), vec![load_image(IMG_PTBL_RS)?]);
    image_cache.insert((Stitch::Ptbl, Side::WS), vec![load_image(IMG_PTBL_WS)?]);

    image_cache.insert((Stitch::SlKwise, Side::RS), vec![load_image(IMG_SL_KWISE)?]);
    image_cache.insert((Stitch::SlKwise, Side::WS), vec![load_image(IMG_SL_KWISE)?]);

    image_cache.insert((Stitch::SlPwise, Side::RS), vec![load_image(IMG_SL_PWISE)?]);
    image_cache.insert((Stitch::SlPwise, Side::WS), vec![load_image(IMG_SL_PWISE)?]);
    
    image_cache.insert((Stitch::Ssk, Side::RS), vec![load_image(IMG_SSK_RS)?]);
    image_cache.insert((Stitch::Ssk, Side::WS), vec![load_image(IMG_SSK_WS)?]);

    image_cache.insert((Stitch::Ssp, Side::RS), vec![load_image(IMG_SSP_RS)?]);
    image_cache.insert((Stitch::Ssp, Side::WS), vec![load_image(IMG_SSP_WS)?]);

    image_cache.insert((Stitch::Yo, Side::RS), vec![load_image(IMG_YO)?]);
    image_cache.insert((Stitch::Yo, Side::WS), vec![load_image(IMG_YO)?]);

    Ok(image_cache)
}
