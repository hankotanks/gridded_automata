fn main(neighborhood: Neighborhood, state: u32) -> u32 {
    if(state == 0u && matching(neighborhood, 2u) == 2u) {
        return 2u;
    } else if(state == 2u) {
        return 1u;
    }

    return 0u;
}
