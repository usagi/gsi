use crate::{
 error::GsiError,
 http
};
use serde::{
 Deserialize,
 Serialize
};
use std::collections::HashMap;

pub const URL_ROOT_DIR_GITHUB: &str = "https://raw.githubusercontent.com/gsi-cyberjapan/gsimaps/gh-pages/layers_txt/";
pub const URL_ROOT_DIR_GSI: &str = "https://maps.gsi.go.jp/layers_txt/";
/// Reference: <https://github.com/gsi-cyberjapan/layers-dot-txt-spec/blob/master/list.md>
pub const PATH_FIRST_LAYERS: [&str; 8] = [
 // ベースマップ
 "layers0.txt",
 // 年代別の写真
 "layers1.txt",
 // 標高・土地の凹凸
 "layers2.txt",
 // 土地の成り立ち・土地利用
 "layers3.txt",
 // 基準点・地磁気・地殻変動
 "layers4.txt",
 // 災害伝承・避難場所
 "layers5.txt",
 // 近年の災害
 "layers6.txt",
 // その他
 "layers7.txt"
];

pub type LeafletLatLngBounds = [[f64; 2]; 2];

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LeafletStringOrStringArray
{
 String(String),
 StringArray(Vec<String>)
}

/// note: https://github.com/serde-rs/serde/issues/1030#issuecomment-522278006
fn default_true() -> bool
{
 true
}

/// note: https://github.com/serde-rs/serde/issues/368
fn default_subdomains() -> LeafletStringOrStringArray
{
 LeafletStringOrStringArray::String(String::from("abc"))
}

fn is_default_subdomains(a: &LeafletStringOrStringArray) -> bool
{
 match a
 {
  LeafletStringOrStringArray::String(s) if &s[..] == "abc" => true,
  _ => false
 }
}

fn is_true(a: &bool) -> bool
{
 a.clone()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer
{
 #[serde(rename = "type")]
 pub ty: String,
 pub id: String,
 pub title: String,
 pub url: String,

 #[serde(default = "default_subdomains", skip_serializing_if = "is_default_subdomains")]
 pub subdomains: LeafletStringOrStringArray,
 #[serde(default, skip_serializing_if = "String::is_empty")]
 pub attribution: String,
 #[serde(default = "default_true", skip_serializing_if = "is_true")]
 pub cocotile: bool,
 #[serde(rename = "minZoom", default, skip_serializing_if = "Option::is_none")]
 pub min_zoom: Option<u8>,
 #[serde(rename = "maxZoom", default, skip_serializing_if = "Option::is_none")]
 pub max_zoom: Option<u8>,
 #[serde(rename = "maxNativeZoom", default, skip_serializing_if = "Option::is_none")]
 pub max_native_zoom: Option<u8>,

 #[serde(rename = "iconUrl", default, skip_serializing_if = "String::is_empty")]
 pub icon_url: String,
 #[serde(rename = "legendUrl", default, skip_serializing_if = "String::is_empty")]
 pub legend_url: String,
 /// note: https://github.com/gsi-cyberjapan/layers-dot-txt-spec/issues/1
 #[serde(rename = "styleurl", default, skip_serializing_if = "String::is_empty")]
 pub style_url: String,

 #[serde(rename = "errorTileUrl", default, skip_serializing_if = "String::is_empty")]
 pub error_tile_url: String,
 #[serde(default, skip_serializing_if = "Option::is_none")]
 pub bounds: Option<LeafletLatLngBounds>,

 #[serde(rename = "html", default, skip_serializing_if = "String::is_empty")]
 pub html: String
}

impl Layer
{
 /// Get the {ext} part. ( It's not same to the extension part of filepath. )
 /// eg.1: url = "https://maps.gsi.go.jp/xyz/dem/{z}/{x}/{y}.txt" => ".txt"
 /// eg.2: url = "https://maps.gsi.go.jp/xyz/std/{z}/{x}/{y}.png?_=20200803a" => ".png?_=20200803a"
 /// eg.1: url = "https://example.com/{x}/{y}/{z}" => ""
 pub fn ext(&self) -> String
 {
  let p = std::path::Path::new(&self.url);
  match p.extension()
  {
   Some(ext) => format!(".{}", ext.to_string_lossy()),
   None => "".into()
  }
 }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayerGroup
{
 #[serde(rename = "type")]
 pub ty: String,
 /// note: https://github.com/gsi-cyberjapan/layers-dot-txt-spec/issues/2
 #[serde(default)]
 pub id: String,
 pub title: String,
 #[serde(default)]
 pub toggleall: bool,
 #[serde(default)]
 pub entries: Vec<LayerVariant>,
 #[serde(default)]
 pub src: String,
 #[serde(rename = "iconUrl", default)]
 pub icon_url: String,
 #[serde(rename = "legendUrl", default)]
 pub legend_url: String,
 #[serde(rename = "html", default)]
 pub html: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LayerVariant
{
 Layer(Layer),
 LayerGroup(LayerGroup)
}

impl LayerVariant
{
 pub fn is_layer(&self) -> bool
 {
  match self
  {
   LayerVariant::Layer(_) => true,
   _ => false
  }
 }

 pub fn is_layer_group(&self) -> bool
 {
  match self
  {
   LayerVariant::LayerGroup(_) => true,
   _ => false
  }
 }

 pub fn as_layer(&self) -> Option<&Layer>
 {
  match self
  {
   LayerVariant::Layer(l) => Some(l),
   _ => None
  }
 }

 pub fn as_layer_group(&self) -> Option<&LayerGroup>
 {
  match self
  {
   LayerVariant::LayerGroup(g) => Some(g),
   _ => None
  }
 }

 pub fn ty(&self) -> &str
 {
  match self
  {
   LayerVariant::Layer(l) => &l.ty[..],
   LayerVariant::LayerGroup(g) => &g.ty[..]
  }
 }

 pub fn title(&self) -> &str
 {
  match self
  {
   LayerVariant::Layer(l) => &l.title[..],
   LayerVariant::LayerGroup(g) => &g.title[..]
  }
 }

 pub fn icon_url(&self) -> &str
 {
  match self
  {
   LayerVariant::Layer(l) => &l.icon_url[..],
   LayerVariant::LayerGroup(g) => &g.icon_url[..]
  }
 }

 pub fn legend_url(&self) -> &str
 {
  match self
  {
   LayerVariant::Layer(l) => &l.legend_url[..],
   LayerVariant::LayerGroup(g) => &g.legend_url[..]
  }
 }

 pub fn html(&self) -> &str
 {
  match self
  {
   LayerVariant::Layer(l) => &l.html[..],
   LayerVariant::LayerGroup(g) => &g.html[..]
  }
 }

 pub fn iter<'a>(&'a self) -> Box<dyn std::iter::Iterator<Item = &'a Self> + 'a>
 {
  match self
  {
   LayerVariant::Layer(_) => Box::new(std::iter::once(self)),
   LayerVariant::LayerGroup(g) => Box::new(std::iter::once(self).chain(Box::new(g.entries.iter().flat_map(|v| v.iter()))))
  }
 }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layers
{
 pub layers: Vec<LayerVariant>
}

impl Layers
{
 pub fn iter<'a>(&'a self) -> Box<dyn std::iter::Iterator<Item = &'a LayerVariant> + 'a>
 {
  Box::new(self.layers.iter().flat_map(|a| a.iter()))
 }
}

/// { key: id -> value: Layer } mapper
pub type CachedLayers = HashMap<String, Layer>;

pub async fn get_first_layers_from(base_url: &str) -> Vec<Result<Layers, GsiError>>
{
 let mapper = PATH_FIRST_LAYERS.iter().map(|path| {
  let url = format!("{}{}", base_url, path);
  async move { get_layers_from_url(&url).await }
 });

 let results = mapper.collect::<Vec<_>>();
 let results = futures::future::join_all(results).await;

 results
}

pub async fn get_first_layers_from_github() -> Vec<Result<Layers, GsiError>>
{
 get_first_layers_from(URL_ROOT_DIR_GITHUB).await
}

pub async fn get_first_layers_from_gsi() -> Vec<Result<Layers, GsiError>>
{
 get_first_layers_from(URL_ROOT_DIR_GSI).await
}

pub fn get_layers_from_json(source: &str) -> Result<Layers, GsiError>
{
 let layers = serde_json::from_str::<Layers>(source).map_err(|e| GsiError::SerdeJsonError(e))?;
 Ok(layers)
}

pub async fn get_layers_from_url(url: &str) -> Result<Layers, GsiError>
{
 http::get_json::<Layers>(url).await
}

pub async fn get_layers_from_github(path: &str) -> Result<Layers, GsiError>
{
 let url = format!("{}{}", URL_ROOT_DIR_GITHUB, path);
 get_layers_from_url(&url).await
}

pub async fn get_layers_from_gsi(path: &str) -> Result<Layers, GsiError>
{
 let url = format!("{}{}", URL_ROOT_DIR_GSI, path);
 get_layers_from_url(&url).await
}

pub async fn merge_layer_cache_from_url(cached_layers: &mut CachedLayers, layers_txt_url: &str) -> Result<Vec<LayerGroup>, GsiError>
{
 eprintln!("merge_layer_cache_from_url, layers_txt_url={}", layers_txt_url);

 let layers = get_layers_from_url(layers_txt_url).await?;

 let mut has_src_groups: Vec<LayerGroup> = vec![];

 for layer_variant in layers.iter()
 {
  match layer_variant
  {
   LayerVariant::Layer(l) =>
   {
    cached_layers.insert(l.id.clone(), l.clone());
   },
   LayerVariant::LayerGroup(g) if !g.src.is_empty() => has_src_groups.push(g.clone()),
   _ => ()
  }
 }

 Ok(has_src_groups)
}

pub async fn make_cached_layers_from_urls(layers_txt_urls: &[&str], resolve_src: bool) -> Result<CachedLayers, GsiError>
{
 let mut cached_layers = CachedLayers::new();

 let mut url_queue = std::collections::vec_deque::VecDeque::<String>::new();
 layers_txt_urls.iter().for_each(|&url| url_queue.push_back(url.into()));

 while let Some(url) = url_queue.pop_front()
 {
  let has_src_groups = merge_layer_cache_from_url(&mut cached_layers, &url).await?;
  if resolve_src
  {
   let base_url = url.rsplitn(2, '/').collect::<Vec<_>>()[1];
   has_src_groups.iter().for_each(|g| {
    match g.src.starts_with("./")
    {
     true => url_queue.push_back(format!("{}/{}", base_url, &g.src)),
     false => url_queue.push_back(g.src.clone())
    }
   })
  }
 }
 Ok(cached_layers)
}

pub async fn make_cached_layers_from_url(layers_txt_url: &str, resolve_src: bool) -> Result<CachedLayers, GsiError>
{
 make_cached_layers_from_urls(&[layers_txt_url], resolve_src).await
}

pub async fn make_cached_layers_github(resolve_src: bool) -> Result<CachedLayers, GsiError>
{
 let layers_txt_urls = PATH_FIRST_LAYERS
  .iter()
  .map(|&path| format!("{}{}", URL_ROOT_DIR_GITHUB, path))
  .collect::<Vec<_>>();
 let layers_txt_urls = layers_txt_urls.iter().map(AsRef::as_ref).collect::<Vec<_>>();
 make_cached_layers_from_urls(&layers_txt_urls, resolve_src).await
}

pub async fn make_cached_layers_gsi(resolve_src: bool) -> Result<CachedLayers, GsiError>
{
 let layers_txt_urls = PATH_FIRST_LAYERS
  .iter()
  .map(|&path| format!("{}{}", URL_ROOT_DIR_GSI, path))
  .collect::<Vec<_>>();
 let layers_txt_urls = layers_txt_urls.iter().map(AsRef::as_ref).collect::<Vec<_>>();
 make_cached_layers_from_urls(&layers_txt_urls, resolve_src).await
}

pub trait CachedLayersSerializeDeserialize
{
 fn to_json_string(&self) -> Result<String, serde_json::Error>;
 fn to_json_vec(&self) -> Result<Vec<u8>, serde_json::Error>;
 fn from_json_string(json_string: &str) -> Result<CachedLayers, serde_json::Error>;
 fn from_json_slice(json_slice: &[u8]) -> Result<CachedLayers, serde_json::Error>;
}

impl CachedLayersSerializeDeserialize for CachedLayers
{
 fn to_json_string(&self) -> Result<String, serde_json::Error>
 {
  serde_json::to_string(&self)
 }

 fn to_json_vec(&self) -> Result<Vec<u8>, serde_json::Error>
 {
  serde_json::to_vec(&self)
 }

 fn from_json_string(json_string: &str) -> Result<CachedLayers, serde_json::Error>
 {
  serde_json::from_str(json_string)
 }

 fn from_json_slice(json_slice: &[u8]) -> Result<CachedLayers, serde_json::Error>
 {
  serde_json::from_slice(json_slice)
 }
}
