use itertools::iproduct;

pub fn day21(input: &str) -> (String, String) {
    // TODO: read from file
    let mut scores = [0, 0];
    let mut positions = vec![(1..=10).cycle().skip(4), (1..=10).cycle().skip(6)];
    let mut deterministic_die = (1..=100).cycle();
    let mut player_turns = (0..scores.len()).cycle();
    let mut rolls = 0;

    while !scores.into_iter().any(|v| v >= 1000) {
        let current_player = player_turns.next().unwrap();
        let move_amount: u32 = deterministic_die.by_ref().take(3).sum();
        //println!("Move amount: {}", move_amount);
        for _ in 1..move_amount {
            positions[current_player].by_ref().next();
        }
        let new_pos = positions[current_player].by_ref().next().unwrap();
        scores[current_player] += new_pos;
        //println!("Current player: {}", current_player);
        //println!("Position: {}", new_pos);
        //println!("{:?}", current_player);
        //println!("Score: {:?}", scores[current_player]);
        rolls += 1;
    }

    let score = scores.into_iter().min().unwrap() * rolls * 3;

    let i = (1..=3)
        .into_iter()
        .flat_map(|v| (1..=3).map(move |w| (v, w)))
        .collect::<Vec<(usize, usize)>>();

    for (i, j, k) in iproduct!(i.clone(), i.clone(), i.clone()) {
        println!("{:?} {:?} {:?}", i, j, k);
    }

    (score.to_string(), "".to_string())
}
