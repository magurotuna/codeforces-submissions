//! https://github.com/hatoo/competitive-rust-snippets
//!
//! MIT License
//!
//! Copyright (c) 2018 hatoo
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
#![allow(
    unused_imports,
    unused_attributes,
    unused_macros,
    dead_code,
    non_snake_case
)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;
#[macro_export]
macro_rules !get {(@inner [$src :expr ] chars ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .chars () .collect ::<Vec <char >>() } } ;(@inner [$src :expr ] usize1 ) =>{{get !(@inner [$src ] usize ) -1 } } ;(@inner [$src :expr ] [usize1 ] ) =>{{get !(@inner [$src ] [usize ] ) .into_iter () .map (|v |v -1 ) .collect ::<Vec <usize >>() } } ;(@inner [$src :expr ] [[usize1 ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [usize1 ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[chars ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [chars ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [($($tt :tt ) ,*) ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] ($($tt ) ,*) ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] ($($tt :tt ) ,*) ) =>{{let mut buf :String =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;let mut iter =buf .split_whitespace () ;($(get !(@inner_elem_parse [$tt ] iter .next () .unwrap () ) ,) *) } } ;(@inner [$src :expr ] [$t :ty ] ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .map (|t |t .parse ::<$t >() .unwrap () ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[$t :ty ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [$t :ty ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] $t :ty ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .next () .unwrap () .parse ::<$t >() .unwrap () } } ;(@inner_elem_parse [usize1 ] $elem :expr ) =>{{get !(@inner_elem_parse [usize ] $elem ) -1 } } ;(@inner_elem_parse [$t :ty ] $elem :expr ) =>{{$elem .parse ::<$t >() .unwrap () } } ;($tt :tt ) =>{{use std ::io ::BufRead ;let get_stdin =std ::io ::stdin () ;let mut locked_stdin =get_stdin .lock () ;get !(@inner [&mut locked_stdin ] $tt ) } } ;}
macro_rules !debug {($($a :expr ) ,*$(,) *) =>{#[cfg (debug_assertions ) ] eprintln !(concat !($("| " ,stringify !($a ) ,"={:?} " ) ,*,"|" ) ,$(&$a ) ,*) ;} ;}
macro_rules !echo {($($a :expr ) ,*) =>{let mut s =Vec ::new () ;$(s .push (format !("{}" ,$a ) ) ;) *println !("{}" ,s .join (" " ) ) ;} }
#[macro_export]
macro_rules !chmin {($base :expr ,$($cmps :expr ) ,+$(,) *) =>{{let cmp_min =min !($($cmps ) ,+) ;if $base >cmp_min {$base =cmp_min ;true } else {false } } } ;}
#[macro_export]
macro_rules !chmax {($base :expr ,$($cmps :expr ) ,+$(,) *) =>{{let cmp_max =max !($($cmps ) ,+) ;if $base <cmp_max {$base =cmp_max ;true } else {false } } } ;}
#[macro_export]
macro_rules !min {($a :expr $(,) *) =>{{$a } } ;($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::min ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::min ($a ,min !($($rest ) ,+) ) } } ;}
#[macro_export]
macro_rules !max {($a :expr $(,) *) =>{{$a } } ;($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::max ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::max ($a ,max !($($rest ) ,+) ) } } ;}
const BIG_STACK_SIZE: bool = false;
fn main() {
    use std::thread;
    if BIG_STACK_SIZE {
        thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .name("solve".into())
            .spawn(solve)
            .unwrap()
            .join()
            .unwrap();
    } else {
        solve();
    }
}
fn solve() {
    let t = get!(usize);
    'test: for _ in 0..t {
        let n = get!(usize);
        let a = get!([usize]);
        let mut evens = Vec::new();
        let mut odds = Vec::new();
        for aa in a {
            if aa % 2 == 0 {
                evens.push(aa);
            } else {
                odds.push(aa);
            }
        }
        if evens.len() % 2 == 0 && odds.len() % 2 == 0 {
            echo!("YES");
            continue 'test;
        }

        let ev_set: HashSet<_> = evens.iter().cloned().collect();
        let od_set: HashSet<_> = odds.iter().cloned().collect();
        debug!(ev_set, od_set);
        if evens.len() < odds.len() {
            for e in evens {
                if od_set.contains(&(e - 1)) || od_set.contains(&(e + 1)) {
                    echo!("YES");
                    continue 'test;
                }
            }
        } else {
            for o in odds {
                if ev_set.contains(&(o - 1)) || ev_set.contains(&(o + 1)) {
                    echo!("YES");
                    continue 'test;
                }
            }
        }
        echo!("NO");
    }
}
fn solve2() {
    let t = get!(usize);
    'test: for _ in 0..t {
        let n = get!(usize);
        let a = get!([usize]);
        let mut nums = vec![0; 101];
        let mut ev = 0;
        let mut od = 0;
        for aa in a {
            if aa % 2 == 0 {
                ev += 1;
            } else {
                od += 1;
            }
            nums[aa] += 1;
        }
        if ev % 2 == 0 && od % 2 == 0 {
            echo!("YES");
            continue 'test;
        }

        let mut link1 = Vec::new();
        for i in (1..nums.len()).filter(|&x| x % 2 != 0) {
            if nums[i] == 0 {
                continue;
            }

            if nums[i - 1] > 0 && nums[i + 1] == 0 {
                link1.push((i, i - 1));
            } else if nums[i + 1] > 0 && nums[i - 1] == 0 {
                link1.push((i, i + 1));
            }
        }

        for &(from, to) in &link1 {
            let f_num = nums[from];
            let t_num = nums[to];
            if f_num <= t_num {
                nums[from] -= f_num;
                nums[to] -= f_num;
            } else {
                nums[from] -= t_num;
                nums[to] -= t_num;
            }
        }
        link1.clear();

        let mut even = 0;
        let mut odd = 0;
        for i in 1..nums.len() {
            if i % 2 == 0 {
                even += nums[i];
            } else {
                odd += nums[i];
            }
        }
        if even % 2 == 0 && odd % 2 == 0 {
            echo!("YES");
            continue 'test;
        }

        for i in (1..nums.len()).filter(|&x| x % 2 != 0) {
            if nums[i] == 0 {
                continue;
            }

            if nums[i - 1] > 0 && nums[i + 1] == 0 {
                link1.push((i, i - 1));
            } else if nums[i + 1] > 0 && nums[i - 1] == 0 {
                link1.push((i, i + 1));
            }
        }

        for &(from, to) in &link1 {
            let f_num = nums[from];
            let t_num = nums[to];
            if f_num <= t_num {
                nums[from] -= f_num;
                nums[to] -= f_num;
            } else {
                nums[from] -= t_num;
                nums[to] -= t_num;
            }
        }

        let mut even = 0;
        let mut odd = 0;
        for i in 1..nums.len() {
            if i % 2 == 0 {
                even += nums[i];
            } else {
                odd += nums[i];
            }
        }
        if even % 2 == 0 && odd % 2 == 0 {
            echo!("YES");
        } else {
            echo!("NO");
        }
    }
}
