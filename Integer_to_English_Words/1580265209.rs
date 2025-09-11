 impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let below_20 = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
            "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen",
            "Seventeen", "Eighteen", "Nineteen",
        ];
        let tens = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let thousands = ["", "Thousand", "Million", "Billion"];

        fn helper(num: i32, below_20: &[&str], tens: &[&str]) -> String {
            if num == 0 {
                return "".to_string();
            } else if num < 20 {
                return below_20[num as usize].to_string();
            } else if num < 100 {
                return format!(
                    "{}{}",
                    tens[(num / 10) as usize],
                    if num % 10 != 0 {
                        format!(" {}", below_20[(num % 10) as usize])
                    } else {
                        "".to_string()
                    }
                );
            } else {
                return format!(
                    "{} Hundred{}",
                    below_20[(num / 100) as usize],
                    if num % 100 != 0 {
                        format!(" {}", helper(num % 100, below_20, tens))
                    } else {
                        "".to_string()
                    }
                );
            }
        }

        let mut num = num;
        let mut res = String::new();
        let mut i = 0;

        while num > 0 {
            if num % 1000 != 0 {
                let chunk = helper(num % 1000, &below_20, &tens);
                if !res.is_empty() {
                    res = format!("{} {} {}", chunk, thousands[i], res);
                } else {
                    res = format!("{} {}", chunk, thousands[i]);
                }
            }
            num /= 1000;
            i += 1;
        }

        res.trim().to_string()
    }
}
