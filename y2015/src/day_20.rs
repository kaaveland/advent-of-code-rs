fn find_house_with_at_least(gifts: usize, factor: usize) -> usize {
    // Start with 2 ^ 16 houses and double until we find the right one
    let mut size = 0xffff;
    let permit_50 = factor == 10;
    loop {
        let mut houses = vec![0; size];
        let mut elf = 1;
        while elf < houses.len() {
            let mut house = elf;
            let mut visits = 0;
            while house < houses.len() && (permit_50 || visits < 50) {
                houses[house] += factor * elf;
                house += elf;
                visits += 1;
            }
            elf += 1;
        }
        // Check if any house has enough gifts
        if let Some((house_number, _)) = houses
            .into_iter()
            .enumerate()
            .find(|(_, house_gifts)| *house_gifts >= gifts)
        {
            return house_number;
        }
        // Double the neighbourhood and try again
        size *= 2;
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let gifts_required: usize = s.trim().parse()?;
    Ok(find_house_with_at_least(gifts_required, 10).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let gifts_required: usize = s.trim().parse()?;
    Ok(find_house_with_at_least(gifts_required, 11).to_string())
}
