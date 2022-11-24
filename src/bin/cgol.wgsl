fn main(neighborhood: Neighborhood, state: u32) -> u32 {
    var adj = living(neighborhood);

    if (state != 0u && adj == 2u) || adj == 3u {
        if state >= 6u { return state; } else { return state + 1u; }
    }

    return 0u;
}
