fn main(coord: vec2<i32>, state: u32) -> u32 {
    let neighborhood = moore_neighborhood(coord);
    var adj = count_living(neighborhood);

    if((state != 0u && adj == 2u) || adj == 3u) {
        return state + 1u;
    }

    return 0u;
}
