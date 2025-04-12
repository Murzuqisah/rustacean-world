
pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn spell_below_100(n: u64, units: &[&str], teens: &[&str], tens: &[&str]) -> String {
        match n {
            0..=9 => units[n as usize].to_string(),
            10..=19 => teens[(n - 10) as usize].to_string(),
            20..=99 => {
                let t = n / 10;
                let u = n % 10;
                if u == 0 {
                    tens[t as usize].to_string()
                } else {
                    format!("{}-{}", tens[t as usize], units[u as usize])
                }
            }
            _ => "".to_string(),
        }
    }

    fn spell_below_1000(n: u64, units: &[&str], teens: &[&str], tens: &[&str]) -> String {
        let h = n / 100;
        let r = n % 100;
        let mut result = String::new();
        if h > 0 {
            result.push_str(units[h as usize]);
            result.push_str(" hundred");
            if r > 0 {
                result.push(' ');
            }
        }
        if r > 0 {
            result.push_str(&spell_below_100(r, units, teens, tens));
        }
        result
    }

    if n == 1_000_000 {
        return "one million".to_string();
    }

    let thousand = n / 1000;
    let rest = n % 1000;
    let mut result = String::new();

    if thousand > 0 {
        result.push_str(&spell_below_1000(thousand, &units, &teens, &tens));
        result.push_str(" thousand");
        if rest > 0 {
            result.push(' ');
        }
    }

    if rest > 0 {
        result.push_str(&spell_below_1000(rest, &units, &teens, &tens));
    }

    result
}
