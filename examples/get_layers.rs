fn main()
{
 let layers = smol::block_on(async {
  let path = "./layers0.txt";
  let layers = gsi::layers::get_layers_from_gsi(path).await.unwrap();
  layers
 });
 println!("{:?}", &layers);
 let x = serde_json::to_string_pretty(&layers).unwrap();
 println!("{}", &x);
}
