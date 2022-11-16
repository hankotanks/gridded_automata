fn main(coord: vec2<i32>, state: u32) -> u32 {
    if(state == 2u) {
        return 3u;
    } else if(state == 3u) {
        return 1u;
    } else if(state == 1u) {
        let adj_c = count_matching(moore_neighborhood(coord), 2u);
        if(adj_c == 1u || adj_c == 2u) {
            return 2u;
        } else {
            return 1u;
        }
    }

    return 0u;
}
