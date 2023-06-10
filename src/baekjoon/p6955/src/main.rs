fn replace_line(line: &String) -> String {
    let mut current_position: usize = 0;
    let mut barrier = 0;
    let last = line.len();

    let mut result = Vec::new();
    while let Some(pos) = line[current_position..].find(|c: char| c == '.' || c == '/'
        || c == 'J' || c == 'F' || c == 'M' || c == 'A' || c == 'S' || c == 'O' || c == 'N' || c == 'D') {
        current_position += pos;
        let cur = line.chars().nth(current_position).unwrap();

        if cur == '.' || cur == '/' {
            let mut flag = false;
            if current_position - barrier < 2 {
                flag = true;
            }

            if current_position >= 3 {
                let c = line.chars().nth(current_position - 3).unwrap();
                if c.is_ascii_alphanumeric() {
                    flag = true;
                } 
            }

            if current_position + 6 > last {
                flag = true;
            }

            if current_position + 6 < last {
                let c = line.chars().nth(current_position + 6).unwrap();
                if c.is_ascii_alphanumeric() {
                    flag = true;
                }
            }

            if current_position + 3 < last {
                let c = line.chars().nth(current_position + 3).unwrap();
                if c != cur {
                    flag = true;
                }
            }

            if flag {
                current_position += 1;
                continue;
            }

            if cur == '.' {
                let yy_bool = check_yy(&line[current_position - 2..current_position]);
                let mm_bool = check_mm(&line[current_position + 1..current_position + 3]);
                let dd_bool = check_dd(&line[current_position + 4..current_position + 6]);
                
                if yy_bool && mm_bool && dd_bool {
                    result.push(current_position - 2);
                    current_position += 6;
                    barrier = current_position;
                } else {
                    current_position += 1;
                }
            } else if cur == '/' {
                let dd_bool = check_dd(&line[current_position - 2..current_position]);
                let mm_bool = check_mm(&line[current_position + 1..current_position + 3]);
                let yy_bool = check_yy(&line[current_position + 4..current_position + 6]);
                
                if yy_bool && mm_bool && dd_bool {
                    result.push(current_position + 4);
                    current_position += 6;
                    barrier = current_position;
                } else {
                    current_position += 1;
                }
            }
        } else {
            // January, February, March, April, May, June, July, August, September, October, November, December
            if current_position >= 1 {
                let c = line.chars().nth(current_position - 1).unwrap();
                if c.is_ascii_alphanumeric() {
                    current_position += 1;
                    continue;
                } 
            }

            if let Some(pos) = line[current_position..].find(' ') {
                let s = &line[current_position..current_position + pos];
                if s == "January" || s == "February" || s == "March" || s == "April" || s == "May" || s == "June" || s == "July" || s == "August" || s == "September" || s == "October" || s == "November" || s == "December" {
                    current_position += pos;
                    if current_position + 6 > last {
                        current_position += 1;
                        continue;
                    }
                    let c = line.chars().nth(current_position + 3).unwrap();
                    if c != ',' {
                        current_position += 1;
                        continue;
                    }

                    let c = line.chars().nth(current_position + 4).unwrap();
                    if c != ' ' {
                        current_position += 1;
                        continue;
                    }

                    let dd = &line[current_position + 1..current_position + 3];
                    let yy = &line[current_position + 5..current_position + 7];

                    let yy_bool = check_yy(yy);
                    let dd_bool = check_dd(dd);
                    if yy_bool && dd_bool {
                        result.push(current_position + 5);
                        current_position += 7;
                        barrier = current_position;
                    } else {
                        current_position += 1;
                        continue;
                    }
                } else {
                    current_position += 1;
                    continue;
                }
            } else {
                current_position += 1;
                continue;
            }
        }


        if current_position >= last {
            break;
        }
    }

    let mut line = line.to_string();
    for &i in result.iter().rev() {
        line = line[0..i].to_string() + &change_yy(&line[i..i + 2]) + &line[i + 2..];
    }

    line.to_string()
}

fn check_yy(yy: &str) -> bool {
    if yy.len() != 2 {
        return false;
    }
    let parse = yy.parse::<usize>();
    match parse {
        Ok(n) => {
            if n <= 99 {
                return true;
            }
        },
        Err(_) => {},
    }
    false
}

fn check_mm(mm: &str) -> bool {
    if mm.len() != 2 {
        return false;
    }
    let parse = mm.parse::<usize>();
    match parse {
        Ok(n) => {
            if n >= 1 && n <= 12 {
                return true;
            }
        },
        Err(_) => {},
    }
    false
}

fn check_dd(dd: &str) -> bool {
    if dd.len() != 2 {
        return false;
    }
    let parse = dd.parse::<usize>();
    match parse {
        Ok(n) => {
            if n >= 1 && n <= 31 {
                return true;
            }
        },
        Err(_) => {},
    }
    false
}

fn change_yy(yy: &str) -> String {
    let mut result = String::new();
    let parse = yy.parse::<usize>();
    match parse {
        Ok(n) => {
            if n <= 24 {
                result.push_str("20");
                result.push_str(yy);
            } else {
                result.push_str("19");
                result.push_str(yy);
            }
        },
        Err(_) => panic!("error"),
    }
    result
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim_end_matches('\n').to_string();
        let result = replace_line(&mut line);
        println!("{}", result);
    }    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_replace_line() {
        let mut line = String::from("12.12.12");
        let result = replace_line(&mut line);
        assert_eq!(result, "2012.12.12");
    }

    #[test]
    fn test_find_pattern2() {
        let mut line = String::from("12.12.12.12.12.12");
        let result = replace_line(&mut line);
        assert_eq!(result, "2012.12.12.2012.12.12");
    }

    #[test]
    fn test_find_pattern3() {
        let mut line = String::from("12.13.12.12.12.12");
        let result = replace_line(&mut line);
        assert_eq!(result, "12.2013.12.12.12.12");
    }

    #[test]
    fn test_find_pattern4() {
        let mut line = String::from("33.12.12.12.00.12");
        let result = replace_line(&mut line);
        assert_eq!(result, "1933.12.12.12.00.12");
    }

    #[test]
    fn test_find_pattern5() {
        let mut line = String::from("011.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "011.11.11");
    }

    #[test]
    fn test_find_pattern6() {
        let mut line = String::from(" 11.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, " 2011.11.11");
    }

    #[test]
    fn test_find_pattern7() {
        let mut line = String::from("11.11.111");
        let result = replace_line(&mut line);
        assert_eq!(result, "11.11.111");
    }

    #[test]
    fn test_find_pattern8() {
        let mut line = String::from("111.11.111");
        let result = replace_line(&mut line);
        assert_eq!(result, "111.11.111");
    }

    #[test]
    fn test_find_pattern9() {
        let mut line = String::from("111.11.11.11.111");
        let result = replace_line(&mut line);
        assert_eq!(result, "111.2011.11.11.111");
    }

    #[test]
    fn test_replace_line0() {
        let mut line = String::from("111.11.11 11.111");
        let result = replace_line(&mut line);
        assert_eq!(result, "111.11.11 11.111");
    }

    #[test]
    fn test_replace_line1() {
        let mut line = String::from("12/12/12");
        let result = replace_line(&mut line);
        assert_eq!(result, "12/12/2012");
    }

    #[test]
    fn test_replace_line2() {
        let mut line = String::from("12/14/12");
        let result = replace_line(&mut line);
        assert_eq!(result, "12/14/12");
    }

    #[test]
    fn test_replace_line3() {
        let mut line = String::from("12/12/12/12/12/12");
        let result = replace_line(&mut line);
        assert_eq!(result, "12/12/2012/12/12/2012");
    }

    #[test]
    fn test_replace_line4() {
        let mut line = String::from("12/13/12/12/12/12");
        let result = replace_line(&mut line);
        assert_eq!(result, "12/13/12/2012/12/12");
    }

    #[test]
    fn test_replace_line5() {
        let mut line = String::from("33/12/12/33/00/11");
        let result = replace_line(&mut line);
        assert_eq!(result, "33/12/12/1933/00/11");
    }

    #[test]
    fn test_replace_line6() {
        let mut line = String::from("011/11/11");
        let result = replace_line(&mut line);
        assert_eq!(result, "011/11/11");
    }


    #[test]
    fn test_replace_line7() {
        let mut line = String::from(" 11/11/11");
        let result = replace_line(&mut line);
        assert_eq!(result, " 11/11/2011");
    }

    #[test]
    fn test_replace_line8() {
        let mut line = String::from("11/11/111");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11/111");
    }

    #[test]
    fn test_replace_line9() {
        let mut line = String::from("111/11/111");
        let result = replace_line(&mut line);
        assert_eq!(result, "111/11/111");
    }

    #[test]
    fn test_find_pattern20() {
        let mut line = String::from("111/11/11 11/111");
        let result = replace_line(&mut line);
        assert_eq!(result, "111/11/11 11/111");
    }

    #[test]
    fn test_find_patter21() {
        let mut line = String::from("11.11.11/11/11");
        let result = replace_line(&mut line);
        assert_eq!(result, "2011.11.11/11/11");
    }

    #[test]
    fn test_find_patter22() {
        let mut line = String::from("11/11.11/11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11.11/11.11");
    }

    #[test]
    fn test_find_patter23() {
        let mut line = String::from("11/13/11.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/13/2011.11.11");
    }

    #[test]
    fn test_find_patter24() {
        let mut line = String::from("11/11/11.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11/2011.11.11");
    }

    #[test]
    fn test_find_patter25() {
        let mut line = String::from("2011/11/11.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "2011/11/2011.11.11");
    }

    #[test]
    fn test_find_patter26() {
        let mut line = String::from("11/11/11.11.2011");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11/2011.11.2011");
    }

    #[test]
    fn test_find_patter27() {
        let mut line = String::from("11/11/");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11/");
    }

    #[test]
    fn test_find_patter28() {
        let mut line = String::from("11/11");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11");
    }

    #[test]
    fn test_find_patter29() {
        let mut line = String::from("11/11/    ");
        let result = replace_line(&mut line);
        assert_eq!(result, "11/11/    ");
    }

    #[test]
    fn test_find_patter30() {
        let mut line = String::from("11.11.");
        let result = replace_line(&mut line);
        assert_eq!(result, "11.11.");
    }

    #[test]
    fn test_find_patter31() {
        let mut line = String::from("11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "11.11");
    }

    #[test]
    fn test_find_pattern32() {
        let mut line = String::from("..11.11.11");
        let result = replace_line(&mut line);
        assert_eq!(result, "..2011.11.11");
    }

    #[test]
    fn test_find_pattern33() {
        let mut line = String::from("June 17, 19");
        let result = replace_line(&mut line);
        assert_eq!(result, "June 17, 2019");
    }

    #[test]
    fn test_find_pattern34() {
        let mut line = String::from("May 17, 99");
        let result = replace_line(&mut line);
        assert_eq!(result, "May 17, 1999");
    }

    #[test]
    fn test_find_pattern35() {
        let mut line = String::from("November 17, 00");
        let result = replace_line(&mut line);
        assert_eq!(result, "November 17, 2000");
    }

    #[test]
    fn test_find_pattern36() {
        let mut line = String::from("July 50, 15");
        let result = replace_line(&mut line);
        assert_eq!(result, "July 50, 15");
    }

    #[test]
    fn test_find_pattern37() {
        let mut line = String::from("June 17. 19");
        let result = replace_line(&mut line);
        assert_eq!(result, "June 17. 19");
    }

    #[test]
    fn test_find_pattern38() {
        let mut line = String::from(" June 17, 19");
        let result = replace_line(&mut line);
        assert_eq!(result, " June 17, 2019");
    }

    #[test]
    fn test_find_pattern39() {
        let mut line = String::from(" June 17, 19   ");
        let result = replace_line(&mut line);
        assert_eq!(result, " June 17, 2019   ");
    }

    #[test]
    fn test_find_pattern40() {
        let mut line = String::from(" June   17, 19   ");
        let result = replace_line(&mut line);
        assert_eq!(result, " June   17, 19   ");
    }

    #[test]
    fn test_find_pattern41() {
        let mut line = String::from("June17, 19");
        let result = replace_line(&mut line);
        assert_eq!(result, "June17, 19");
    }

    #[test]
    fn test_find_pattern42() {
        let mut line = String::from("June 20,  19");
        let result = replace_line(&mut line);
        assert_eq!(result, "June 20,  19");
    }

    #[test]
    fn test_find_pattern43() {
        let mut line = String::from("June 20.12.12");
        let result = replace_line(&mut line);
        assert_eq!(result, "June 2020.12.12");
    }


    #[test]
    fn test_another_1() {
        let input = "02/03/04/05/06/07";
        let output = "02/03/2004/05/06/2007";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_default_1() {
        let input = "Before 02/03/04, but not after December 19, 99,";
        let output = "Before 02/03/2004, but not after December 19, 1999,";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_default_2() {
        let input = "there was a rehash of the 55.34.02 meeting. A date, like November 15,";
        let output = "there was a rehash of the 55.34.02 meeting. A date, like November 15,";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_default_3() {
        let input = "95 cannot traverse two lines, nor can it be surrounded by alphabetics";
        let output = "95 cannot traverse two lines, nor can it be surrounded by alphabetics";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_default_4() {
        let input = "or numerics like this: 78November 01, 88, or 6801/12/03, or 02/03/04x.";
        let output = "or numerics like this: 78November 01, 88, or 6801/12/03, or 02/03/04x.";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    
    #[test]
    fn test_custom_1() {
        let input = "  ";
        let output = "  ";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_2() {
        let input = "  aa ";
        let output = "  aa ";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_3() {
        let input = " a x ";
        let output = " a x ";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_4() {
        let input = " ,,02/03/04 ";
        let output = " ,,02/03/2004 ";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_5() {
        let input = "55.14.02";
        let output = "55.14.02";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_6() {
        let input = "55.12.02";
        let output = "1955.12.02";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_custom_7() {
        let input = "24.12.02";
        let output = "2024.12.02";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_1() {
        let input = "I was born on 12/07/83";
        let output = "I was born on 12/07/1983";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_2() {
        let input = "She was born on 14/05/85, but I was born on 86.05.14. However, our kid was born on May 14, 13";
        let output = "She was born on 14/05/1985, but I was born on 1986.05.14. However, our kid was born on May 14, 2013";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_3() {
        let input = "The date A12/07/83B should not be recognized as a date";
        let output = "The date A12/07/83B should not be recognized as a date";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_4() {
        let input = "The date 12/07/83 should not be recognized as a date";
        let output = "The date 12/07/1983 should not be recognized as a date";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_5() {
        let input = "The date 12/07/832 should not be recognized as a date";
        let output = "The date 12/07/832 should not be recognized as a date";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_6() {
        let input = "We met on 13/06/99 and married on 00.01.01";
        let output = "We met on 13/06/1999 and married on 2000.01.01";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_7() {
        let input = "She was born on May 14, 23";
        let output = "She was born on May 14, 2023";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_8() {
        let input = "The significant years are 24.12.24 and 25.12.25";
        let output = "The significant years are 2024.12.24 and 1925.12.25";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_9() {
        let input = "The date 30/02/50 does not need to be checked";
        let output = "The date 30/02/1950 does not need to be checked";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_10() {
        let input = "";
        let output = "";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_12() {
        let input = "She was born on May     14,   23";
        let output = "She was born on May     14,   23";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_13() {
        let input = "She was born on May14,23";
        let output = "She was born on May14,23";
        assert_eq!(replace_line(&String::from(input)), output);
    }
    
    #[test]
    fn test_chatgpt_14() {
        let input = "She was born on May14, 23";
        let output = "She was born on May14, 23";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_21() {
        let input = "Today's date is 07/11";
        let output = "Today's date is 07/11";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_22() {
        let input = "She was born on 12/07/AB";
        let output = "She was born on 12/07/AB";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_23() {
        let input = "I have an event on 01/01/2";
        let output = "I have an event on 01/01/2";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_24() {
        let input = "The deadline is AB/20/99";
        let output = "The deadline is AB/20/99";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_25() {
        let input = "His birthday is 12/AB/99";
        let output = "His birthday is 12/AB/99";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_26() {
        let input = "We'll meet on 2023.07.12";
        let output = "We'll meet on 2023.07.12";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_27() {
        let input = "My daughter was born on12/07/99.";
        let output = "My daughter was born on12/07/99.";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_28() {
        let input = "She was in the class of 99";
        let output = "She was in the class of 99";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_29() {
        let input = "The document was dated 12/07/999";
        let output = "The document was dated 12/07/999";
        assert_eq!(replace_line(&String::from(input)), output);
    }

    #[test]
    fn test_chatgpt_30() {
        let input = "The event starts on 12/07/\n99";
        let output = "The event starts on 12/07/\n99";
        assert_eq!(replace_line(&String::from(input)), output);
    }    
}