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
macro_rules !chmin {($base :ident ,$($cmps :expr ) ,+$(,) *) =>{$base =min !($base ,$($cmps ) ,+) ;} ;}
#[macro_export]
macro_rules !chmax {($base :ident ,$($cmps :expr ) ,+$(,) *) =>{$base =max !($base ,$($cmps ) ,+) ;} ;}
#[macro_export]
macro_rules !min {($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::min ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::min ($a ,min !($($rest ) ,+) ) } } ;}
#[macro_export]
macro_rules !max {($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::max ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::max ($a ,max !($($rest ) ,+) ) } } ;}
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
    let n = get!(usize);
    let s = get!([i32]);
    let q = get!(usize);
    let lr = get!([(usize, usize); q]);
    let mut st1 = SegmentTree::new((0, 0), n, closure);
    let mut st2 = SegmentTree::new((0, 0), n.saturating_sub(2), closure);
    let mut st3 = SegmentTree::new((0, 0), n.saturating_sub(1), closure);
    let mut st4 = SegmentTree::new((0, 0), n.saturating_sub(3), closure);

    for i in 0..n {
        st1.update(i, (s[i], 0));
        if i >= 1 {
            st3.update(i - 1, (s[i], 0));
        }
        if i >= 2 {
            st2.update(i - 2, (s[i], 0));
        }
        if i >= 3 {
            st4.update(i - 3, (s[i], 0));
        }
    }

    let mut ans = Vec::with_capacity(q);

    for (l, r) in lr {
        let l = l - 1;
        let candy = match l % 4 {
            0 => st1.query(l, r).1,
            1 => st3.query(l - 1, r - 1).1,
            2 => st2.query(l - 2, r - 2).1,
            3 => st4.query(l - 3, r - 3).1,
            _ => unreachable!(),
        };
        ans.push(candy.to_string());
    }

    echo!(ans.join("\n"));
}

fn closure(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let mut candy = a.1 + b.1;
    if a.0 + b.0 >= 10 {
        candy += 1;
    }
    let new_val = (a.0 + b.0) % 10;
    (new_val, candy)
}

#[derive(Debug)]
pub struct SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    values: Vec<T>,
    n_leaves: usize,
    identity_elem: T,
    func: F,
}
impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(init_value: T, size: usize, func: F) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            values: vec![init_value; 2 * n - 1],
            n_leaves: n,
            identity_elem: init_value,
            func,
        }
    }
    pub fn update(&mut self, leaf_number: usize, replace_value: T) {
        let mut node_number = leaf_number + self.n_leaves - 1;
        self.values[node_number] = replace_value;
        while node_number > 0 {
            node_number = (node_number - 1) / 2;
            self.values[node_number] = (self.func)(
                self.values[node_number * 2 + 1],
                self.values[node_number * 2 + 2],
            );
        }
    }
    pub fn query(&self, begin: usize, end: usize) -> T {
        self.internal_query(begin, end, 0, 0, self.n_leaves)
    }
    fn internal_query(
        &self,
        begin: usize,
        end: usize,
        node_number: usize,
        left: usize,
        right: usize,
    ) -> T {
        if right <= begin || end <= left {
            self.identity_elem
        } else if begin <= left && right <= end {
            self.values[node_number]
        } else {
            let c1 = self.internal_query(begin, end, 2 * node_number + 1, left, (left + right) / 2);
            let c2 =
                self.internal_query(begin, end, 2 * node_number + 2, (left + right) / 2, right);
            (self.func)(c1, c2)
        }
    }
}
