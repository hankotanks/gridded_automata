fn main(neighborhood: Neighborhood, state: u32) -> u32 {
    if(state == 2u) {
        return 3u;
    } else if(state == 3u) {
        return 1u;
    } else if(state == 1u) {
        let adj = matching(neighborhood, 2u);
        if(adj == 1u || adj == 2u) {
            return 2u;
        } else {
            return 1u;
        }
    }

    return 0u;
}
