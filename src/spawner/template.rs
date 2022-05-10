use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

impl Template {
    pub fn spawn(&self, pt: &Point, cb: &mut CommandBuffer) {
        let entity = cb.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(self.glyph),
            },
            Name(self.name.clone()),
        ));

        match self.entity_type {
            EntityType::Item => cb.add_component(entity, Item {}),
            EntityType::Enemy => {
                cb.add_component(entity, Enemy {});
                cb.add_component(entity, FieldOfView::new(6));
                cb.add_component(entity, ChasingPlayer {});
                cb.add_component(
                    entity,
                    Health {
                        current: self.hp.unwrap(),
                        max: self.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &self.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => cb.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => cb.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("Warning: item has unknown effect {}", provides);
                    }
                })
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening game data file");
        from_reader(file).expect("Failed parsing game data")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut cb = CommandBuffer::new(ecs);

        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                entity.spawn(pt, &mut cb);
            }
        });

        cb.flush(ecs);
    }
}
