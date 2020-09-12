use crate::error::GsiError;

pub async fn get_response(url: &str) -> Result<surf::Response, GsiError>
{
 let res = surf::get(url).await.map_err(|_| GsiError::HttpRequestIsFailed)?;
 if !res.status().is_success()
 {
  Err(GsiError::HttpResponseIsNotSucceeded(res.status()))?;
 }
 Ok(res)
}

pub async fn get_blob(url: &str) -> Result<Vec<u8>, GsiError>
{
 let mut res = get_response(url).await?;
 let blob = res.body_bytes().await.map_err(|_| GsiError::HttpResponseCouldNotGetBody)?;
 Ok(blob)
}

pub async fn get_string(url: &str) -> Result<String, GsiError>
{
 let mut res = get_response(url).await?;
 let string = res.body_string().await.map_err(|_| GsiError::HttpResponseCouldNotGetBodyString)?;
 Ok(string)
}

pub async fn get_json<T>(url: &str) -> Result<T, GsiError>
where
 T: serde::de::DeserializeOwned
{
 let mut res = get_response(url).await?;

 // Note: https://github.com/http-rs/surf/issues/228
 // let t = res.body_json::<T>().await.map_err(|e| GsiError::HttpResponseCouldNotGetBodyJson(e))?;

 // Thus, this workaround with `strip_bom`
 use strip_bom::StripBom;
 let s = res.body_string().await.map_err(|_| GsiError::HttpResponseCouldNotGetBodyString)?;
 let s = s.strip_bom();
 let t = serde_json::from_str::<T>(s)?;

 Ok(t)
}
