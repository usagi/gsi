use std::str::Utf8Error;
use surf::http::status::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GsiError
{
 #[error("HTTP Request is failed.")]
 HttpRequestIsFailed,

 #[error("HTTP Response is not succeeded. status-code = {0:?} ")]
 HttpResponseIsNotSucceeded(StatusCode),

 #[error("HTTP Response could not get a body.")]
 HttpResponseCouldNotGetBody,

 #[error("HTTP Response could not get a body as String.")]
 HttpResponseCouldNotGetBodyString,

 #[error("HTTP Response could not get a body as JSON.")]
 HttpResponseCouldNotGetBodyJson(std::io::Error),

 #[error("serde_json error.")]
 SerdeJsonError(#[from] serde_json::Error),

 #[error("`get_dem`, unsupported ext parameter error. The actual ext = {0}. The expected ext is `.png` or `.txt` .")]
 GetDemUnsupportedExt(String),

 #[error("Image error.")]
 ImageError(#[from] image::ImageError),

 #[error("GSJ Nishioka-Nagatsu method error.")]
 GsjNishiokaNagatsuError(#[from] gsj::altitude_tile::error::NishiokaNagatsuError),

 #[error("UTF-8 error.")]
 StdStrUtf8Error(#[from] Utf8Error),

 #[error("`decode_dem_txt`, dimension error. The actual number of the decoded elements is {0}. The expected length is 65536.")]
 DecodeDemTxtDimensionsError(usize)
}
