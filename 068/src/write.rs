use crate::data;
use maple_jobs::job::{StatConstraint, WepSet};
use std::{
    fmt,
    io::{self, Write},
};

pub fn write_intercalated<
    W: Write,
    T: fmt::Display,
    I: ExactSizeIterator<Item = T>,
    S: fmt::Display,
>(
    mut writer: W,
    iter: I,
    sep: S,
) -> io::Result<()> {
    let len = iter.len();

    for (i, elem) in iter.enumerate() {
        if i == len - 1 {
            write!(writer, "{}", elem)?;
        } else {
            write!(writer, "{}{}", elem, sep)?;
        }
    }

    Ok(())
}

pub fn write_intercalated_with<
    W: Write,
    T,
    I: ExactSizeIterator<Item = T>,
    S: fmt::Display,
    F: FnMut(&mut W, T) -> io::Result<()>,
>(
    mut writer: W,
    iter: I,
    mut display_fn: F,
    sep: S,
) -> io::Result<()> {
    let len = iter.len();

    for (i, elem) in iter.enumerate() {
        if i == len - 1 {
            display_fn(&mut writer, elem)?;
        } else {
            display_fn(&mut writer, elem)?;
            write!(writer, "{}", sep)?;
        }
    }

    Ok(())
}

pub fn write_stat_constraint<W: Write>(
    mut writer: W,
    sc: &StatConstraint,
) -> io::Result<()> {
    match sc {
        StatConstraint::Ful(s) => write!(writer, "**Ful**({})", s),
        StatConstraint::Less(s) => write!(writer, "**Less**({})", s),
        StatConstraint::Pure(s) => write!(writer, "**Pure**({})", s),
    }
}

pub fn write_wep_set<W: Write>(mut writer: W, ws: &WepSet) -> io::Result<()> {
    match ws {
        WepSet::All => writer.write_all(b"\\[all\\]"),
        WepSet::WepTypes(tys) => {
            write_intercalated(writer, tys.iter().map(|ty| ty.plural()), ", ")
        }
        WepSet::WepIds(ids) => write_intercalated(
            writer,
            ids.iter().map(|&id| {
                if let Some(name) = data::wep_name(id) {
                    name
                } else {
                    eprintln!("No name for weapon ID {}", id);
                    panic!()
                }
            }),
            ", ",
        ),
    }
}
