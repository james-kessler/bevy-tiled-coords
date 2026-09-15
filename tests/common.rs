use tiled::{LayerType, Map, Object};

pub fn first_object_in_map(map: &Map) -> Object<'_> {
    fn walk<'a>(layers: impl Iterator<Item = tiled::Layer<'a>>) -> Option<Object<'a>> {
        for layer in layers {
            if let Some(object_layer) = layer.as_object_layer() {
                if let Some(object) = object_layer.objects().next() {
                    return Some(object);
                }
            }
            if let LayerType::Group(group) = layer.layer_type() {
                if let Some(object) = walk(group.layers()) {
                    return Some(object);
                }
            }
        }
        None
    }

    walk(map.layers()).expect("map should contain an object")
}
