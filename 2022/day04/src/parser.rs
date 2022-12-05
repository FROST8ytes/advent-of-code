use nom::bytes::complete::tag;
use nom::character::complete::u32 as ncu32;
use nom::IResult;

type AssignmentRange = (u32, u32);
pub type AssignmentPairs = (AssignmentRange, AssignmentRange);

fn sections(input: &str) -> IResult<&str, AssignmentRange> {
    let (input, start) = ncu32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = ncu32(input)?;
    Ok((input, (start, end)))
}

pub fn assignment_pairs(input: &str) -> IResult<&str, AssignmentPairs> {
    let (input, first) = sections(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, second) = sections(input)?;

    Ok((input, (first, second)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data<'a>() -> (&'a str, Vec<AssignmentPairs>) {
        let expected_outcomes: Vec<AssignmentPairs> = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        (
            "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8",
            expected_outcomes,
        )
    }

    #[test]
    fn assignment_pairs_works() {
        let (data, expected_outcomes) = get_data();
        let data: Vec<&str> = data.lines().collect();

        for idx in 0..data.len() {
            let ranges = data.get(idx).unwrap().clone();
            let result = assignment_pairs(ranges);
            let expected_outcome = Ok(("", expected_outcomes.get(idx).unwrap().clone()));

            assert_eq!(expected_outcome, result);
        }
    }
}
