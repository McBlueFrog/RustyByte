use macroquad_tiled as tiled;
use macroquad::prelude::*;
use macroquad_platformer::*;


//TODO:
// Fallback textures from notouch folder or maybe hardcoded
// Automatic tileset size recognition

pub struct GameResources {
    pub tileset: Texture2D,
    pub tiled_map: tiled::Map,
    pub colliders: Vec<Tile>
}


async fn load_atlas() -> std::io::Result<Texture2D> {
    match load_texture("resources/tileset.png").await {
        Ok(tileset) => Ok(tileset),
        Err(e) => {
            eprintln!("Failed to load atlas texture: {:?}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load atlas texture"))
        }
    }
}

async fn load_map() -> std::io::Result<String> {
    match load_string("resources/map.json").await {
        Ok(tiled_map_json) => Ok(tiled_map_json),
        Err(e) => {
            eprintln!("Failed to load map: {:?}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load map"))
        }
    }

}

async fn init_atlas() -> std::io::Result<Texture2D> {
    match load_atlas().await {
        Ok(tileset) => {
            println!("Successfully loaded atlas");
            tileset.set_filter(FilterMode::Nearest);
            Ok(tileset)
        }
        Err(e) => {
            println!("Failed to load atlas");
            return Err(e);
        }
    }
}

async fn build_map(tiled_map_json: String, tileset: Texture2D) -> std::io::Result<macroquad_tiled::Map> {
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]);
    //Ok(tiled_map)
    match tiled_map {
        Ok(tiled_map) => {
            Ok(tiled_map)
        }
        Err(e) => {
            eprintln!("Failed to load map {:?}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load map"))
        }
    }
}

async fn init_map(tileset: Texture2D) -> std::io::Result<(macroquad_tiled::Map,Vec<Tile>)> {
    match load_map().await {
        Ok(tiled_map_json) => {
            let tiled_map = build_map(tiled_map_json, tileset).await;
            match tiled_map {
                Ok(tiled_map) => {
                    let mut static_colliders = vec![];
                    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
                        static_colliders.push(if tile.is_some() {
                            Tile::Solid
                        } else {
                            Tile::Empty
                        });
                    }
                    println!("Successfully initialized map data");
                    Ok((tiled_map,static_colliders))
                }
                Err(e) => {
                    println!("Failed to generate Map");
                    Err(e)
                }
            }
            
        }
        Err(e) => {
            println!("Error while loading map data");
            return Err(e);
        }
    }
}




pub async fn load_resources() -> Result<GameResources, std::io::Error> {
        let tileset =  init_atlas().await.unwrap();
        let map_result = init_map(tileset.clone()).await;
            match map_result {
                Ok((tiled_map,colliders)) => {
                    println!("Successfully loaded resources");
                    Ok(GameResources {
                    tileset: tileset,
                    tiled_map: tiled_map,
                    colliders: colliders, // setting GameResources global structure
                    })
                }
                Err(e) => {
                    eprintln!("Failed to initialize map: {}", e);
                    Err(e)
                }
            }
    }

