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
const BIG_STACK_SIZE: bool = true;
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
    let T = get!(usize);
    let mut ans = Vec::with_capacity(T);
    for _ in 0..T {
        let (h, c, t) = get!((i64, i64, i64));

        if h == t {
            ans.push(1.to_string());
            continue;
        }

        if h + c >= 2 * t {
            ans.push(2.to_string());
            continue;
        }

        let mut ok = 0_i64;
        let mut ng = 1_000_000_000_i64;
        while ng - ok > 1 {
            let x = (ok + ng) / 2;
            let lhs = (x + 1) * h + x * c;
            let rhs = t * (2 * x + 1);
            if lhs >= rhs {
                ok = x;
            } else {
                ng = x;
            }
        }
        debug!(ok);
        let f = |x| (x + 1) * h + x * c - (2 * x + 1) * t;
        let fl = |x| (x + 1) * h + x * c;
        let fr = |x| (2 * x + 1) * t;
        for x in max(0, ok - 10)..(ok + 10) {
            debug!(x, f(x), fl(x), fr(x));
        }
        let v1 = f(ok);
        let v1 = v1.abs();
        let ok2 = ok + 1;
        let v2 = f(ok2);
        let v2 = v2.abs();
        if v1 <= v2 {
            ans.push(cnt(ok).to_string());
        } else {
            ans.push(cnt(ok2).to_string());
        }

        // let f = |x| ((h + c) * x + h) / (2.0 * x + 1.0);
        // match t.partial_cmp(&avg).unwrap() {
        //     Ordering::Less | Ordering::Equal => {
        //         ans.push(2.to_string());
        //         continue;
        //     }
        //     Ordering::Greater => {
        //         let mut ok = 0i64;
        //         let mut ng = 1_000_000_000i64;
        //         while ng - ok > 1 {
        //             let mid = (ok + ng) / 2;
        //             let val = f(mid as f64);
        //             //let val = ((h + c) * mid + h) / (2.0 * mid + 1.0);
        //             match val.partial_cmp(&t).unwrap() {
        //                 Ordering::Greater | Ordering::Equal => ok = mid,
        //                 _ => ng = mid,
        //             }
        //         }
        //         let v1 = f(ok as f64);
        //         let v2 = f((ok + 1) as f64);
        //         let d1 = (v1 - t).abs();
        //         let d2 = (v2 - t).abs();
        //         match d1.partial_cmp(&d2).unwrap() {
        //             Ordering::Less | Ordering::Equal => ans.push(cnt(ok).to_string()),
        //             _ => ans.push(cnt(ok + 1).to_string()),
        //         }
        //     }
        // }
    }
    echo!(ans.join("\n"));
}

fn cnt(ok: i64) -> i64 {
    2 * ok + 1
}
