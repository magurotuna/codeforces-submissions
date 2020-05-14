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
    let t = get!(usize);
    let mut ans = Vec::with_capacity(t);
    for _ in 0..t {
        let n = get!(usize);
        if n == 1 {
            ans.push("1".to_string());
            continue;
        }
        if n == 2 {
            ans.push("1 2".to_string());
            continue;
        }
        let mut tans = vec![0i32; n];
        let mut len_heap = BinaryHeap::new();
        len_heap.push((n, 0, n));
        dfs2(&mut tans, &mut len_heap, 1);
        //dfs(&mut tans, 1, false);
        debug!(tans);
        let mut cnt = vec![0i32; n];
        for i in 0..n {
            cnt[tans[i] as usize] += 1;
        }
        debug!(cnt);
        let mut cumsum = vec![0i32; n + 10];
        for i in 0..n {
            cumsum[i + 1] = cumsum[i] + cnt[i];
        }
        for i in 0..n {
            cnt[i] = cumsum[i] + 1;
        }
        debug!(cnt);
        for i in 0..n {
            let prev = tans[i] as usize;
            tans[i] = cnt[prev];
            cnt[prev] += 1;
        }
        debug!(tans);

        ans.push(
            tans.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    echo!(ans.join("\n"));
}

fn dfs2(vec: &mut [i32], heap: &mut BinaryHeap<(usize, usize, usize)>, cur: i32) {
    if heap.is_empty() {
        return;
    }

    let (len, from, to) = heap.pop().unwrap();
    debug!(len, from, to);
    vec[(to - from - 1) / 2 + from] = cur;
    if len == 1 {
        ()
    } else if len == 2 {
        vec[(to - from - 1) / 2 + 1 + from] = cur + 1;
    } else if len % 2 == 0 {
        let mid = (to - from) / 2 + from;
        heap.push(((len - 1) / 2, from, mid - 1));
        heap.push((len / 2, mid, to));
    } else {
        let mid = (to - from) / 2 + from;
        heap.push((len / 2, from, mid));
        heap.push((len / 2, mid + 1, to));
    }
    while let Some(&(l, f, t)) = heap.peek() {
        if len != l {
            break;
        }
        let len = l;
        let from = f;
        let to = t;
        heap.pop();
        debug!(len, from, to);
        vec[(to - from - 1) / 2 + from] = cur;
        if len == 1 {
            ()
        } else if len == 2 {
            vec[(to - from - 1) / 2 + 1 + from] = cur + 1;
        } else if len % 2 == 0 {
            let mid = (to - from) / 2 + from;
            heap.push(((len - 1) / 2, from, mid - 1));
            heap.push((len / 2, mid, to));
        } else {
            let mid = (to - from) / 2 + from;
            heap.push((len / 2, from, mid));
            heap.push((len / 2, mid + 1, to));
        }
    }
    dfs2(vec, heap, cur + 1);
}

fn dfs(vec: &mut [i32], cur: i32, flg: bool) {
    let len = vec.len();
    if len == 1 {
        vec[0] = cur;
        return;
    }
    if len == 2 {
        vec[0] = cur;
        vec[1] = cur + 1;
        return;
    }

    if len % 2 == 0 {
        let piv = (len - 1) / 2;
        vec[piv] = cur;
        dfs(&mut vec[..piv], cur + 2, false);
        dfs(&mut vec[(piv + 1)..], cur + 1, (len - piv + 1) % 2 != 0);
    } else {
        let piv = len / 2;
        vec[piv] = cur;
        let next = if flg { cur + 2 } else { cur + 1 };
        dfs(&mut vec[..piv], next, false);
        dfs(&mut vec[(piv + 1)..], next, false);
    }
}
