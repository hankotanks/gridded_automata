fn main(neighborhood: Neighborhood, state: u32) -> u32 {
    let u: u32 = up(neighborhood);
    let l: u32 = left(neighborhood);
    let r: u32 = right(neighborhood);
    let d: u32 = down(neighborhood);

    switch state {
        case 1u, 2u, 3u, 4u { return 5u; }
        case 6u, 7u, 8u, 9u { return 0u; }
        case 0u {
            switch u { case 4u, 9u { return 2u; } default {} }
            switch l { case 3u, 8u { return 4u; } default {} }
            switch r { case 2u, 7u { return 1u; } default {} }
            switch d { case 1u, 6u { return 3u; } default {} }
        }
        case 5u {
            switch u { case 4u, 9u { return 8u; } default {} }
            switch l { case 3u, 8u { return 6u; } default {} }
            switch r { case 2u, 7u { return 9u; } default {} }
            switch d { case 1u, 6u { return 7u; } default {} }
        }
        default {}
    }

    return state;
}
