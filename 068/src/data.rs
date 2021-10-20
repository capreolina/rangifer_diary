pub fn wep_name(id: u32) -> Option<&'static str> {
    Some(match id {
        0 => "\\[unarmed\\]",
        1302001 => "Saw",
        1312002 => "Scythe",
        _ => return None,
    })
}

pub fn skill_name(id: u32) -> Option<&'static str> {
    Some(match id {
        1121000 => "Maple Warrior",
        1121011 => "Hero’s Will",
        3000002 => "The Eye of Amazon",
        4000001 => "Keen Eyes",
        5000000 => "Bullet Time",
        5001005 => "Dash",
        5200000 => "Gun Mastery",
        5201002 => "Grenade",
        5201003 => "Gun Booster",
        5201005 => "Wings",
        5211001 => "Octopus",
        5211002 => "Gaviota",
        5220002 => "Wrath of the Octopi",
        5221003 => "Aerial Strike",
        5221009 => "Hypnotize",
        _ => return None,
    })
}
