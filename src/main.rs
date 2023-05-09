use std::str::FromStr;
use num::Complex;

fn main() {
    println!("Hello, world!");
}

/// 'limit'を繰り返し回数の上限として、'c'がマンデンブロ集合に含まれるかどうかを判定する
/// 'c'がマンデンブロ集合に含まれないなら'Some(i)'を返す。'i'は'c'が原点を中心とする
/// 半径2の円から出るまでにかかった繰り返し回数となる。'c'がマンデンブロ集合に含まれているらしい場合、
/// （正確に言うと'c'が、繰り返し回数に達してもマンデンブロ集合に含まれなかったことを示せなかった場合）
/// Noneを返す。
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// 文字列's'は座標のペア。'"400x600"'や'"1.0,0.5"'など
/// 
/// 's'は<left><sep><right>の形でなければならない。<sep>は’separator'引数で与えられる文字で、
/// <left>、<right>は共に'T::from_str'でパースできる文字列、
/// 'separator'はASCII文字でなければならない
/// 
/// 's'が適切な形であれば'Some<(x,y)>'を返却する。パースできなければNoneを返す
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("20,10", ','), Some((20,10)));
    assert_eq!(parse_pair::<i32>("20,10x", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}