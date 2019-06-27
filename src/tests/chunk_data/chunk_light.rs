
use crate::chunk_data::chunk_light::ChunkLight; // Subject
use crate::chunk_data::constants::BLOCK_COUNT;

#[test]
pub fn reading_block_light_level_works() {
  // Setup block light levels
  let mut block = vec![0 as u8; BLOCK_COUNT as usize];
  let block0 = 0x01;
  let block1 = 0x02;
  block[0] = (block1 << 4) | block0;
  // Create chunk light
  let chunk_light = ChunkLight::from(block, vec![0 as u8; BLOCK_COUNT as usize]);

  assert_eq!(chunk_light.get_block(0, 0, 0), block0);
  assert_eq!(chunk_light.get_block(1, 0, 0), block1);
  assert_eq!(chunk_light.get_block(2, 0, 0), 0);
}

#[test]
pub fn writing_block_light_level_works() {
  // Create a blank ChunkLight
  let mut chunk_light = ChunkLight::new();

  // Set the light level of (0,0,0) to 1
  chunk_light.set_block(0,0,0, 1);
  assert_eq!(chunk_light.block[0], 1);

  // Clean for next block
  chunk_light.block[0] = 0;

  // Set the light level of (1,0,0) to 1
  chunk_light.set_block(1,0,0, 1);
  assert_eq!(chunk_light.block[0], 0x10);
}

#[test]
pub fn reading_sky_light_level_works() {
  // Setup block light levels
  let mut sky = vec![0 as u8; BLOCK_COUNT as usize];
  let sky0 = 0x01;
  let sky1 = 0x02;
  sky[0] = (sky1 << 4) | sky0;
  // Create chunk light
  let chunk_light = ChunkLight::from(vec![0 as u8; BLOCK_COUNT as usize], sky);

  assert_eq!(chunk_light.get_sky(0, 0, 0), sky0);
  assert_eq!(chunk_light.get_sky(1, 0, 0), sky1);
  assert_eq!(chunk_light.get_sky(2, 0, 0), 0);
}

#[test]
pub fn writing_sky_light_level_works() {
  // Create a blank ChunkLight
  let mut chunk_light = ChunkLight::new();

  // Set the light level of (0,0,0) to 1
  chunk_light.set_sky(0,0,0, 1);
  assert_eq!(chunk_light.sky[0], 1);

  // Clean for next sky
  chunk_light.sky[0] = 0;

  // Set the light level of (1,0,0) to 1
  chunk_light.set_sky(1,0,0, 1);
  assert_eq!(chunk_light.sky[0], 0x10);
}