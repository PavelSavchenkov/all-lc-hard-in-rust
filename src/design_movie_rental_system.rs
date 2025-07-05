//! Solution for https://leetcode.com/problems/design-movie-rental-system
//! 1912. Design Movie Rental System

use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Default)]
struct MovieRentingSystem {
    rented: BTreeSet<(i32, i32, i32)>, // price, shop, movie
    unrented_by_movie: HashMap<i32, BTreeSet<(i32, i32)>>, // movie -> {(price, shop)}
    movie_price_by_shop: HashMap<(i32, i32), i32>, // (shop, movie) -> price
}

const TOP: usize = 5;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut this = Self::default();
        for e in &entries {
            let shop = e[0];
            let movie = e[1];
            let price = e[2];
            this.unrented_by_movie
                .entry(movie)
                .or_insert(BTreeSet::new())
                .insert((price, shop));
            this.movie_price_by_shop.insert((shop, movie), price);
        }
        this
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let set = self.unrented_by_movie.get(&movie);
        if set.is_none() {
            return Vec::new();
        }
        let set = set.unwrap();
        for &(price, shop) in set {
            ans.push(shop);
            if ans.len() == TOP {
                break;
            }
        }
        ans
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.movie_price_by_shop.get(&(shop, movie)).unwrap();
        self.rented.insert((price, shop, movie));
        self.unrented_by_movie
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.movie_price_by_shop.get(&(shop, movie)).unwrap();
        self.rented.remove(&(price, shop, movie));
        self.unrented_by_movie
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for &(price, shop, movie) in &self.rented {
            ans.push(vec![shop, movie]);
            if ans.len() == TOP {
                break;
            }
        }
        ans
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
