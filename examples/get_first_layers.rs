fn main()
{
 let first_layers = smol::block_on(gsi::layers::get_first_layers_from_gsi());

 // It's so verbose.
 // println!("{:?}", first_layers);

 // Then, alternate show essentially.
 for (first_layers_index, layers_maybe) in first_layers.iter().enumerate()
 {
  println!(
   "[layers-{}]: {}",
   first_layers_index,
   if layers_maybe.is_ok() { "Ok" } else { "Err" }
  );
  if let Ok(layers) = layers_maybe
  {
   for (layer_index, layer_variant) in layers.iter().enumerate()
   {
    let (ty, additional_infos) = match layer_variant
    {
     gsi::layers::LayerVariant::Layer(l) => ("L", format!("id={}, ext={}", l.id, l.ext())),
     gsi::layers::LayerVariant::LayerGroup(g) => ("G", format!("#entries={}, src={}", g.entries.len(), g.src))
    };
    println!(
     r#" ({}|{}) => title="{}", {}"#,
     layer_index,
     ty,
     layer_variant.title(),
     additional_infos
    )
   }
  }
 }
}
