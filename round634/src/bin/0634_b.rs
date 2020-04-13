#![allow(unused_macros)]

// cf. [Rustで競技プログラミングの入力をスッキリ記述するマクロ - Qiita](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        t: usize,
        nab: [(usize, usize, usize); t],
    }
    let mut ans = Vec::with_capacity(t);
    use std::collections::HashMap;

    for (n, a, b) in nab {
        let mut map = HashMap::new();
        let mut s = vec!['?'; n];
        let mut cnt = 0;
        for i in 0..(n - a + 1) {
            if i == 0 {
                for j in 0..a {
                    if j <= a - b {
                        *map.entry('a').or_insert(0usize) += 1;
                        s[j] = 'a';
                    } else {
                        cnt += 1;
                        cnt %= 26;
                        *map.entry(i2c(cnt)).or_insert(0usize) += 1;
                        s[j] = i2c(cnt);
                    }
                }
            } else {
                if map.len() == b {
                    s[i + a - 1] = i2c(cnt);
                    *map.entry(i2c(cnt)).or_insert(0usize) += 1;
                } else {
                    cnt += 1;
                    cnt %= 26;
                    *map.entry(i2c(cnt)).or_insert(0usize) += 1;
                    s[i + a - 1] = i2c(cnt);
                }
            }
            *map.entry(s[i]).or_insert(99999999) -= 1;
            if *map.get(&s[i]).unwrap() == 0 {
                map.remove(&s[i]);
            }
        }
        dbg!(check(&s, a, b));
        ans.push(s.into_iter().collect::<String>());
    }
    println!("{}", ans.join("\n"));
}

fn i2c(i: usize) -> char {
    ('a' as u8 + i as u8) as char
}

fn check(c: &[char], a: usize, b: usize) -> bool {
    use std::collections::BTreeSet;
    for i in 0..(c.len() - a + 1) {
        let mut set = BTreeSet::new();
        for j in i..(i + a) {
            set.insert(c[j]);
        }
        if set.len() != b {
            //dbg!((&set, set.len(), i, c[i], c[i + a - 1]));
            return false;
        }
    }
    true
}
