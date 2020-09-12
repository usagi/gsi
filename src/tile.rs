use crate::{
 constant,
 error::GsiError,
 http
};
use image::DynamicImage;

pub fn make_url(pattern: &str, key_value_pairs: &[(&str, &str)]) -> String
{
 key_value_pairs.iter().fold(pattern.into(), |a, &(k, v)| a.replace(k, v))
}

pub fn make_url_id_xyz_ext(id: &str, x: u32, y: u32, z: u8, ext: &str) -> String
{
 make_url(constant::URL_PATTERN, &[
  (constant::URL_PATTERN_ID, id.into()),
  (constant::URL_PATTERN_Z, &z.to_string()),
  (constant::URL_PATTERN_X, &x.to_string()),
  (constant::URL_PATTERN_Y, &y.to_string()),
  (constant::URL_PATTERN_EXT, &ext.to_string())
 ])
}

pub fn make_url_pattern_id_xyz(pattern: &str, id: &str, x: u32, y: u32, z: u8) -> String
{
 make_url(pattern, &[
  (constant::URL_PATTERN_ID, id.into()),
  (constant::URL_PATTERN_Z, &z.to_string()),
  (constant::URL_PATTERN_X, &x.to_string()),
  (constant::URL_PATTERN_Y, &y.to_string())
 ])
}

pub async fn get_tile_as_blob(id: &str, x: u32, y: u32, z: u8, ext: &str) -> Result<Vec<u8>, GsiError>
{
 let url = make_url_id_xyz_ext(id, x, y, z, ext);
 http::get_blob(&url).await
}

pub async fn get_tile_as_image(id: &str, x: u32, y: u32, z: u8, ext: &str) -> Result<DynamicImage, GsiError>
{
 let t = get_tile_as_blob(id, x, y, z, ext).await?;
 let i = image::load_from_memory(&t)?;
 Ok(i)
}

pub async fn get_tile_as_string(id: &str, x: u32, y: u32, z: u8, ext: &str) -> Result<String, GsiError>
{
 let url = make_url_id_xyz_ext(id, x, y, z, ext);
 http::get_string(&url).await
}

pub async fn get_tile_as_dem(id: &str, x: u32, y: u32, z: u8, ext: &str) -> Result<Vec<f64>, GsiError>
{
 // note: GSI's URL pattern has a secret parameters such as "https://maps.gsi.go.jp/xyz/std/{z}/{x}/{y}.png?_=20200803a".
 //       Then, just in case, use `if` and `.starts_with` pattern.
 //       Reference: https://github.com/gsi-cyberjapan/gsimaps/blob/229a5c3471c73ac4657982379f245f2b05a9a5b9/layers_txt/layers0.txt#L7
 if ext.starts_with(".png")
 {
  let i = get_tile_as_image(id, x, y, z, ext).await?;
  gsj::altitude_tile::nishioka_nagatsu_2015::check(&i).map_err(|e| GsiError::GsjNishiokaNagatsuError(e))?;
  let dem = gsj::altitude_tile::nishioka_nagatsu_2015::decode(&i, constant::DEM_PNG_RESOLUTION);
  Ok(dem)
 }
 else if ext.starts_with(".txt")
 {
  let csv = get_tile_as_string(id, x, y, z, ext).await?;
  let dem_maybe = decode_dem_txt(&csv);
  dem_maybe
 }
 else
 {
  Err(GsiError::GetDemUnsupportedExt(ext.into()))
 }
}

pub fn decode_dem_txt(dem_txt: &str) -> Result<Vec<f64>, GsiError>
{
 let dem = dem_txt
  .lines()
  .map(|line| line.split(","))
  .flatten()
  .map(|t| {
   let f_maybe = t.parse::<f64>();
   match f_maybe
   {
    Ok(f) => f,
    _ => std::f64::NAN
   }
  })
  .collect::<Vec<_>>();

 const EXPECTED_LEN: usize = constant::PIXELS_PER_TILE_ARRIS as usize * constant::PIXELS_PER_TILE_ARRIS as usize;

 match dem.len()
 {
  EXPECTED_LEN => Ok(dem),
  _ => Err(GsiError::DecodeDemTxtDimensionsError(dem.len()))
 }
}

pub async fn cocotile(x: u32, y: u32, z: u8) -> Result<Vec<String>, GsiError>
{
 let url = make_url_id_xyz_ext(constant::ID_COCOTILE, x, y, z, constant::EXT_COCOTILE);
 let csv = http::get_string(&url).await?;
 Ok(csv.split(",").map(|v| v.trim().into()).collect::<Vec<String>>())
}
