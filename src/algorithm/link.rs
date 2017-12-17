use std::collections::HashMap;
use std::collections::HashSet;

use database::MapPoint;

pub fn starting_link(adjacent: &HashMap<MapPoint, HashSet<usize>>) -> Option<(&MapPoint, usize)> {
    for (point, links_indexes) in adjacent {
        if links_indexes.len() != 2 {
            if let Some(&index) = links_indexes.iter().next() {
                return Some((point, index));
            }
        }
    }

    None
}

pub fn loop_link(adjacent: &HashMap<MapPoint, HashSet<usize>>) -> Option<(&MapPoint, usize)> {
    for (point, links_indexes) in adjacent {
        if links_indexes.len() == 2 {
            if let Some(&index) = links_indexes.iter().next() {
                return Some((point, index));
            }
        }
    }

    None
}

pub fn next_link(
    adjacent: &HashMap<MapPoint, HashSet<usize>>,
    point: &MapPoint,
    link_index: usize,
) -> Option<usize> {
    if let Some(link_indexes) = adjacent.get(point) {
        if link_indexes.len() == 2 {
            link_indexes
                .iter()
                .cloned()
                .filter(|&index| index != link_index)
                .next()
        } else {
            None
        }
    } else {
        None
    }
}
