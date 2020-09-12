use image::GenericImageView;

const ID_STD: &str = "std";
const ID_SEAMLESSPHOTO: &str = "seamlessphoto";
const ID_DEM_PNG: &str = "dem_png";
const ID_DEM: &str = "dem";
const X: u32 = 14622;
const Y: u32 = 6017;
const Z: u8 = 14;
const EXT_PNG: &str = ".png";
const EXT_JPG: &str = ".jpg";
const EXT_TXT: &str = ".txt";

#[test]
fn make_url()
{
 let actual = gsi::tile::make_url_id_xyz_ext(ID_STD, X, Y, Z, EXT_PNG);
 const EXPECTED: &str = "https://cyberjapandata.gsi.go.jp/xyz/std/14/14622/6017.png";
 assert_eq!(actual, EXPECTED);
}

#[test]
fn get_image_std()
{
 let actual = smol::block_on(gsi::tile::get_tile_as_image(ID_STD, X, Y, Z, EXT_PNG)).unwrap();
 let expected = image::open("tests/gsi-std-z14-x14622-y6017.png").unwrap();
 for ((_, _, a), (_, _, e)) in actual.pixels().zip(expected.pixels())
 {
  assert_eq!(a, e);
 }
}

#[test]
fn get_image_seamlessphoto()
{
 let actual = smol::block_on(gsi::tile::get_tile_as_image(ID_SEAMLESSPHOTO, X, Y, Z, EXT_JPG)).unwrap();
 let expected = image::open("tests/gsi-seamlessphoto-z14-x14622-y6017.jpg").unwrap();
 for ((_, _, a), (_, _, e)) in actual.pixels().zip(expected.pixels())
 {
  assert_eq!(a, e);
 }
}

#[test]
fn get_dems()
{
 let dem_from_png = smol::block_on(gsi::tile::get_tile_as_dem(ID_DEM_PNG, X, Y, Z, EXT_PNG)).unwrap();
 let dem_from_txt = smol::block_on(gsi::tile::get_tile_as_dem(ID_DEM, X, Y, Z, EXT_TXT)).unwrap();
 for (altitude_from_png, altitude_from_txt) in dem_from_png.iter().zip(dem_from_txt.iter())
 {
  assert_eq!(altitude_from_png.round(), altitude_from_txt.round());
 }
}
