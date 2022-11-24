fn main(neighborhood: Neighborhood, state: u32) -> u32 {
    if(state == 0u && living(neighborhood) == 2u) {
        return 1u;
    } 
    
    return 0u;
}
