use std::collections::HashMap;
use std::collections::HashSet;

use database::MapLink;
use database::MapPoint;

pub fn adjacent_links(
    links: &Vec<MapLink>,
    ignore_links: &HashSet<usize>,
) -> HashMap<MapPoint, HashSet<usize>> {
    let mut result = HashMap::default();

    for (index, link) in links.iter().enumerate() {
        let points = link.points();

        if !ignore_links.contains(&index) && points.len() > 1 {
            if let Some(point) = points.first() {
                result
                    .entry(point.clone())
                    .or_insert_with(|| HashSet::default())
                    .insert(index);
            }

            if let Some(point) = points.last() {
                result
                    .entry(point.clone())
                    .or_insert_with(|| HashSet::default())
                    .insert(index);
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::hash::Hash;

    use database::MapLink;
    use database::MapPoint;

    use super::adjacent_links;

    fn vec_to_set<T>(list: Vec<T>) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        list.into_iter().collect()
    }

    #[test]
    fn shoud_return_empty_map_when_links_empty() {
        let links = vec![];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert!(result.is_empty());
    }

    #[test]
    fn shoud_return_two_points_when_one_link_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let links = vec![MapLink::new(vec![point_1.clone(), point_2.clone()])];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_2), Some(&vec_to_set(vec![0])));
    }

    #[test]
    fn shoud_return_three_points_when_two_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let links = vec![
            MapLink::new(vec![point_1.clone(), point_2.clone()]),
            MapLink::new(vec![point_2.clone(), point_3.clone()]),
        ];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_2), Some(&vec_to_set(vec![0, 1])));
        assert_eq!(result.get(&point_3), Some(&vec_to_set(vec![1])));
    }

    #[test]
    fn shoud_return_three_points_when_cyclyc_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let links = vec![
            MapLink::new(vec![point_1.clone(), point_2.clone()]),
            MapLink::new(vec![point_2.clone(), point_3.clone()]),
            MapLink::new(vec![point_3.clone(), point_1.clone()]),
        ];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![2, 0])));
        assert_eq!(result.get(&point_2), Some(&vec_to_set(vec![0, 1])));
        assert_eq!(result.get(&point_3), Some(&vec_to_set(vec![1, 2])));
    }

    #[test]
    fn shoud_return_two_points_when_long_link_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let links = vec![
            MapLink::new(vec![point_1.clone(), point_2.clone(), point_3.clone()]),
        ];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_2), None);
        assert_eq!(result.get(&point_3), Some(&vec_to_set(vec![0])));
    }

    #[test]
    fn shoud_return_empty_map_when_single_point_link_given() {
        let point = MapPoint::new(100, 100);
        let links = vec![MapLink::new(vec![point.clone()])];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert!(result.is_empty());
    }

    #[test]
    fn shoud_return_empty_map_when_empty_link_given() {
        let links = vec![MapLink::new(vec![])];
        let result = adjacent_links(&links, &vec_to_set(vec![]));

        assert!(result.is_empty());
    }

    #[test]
    fn shoud_return_empty_map_when_link_ignored_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let links = vec![MapLink::new(vec![point_1.clone(), point_2.clone()])];
        let result = adjacent_links(&links, &vec_to_set(vec![0]));

        assert!(result.is_empty());
    }

    #[test]
    fn shoud_return_three_points_when_one_link_ignored_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let links = vec![
            MapLink::new(vec![point_1.clone(), point_2.clone()]),
            MapLink::new(vec![point_2.clone(), point_3.clone()]),
            MapLink::new(vec![point_3.clone(), point_1.clone()]),
        ];
        let result = adjacent_links(&links, &vec_to_set(vec![1]));

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![2, 0])));
        assert_eq!(result.get(&point_2), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_3), Some(&vec_to_set(vec![2])));
    }

    #[test]
    fn shoud_return_two_points_when_two_links_ignored_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let links = vec![
            MapLink::new(vec![point_1.clone(), point_2.clone()]),
            MapLink::new(vec![point_2.clone(), point_3.clone()]),
            MapLink::new(vec![point_3.clone(), point_1.clone()]),
        ];
        let result = adjacent_links(&links, &vec_to_set(vec![1, 2]));

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&point_1), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_2), Some(&vec_to_set(vec![0])));
        assert_eq!(result.get(&point_3), None);
    }
}
