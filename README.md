# GSI

Implementation of a map tile retriever for "Chi-ri-in-tile" ("地理院タイル"; ja-JP) of "Geospatial Information Authority of Japan" (GSI).

## Features

- Tile
    - [x] Get a blob `Vec<u8>` of a tile using `id`, `x`, `y`, `z`, `ext` and params.
    - [x] Get a image `image::DynamicImage` of a image tile.
    - [x] Get a altitudes `Vec<f64>` of a DEM(.png; GSJ-Nishioka-Nagatsu-2015) tile.
    - [x] Get a altitudes `Vec<f64>` of a DEM(.txt; CSV) tile.
- [`cocotile`]
    - [x] Get a tile-`id`s as `Vec<String>`.
- [`Layers.txt`]
    - [x] Get a Layers.txt as `Layers`.
        - [x] `Layers` type = Layers.txt; `{"layers":[{...},...]}`
        - [x] `LayerVariant` type = enum of `Layer` | `LayerGroup`
        - [x] `Layer` type and `LayerGroup` type = entry of `"type":"Layer"` | `"type":"LayerGroup"`
    - [x] Make/Store/Load `CachedLayers` (=`HashMap<String, Layer>`) feature for local `Layer` informations caching
        - [x] Retrieve the other Layers.txt source file of `"src"` external referencing in `Layer`
            - [x] Explicit enabling/disabling switching
        - [x] `.to_json_string`, `.to_json_vec`; serde_json syntax sugar for storing a local `Layer` informations cache
        - [x] `.from_json_string`, `.from_json_slice`; serde_json syntax sugar for loading a local `Layer` informations cache

[`cocotile`]:https://github.com/gsi-cyberjapan/cocotile-spec
[`layers.txt`]:https://github.com/gsi-cyberjapan/layers-dot-txt-spec

## Examples or Tests

- To see: [tests/](tests/) and [examples/](examples/)

## Reference

- <https://maps.gsi.go.jp/development/ichiran.html>; ja-JP
- <https://www.gsj.jp/>; ja-JP
- <https://github.com/gsi-cyberjapan/gsimaps>; ja-JP
- <https://github.com/gsi-cyberjapan/layers-dot-txt-spec>; ja-JP
    - <https://github.com/gsi-cyberjapan/layers-dot-txt-spec/blob/master/list.md>; ja-JP
- <https://github.com/gsi-cyberjapan/cocotile-spec>; ja-JP

### See also

- <https://maps.gsi.go.jp/development/demtile.html>; ja-JP

## License

- [MIT](LICENSE)

### GSI files

These files are GSI's tiles. It's for only to use unit tests, no need essentially.

- `tests/gsi-std-z14-x14622-y6017.png` from <https://cyberjapandata.gsi.go.jp/xyz/std/14/14622/6017.png>
- `tests/seamlessphoto-std-z14-x14622-y6017.jpg` from <https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/14/14622/6017.jpg>

- <https://www.gsi.go.jp/kikakuchousei/kikakuchousei40182.html>; ja-JP

## Author

- USAGI.NETWORK / Usagi Ito <https://usagi.network/>
