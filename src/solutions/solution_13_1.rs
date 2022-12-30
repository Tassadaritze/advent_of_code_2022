use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{cut, map};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::cmp::Ordering;

#[derive(Debug)]
enum Fragment {
    Integer(u8),
    List(Vec<Fragment>),
}

impl AsRef<u8> for Fragment {
    fn as_ref(&self) -> &u8 {
        match self {
            Fragment::Integer(val) => val,
            _ => panic!("could not convert a Fragment reference into a u8 reference"),
        }
    }
}

impl AsRef<Vec<Fragment>> for Fragment {
    fn as_ref(&self) -> &Vec<Fragment> {
        match self {
            Fragment::List(val) => val,
            _ => panic!("could not convert a Fragment reference into a Vec<Fragment> reference"),
        }
    }
}

fn vec(input: &str) -> IResult<&str, Vec<Fragment>> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(separated_list0(char(','), fragment), char(']'))),
        ),
    )(input)
}

fn fragment(input: &str) -> IResult<&str, Fragment> {
    alt((
        map(vec, Fragment::List),
        map(nom::character::complete::u8, Fragment::Integer),
    ))(input)
}

fn compare(list_1: &[Fragment], list_2: &[Fragment]) -> Option<bool> {
    let mut result: Option<bool> = None;

    for (i, val_1) in list_1.iter().enumerate() {
        let val_2 = match list_2.get(i) {
            Some(val) => val,
            None => return Some(false),
        };

        if std::mem::discriminant(val_1) == std::mem::discriminant(val_2) {
            match val_1 {
                Fragment::Integer(val_1) => {
                    let val_2: &u8 = val_2.as_ref();

                    match val_1.cmp(val_2) {
                        Ordering::Greater => return Some(false),
                        Ordering::Less => return Some(true),
                        _ => (),
                    }
                }
                Fragment::List(val_1) => {
                    let val_2: &Vec<Fragment> = val_2.as_ref();

                    result = compare(val_1, val_2);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        } else {
            result = match val_1 {
                Fragment::Integer(val_1) => compare(
                    &[Fragment::Integer(*val_1)],
                    <Fragment as AsRef<Vec<Fragment>>>::as_ref(val_2),
                ),
                Fragment::List(val_1) => compare(
                    val_1,
                    &[Fragment::Integer(*<Fragment as AsRef<u8>>::as_ref(val_2))],
                ),
            };
            if result.is_some() {
                return result;
            }
        }
    }

    if result.is_none() && list_2.len() > list_1.len() {
        return Some(true);
    }

    result
}

fn compare_wrapper(list_1: &[Fragment], list_2: &[Fragment]) -> bool {
    if let Some(val) = compare(list_1, list_2) {
        return val;
    }
    true
}

pub fn solve(input: &str) -> String {
    let mut sum: usize = 0;
    for (index, pair) in input.split("\n\n").enumerate() {
        let mut lines = pair.lines();
        let (packet_1, packet_2) = (
            match fragment(lines.next().unwrap()).unwrap().1 {
                Fragment::List(list) => list,
                _ => panic!("what"),
            },
            match fragment(lines.next().unwrap()).unwrap().1 {
                Fragment::List(list) => list,
                _ => panic!("what"),
            },
        );
        println!("{:?}", packet_1);
        println!("{:?}", packet_2);

        if compare_wrapper(&packet_1, &packet_2) {
            println!("CORRECT");
            sum += index + 1
        } else {
            println!("WRONG");
        };
    }

    sum.to_string()
}
