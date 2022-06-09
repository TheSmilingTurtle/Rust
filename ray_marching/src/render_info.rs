struct RenderInfo {
    traveled_dist: f64,
    steps_taken: u64,

    light_dist: f64
    
    intersection_depth: //positive when there was intersection, negative else, no need for intersection boolean

    colour: Colour // still need to figure this one out
}