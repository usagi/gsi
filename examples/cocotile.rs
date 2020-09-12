fn main()
{
 let x = 14623;
 let y = 6017;
 let z = 14;
 let ids = smol::block_on(gsi::tile::cocotile(x, y, z)).unwrap();
 println!("{:?}", ids);
}
