 

/*

This will act in place of the 'wad' system   -- for rendering menu scene 

(our menu scene does not have a WAD file) 

*/
use super::scene::{Scene, Config as SceneConfig};

use super::errors::{Error, ErrorKind, Result};
use engine::{DependenciesFrom, System};
use failchain::{bail, ResultExt};
use log::info;
use std::path::PathBuf;
use wad::{
    Archive, Level as WadLevel, LevelAnalysis, LevelVisitor, LevelWalker, Result as WadResult,
    TextureDirectory, WadName,WadMetadata
};

use wad::types::{WadCoord, WadLinedef, WadSector, WadSidedef, WadSubsector, WadThing, WadVertex};
use std::str::{self, FromStr};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Config {
    pub wad_path: PathBuf,
    pub metadata_path: PathBuf,
    pub initial_level_index: usize,
}

pub struct SceneLayout {
  //  pub archive: Archive,
    pub textures: TextureDirectory,
    pub level: WadLevel,
    pub analysis: LevelAnalysis,

    level_name: WadName,
    current_level_index: usize,
    next_level_index: usize,
    level_changed: bool,

    pub sidedefs: Vec<WadSidedef>,
    pub sectors: Vec<WadSector> 

    //meta:WadMetadata
}

impl SceneLayout {
    pub fn level_name(&self) -> WadName {
        self.level_name
    }

    pub fn level_index(&self) -> usize {
        self.current_level_index
    }

    pub fn change_level(&mut self, new_level_index: usize) {
        self.next_level_index = new_level_index;
    }

    pub fn level_changed(&self) -> bool {
        self.level_changed
    }

    pub fn walk<V: LevelVisitor>(&self, visitor: &mut V) {
        LevelWalker::new(
            &self.level,
            &self.analysis,
            &self.textures,
            &self.metadata(),
            visitor,
        )
        .walk();
    }



    //stub in for now ! 
    pub fn metadata(&self) -> WadMetadata {
       let metadata = WadMetadata::from_text(
            r#"
            [[sky]]
                level_pattern = "MAP(0[1-9]|10|11)"
                texture_name = "SKY1"
                tiled_band_size = 0.15
            [[sky]]
                level_pattern = "MAP(1[2-9]|20)"
                texture_name = "SKY2"
                tiled_band_size = 0.15
            [[sky]]
                level_pattern = "MAP(2[1-9]|32)"
                texture_name = "SKY3"
                tiled_band_size = 0.15
            [animations]
                flats = [
                    ["NUKAGE1", "NUKAGE2", "NUKAGE3"],
                    [],
                ]
                walls = [
                    [],
                    ["DBRAIN1", "DBRAIN2", "DBRAIN3",  "DBRAIN4"],
                ]
            [things]
                [[things.decorations]]
                    thing_type = 10
                    radius = 16
                    sprite = "PLAY"
                    sequence = "W"
                    obstacle = false
                    hanging = false

                [[things.decorations]]
                    thing_type = 12
                    radius = 8
                    sprite = "PLAY"
                    sequence = "W"
                    obstacle = false
                    hanging = false

                [[things.weapons]]
                    # BFG 9000
                    thing_type = 2006
                    radius = 20
                    sprite = "BFUG"
                    sequence = "A"
                    hanging = false

                [[things.artifacts]]
                    # Computer map
                    thing_type = 2026
                    radius = 20
                    sprite = "PMAP"
                    sequence = "ABCDCB"
                    hanging = false

                [[things.ammo]]
                    # Box of ammo
                    thing_type = 2048
                    radius = 20
                    sprite = "AMMO"
                    sequence = "A"
                    hanging = false

                [[things.powerups]]
                    # Backpack
                    thing_type = 8
                    radius = 20
                    sprite = "BPAK"
                    sequence = "A"
                    hanging = false

                [[things.keys]]
                    # Red keycard
                    thing_type = 13
                    radius = 20
                    sprite = "RKEY"
                    sequence = "AB"
                    hanging = false

                [[things.monsters]]
                    # Baron of Hell
                    thing_type = 3003
                    radius = 24
                    sprite = "BOSS"
                    sequence = "A"
                    hanging = false
        "#,
        ).expect("Could not parse WadMetadata text");


        return metadata 

    } 
    
}




#[derive(DependenciesFrom)]
pub struct Dependencies<'context> {
    config: &'context SceneConfig,
}

impl<'context> System<'context> for SceneLayout {
    type Dependencies = Dependencies<'context>;
    type Error = Error;

    fn debug_name() -> &'static str {
        "wad"
    }

    fn create(deps: Dependencies) -> Result<Self> {


       /* let (archive, textures, level_index, level_name) = (|| -> WadResult<_> {
            let archive = Archive::open(&deps.config.wad_path, &deps.config.metadata_path)?;
            let textures = TextureDirectory::from_archive(&archive)?;
            let level_index = deps.config.initial_level_index;
            let level_name = archive.level_lump(level_index)?.name();
            Ok((archive, textures, level_index, level_name))
        })()
        .chain_err(|| ErrorKind(format!("WAD setup failed with: {:#?}", deps.config)))?;*/

        let level_index = 0;
        let level_name = WadName::from_str("mscene").expect("Could not build wad name");

       /* if level_index >= archive.num_levels() {
            bail!(
                ErrorKind,
                "Level index {} is not in valid range 0..{}, see --list-levels for level names.",
                level_index,
                archive.num_levels()
            );
        }*/ 

        info!(
            "Loading initial level {:?} ({})...",
            level_name, level_index
        );
     /*  let level = WadLevel::from_archive(&archive, level_index).chain_err(|| {
            ErrorKind(format!(
                "when loading WAD level with config {:#?}",
                deps.config
            ))
        })?;*/

        let level = WadLevel::new_empty();

        //need to load fake data here ! 

        info!("Analysing level...");
        let analysis = LevelAnalysis::new_empty();
        
         
        
        
        //LevelAnalysis::new(&level, archive.metadata());

        //empty 
        let sidedefs = Vec::new();
        let sectors = Vec::new();

         //empty 
        let textures = TextureDirectory::new_empty() ; 


        Ok(SceneLayout {
          //  archive,
            textures,
            level,
            analysis,
            current_level_index: level_index,
            next_level_index: level_index,
            level_changed: false,
            level_name,
            sidedefs,
            sectors
          //  meta
        })
    }

    fn update(&mut self, _deps: Dependencies) -> Result<()> {
        self.level_changed = false;

       /*  if self.next_level_index != self.current_level_index {
            if self.next_level_index >= self.archive.num_levels() {
                info!(
                    "New level index {} is out of bounds, keeping current.",
                    self.next_level_index
                );
                self.next_level_index = self.current_level_index;
            } else {
                self.current_level_index = self.next_level_index;
                self.level_name = self
                    .archive
                    .level_lump(self.next_level_index)
                    .chain_err(|| {
                        ErrorKind(format!(
                            "while accessing level name for next level request {}",
                            self.next_level_index
                        ))
                    })?
                    .name();
                info!(
                    "Loading new level {:?} ({})...",
                    self.level_name, self.next_level_index
                );
                self.level = WadLevel::from_archive(&self.archive, self.current_level_index)
                    .chain_err(|| {
                        ErrorKind(format!(
                            "while loading next level {} ({}) for next level request",
                            self.level_name, self.next_level_index
                        ))
                    })?;
                info!("Analysing new level...");
                self.analysis = LevelAnalysis::new(&self.level, self.archive.metadata());
                info!("Level replaced.");
                self.level_changed = true;
            }
        }*/
        Ok(())
    }
}
