fn main(coord: vec2<i32>, state: u32) -> u32 {
    let neighborhood = moore_neighborhood(coord);
    var neighbor_count = count_living(neighborhood);

    if((state != 0u && neighbor_count == 2u) || neighbor_count == 3u) {
        return 1u;
    } else {
        return 0u;
    }
}
