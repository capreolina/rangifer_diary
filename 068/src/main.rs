mod data;
mod write;

use crate::{data::skill_name, write::write_wep_set};
use maple_jobs::job::{Job, Location, WepSet};
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{self, Write},
};
use write::{
    write_intercalated, write_intercalated_with, write_stat_constraint,
};

static PRELUDE: &[u8] = b"| name | classes | location | primary stats | \
secondary stats | stat constraints | allowed weapons | canonical weapons | \
ammo? | allowed skills |\n| ---- | ---- | ---- | ---- | ---- | ---- | ---- | \
---- | ---- | ---- |\n";

fn main() -> Result<(), Box<dyn Error>> {
    let mut universe = {
        let mut universe_ron_file = File::open("./odd_job_universe.ron")?;
        maple_jobs::from_reader(&mut universe_ron_file)?
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(PRELUDE)?;

    let mut universe: Vec<_> = universe.drain().collect();
    universe.sort_unstable_by(|(n0, j0), (n1, j1)| {
        job_compare((n0, j0), (n1, j1))
    });

    for (name, job) in universe {
        stdout.write_all(b"| **")?;
        stdout.write_all(name.as_bytes())?;
        stdout.write_all(b"** | ")?;

        write_intercalated(&mut stdout, job.classes.iter(), ", ")?;

        write!(stdout, " | {} | ", job.location)?;

        write_intercalated(&mut stdout, job.stats.primary.iter(), ", ")?;
        stdout.write_all(b" | ")?;
        write_intercalated(&mut stdout, job.stats.secondary.iter(), ", ")?;
        stdout.write_all(b" | ")?;

        for (i, sum) in job.stats.constraints.iter().enumerate() {
            if i == job.stats.constraints.len() - 1 {
                if sum.len() > 1 {
                    stdout.write_all(b"\\[")?;
                }
                write_intercalated_with(
                    &mut stdout,
                    sum.iter(),
                    |w, sc| write_stat_constraint(w, sc),
                    " ∨ ",
                )?;
                if sum.len() > 1 {
                    stdout.write_all(b"\\]")?;
                }
            } else {
                if sum.len() > 1 {
                    stdout.write_all(b"\\[")?;
                }
                write_intercalated_with(
                    &mut stdout,
                    sum.iter(),
                    |w, sc| write_stat_constraint(w, sc),
                    " ∨ ",
                )?;
                if sum.len() > 1 {
                    stdout.write_all(b"\\]")?;
                }

                stdout.write_all(" ∧ ".as_bytes())?;
            }
        }
        stdout.write_all(b" | ")?;

        write_wep_set(&mut stdout, &job.weaponry.allowed)?;
        stdout.write_all(b" | ")?;
        write_wep_set(&mut stdout, &job.weaponry.canonical)?;
        stdout.write_all(b" | ")?;

        stdout.write_all(if job.ammo { b"yes" } else { b"no" })?;
        stdout.write_all(b" | ")?;

        if let Some(skills) = job.skills {
            write_intercalated(
                &mut stdout,
                skills.iter().map(|&id| {
                    if let Some(name) = skill_name(id) {
                        name
                    } else {
                        eprintln!("No name for skill ID {}", id);
                        panic!()
                    }
                }),
                ", ",
            )?;
        } else {
            stdout.write_all(b"\\[all\\]")?;
        }
        stdout.write_all(b" |\n")?;
    }

    Ok(())
}

fn job_compare<S: AsRef<str>>(
    (n0, j0): (S, &Job),
    (n1, j1): (S, &Job),
) -> Ordering {
    (match (j0.location, j1.location) {
        (l0, l1) if l0 == l1 => Ordering::Equal,
        (Location::Camp, _) => Ordering::Less,
        (_, Location::Camp) => Ordering::Greater,
        (Location::MapleIsland, _) => Ordering::Less,
        (_, Location::MapleIsland) => Ordering::Greater,
        _ => Ordering::Equal,
    })
    .then_with(|| {
        let mut classes0: Vec<_> =
            j0.classes.iter().map(|&c| u16::from(c)).collect();
        classes0.sort_unstable();
        let mut classes1: Vec<_> =
            j1.classes.iter().map(|&c| u16::from(c)).collect();
        classes1.sort_unstable();

        classes0.iter().cmp(classes1.iter())
    })
    .then_with(|| {
        j0.stats
            .constraints
            .is_empty()
            .cmp(&j1.stats.constraints.is_empty())
    })
    .then_with(|| {
        (j0.weaponry.allowed == WepSet::All)
            .cmp(&(j1.weaponry.allowed == WepSet::All))
    })
    .then_with(|| j0.ammo.cmp(&j1.ammo))
    .then_with(|| j0.skills.is_none().cmp(&j1.skills.is_none()))
    .then_with(|| n0.as_ref().cmp(n1.as_ref()))
}
