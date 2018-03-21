pub fn solve(days_in_month: &[u8; 12]) -> u8 {
    let mut current_day = 0;
    let mut current_month = 0;
    let mut current_year = 1900;
    let mut nb_sundays = 0;

    loop {
        if current_year > 2000 {
            break;
        }

        if current_day == 6 && current_year > 1900 {
            nb_sundays += 1;
        }

        let nb_days_for_month;

        if current_month == 1 && current_year % 4 == 0 && ((current_year % 100 == 0 && current_year % 400 == 0) || (current_year % 100 != 0)) {
            nb_days_for_month = days_in_month[current_month] + 1;
            
        } else {
            nb_days_for_month = days_in_month[current_month]
        }

        current_day += nb_days_for_month % 7;
        current_day = current_day % 7;

        current_month += 1;

        if current_month > 11 {
            current_month = 0;
            current_year += 1;
        }
    }

    nb_sundays
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        assert_eq!(solve(&days_in_month), 171);
    }
}
