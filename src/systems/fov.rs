use crate::prelude::*;

#[system(for_each)]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(fov: &mut FieldOfView, pos: &Point, #[resource] map: &Map) {
    if fov.is_dirty {
        fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
        fov.is_dirty = false;
    }
}
