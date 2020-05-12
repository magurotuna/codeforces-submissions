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
    let n = get!(usize);
    let a = get!([i64]);
    let mut cum_gcd = vec![0; n + 1];
    let mut rev_gcd = vec![0; n + 1];
    for i in 0..n {
        cum_gcd[i + 1] = gcd(a[i], cum_gcd[i]);
    }
    for i in (0..n).rev() {
        rev_gcd[i] = gcd(a[i], rev_gcd[i + 1]);
    }
    let ans = (0..n)
        .map(|i| {
            let other_gcd = gcd(cum_gcd[i], rev_gcd[i + 1]);
            lcm(a[i], other_gcd)
        })
        .fold(0, |acc, cur| gcd(acc, cur));
    echo!(ans);
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Int,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Int,
{
    a / gcd(a, b) * b
}

fn factorize(num: i64, primes: &[i64]) -> std::collections::HashMap<i64, usize> {
    let mut ret = std::collections::HashMap::new();
    let mut tmp = num;
    for &prime in primes {
        if prime > num {
            break;
        }
        while tmp % prime == 0 {
            tmp = tmp / prime;
            *ret.entry(prime).or_insert(0) += 1;
        }
    }
    if tmp > 1 {
        *ret.entry(tmp).or_insert(0) += 1;
    }
    ret
}

pub trait Int:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::hash::Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
    fn next(self) -> Self;
    fn prev(self) -> Self;
    fn sqrt_floor(self) -> Self;
}
macro_rules !impl_int_for_numerics {($($t :ty ) *) =>($(impl Int for $t {fn zero () ->Self {0 } fn one () ->Self {1 } fn next (self ) ->Self {self +Self ::one () } fn prev (self ) ->Self {self -Self ::one () } fn sqrt_floor (self ) ->Self {if self <Self ::zero () {return Self ::zero () ;} let two =Self ::one () .next () ;let mut ok =Self ::zero () ;let mut ng =self .next () ;while ng -ok >1 {let mid =(ng +ok ) /two ;if mid *mid <=self {ok =mid ;} else {ng =mid ;} } ok } } ) *) }
impl_int_for_numerics !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
pub trait Prime<T: Int> {
    fn lower_primes(&self) -> Vec<T>;
    fn factorize(&self) -> std::collections::HashMap<T, usize>;
}
impl<T> Prime<T> for T
where
    T: Int,
{
    /// エラトステネスの篩を用いてself以下の素数を求める
    /// 計算量: O(n log log n)
    fn lower_primes(&self) -> Vec<T> {
        let &this = self;
        let mut v = Vec::new();
        if this <= T::one() {
            return v;
        }
        let mut deque = std::collections::VecDeque::new();
        let mut t = T::one().next();
        while t <= this {
            deque.push_back(t);
            t = t.next();
        }
        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p * p <= this {
            deque = deque
                .iter()
                .filter(|&&x| x % p != T::zero())
                .copied()
                .collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }
    /// エラトステネスの篩を用いてselfを素因数分解する
    fn factorize(&self) -> std::collections::HashMap<T, usize> {
        let mut ret = std::collections::HashMap::new();
        let primes = self.sqrt_floor().lower_primes();
        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == T::zero() {
                tmp = tmp / prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > T::one() {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}
