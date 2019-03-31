use super::adjacent_links;
use super::loop_link;
use super::next_link;
use crate::database::MapLink;
use crate::database::MapPoint;
use std::collections::HashSet;

pub fn collect_polygon(links: &Vec<MapLink>) -> Option<Vec<MapPoint>> {
    if links.len() < 2 {
        return None;
    }

    let mut used_links = HashSet::default();
    let adjacent = adjacent_links(links, &used_links);

    if adjacent.values().any(|links| links.len() != 2) {
        return None;
    }

    let mut result = Vec::default();

    if let Some((start_point, start_index)) = loop_link(&adjacent) {
        let mut link_index = start_index;
        let mut link_point = start_point;

        result.push(link_point.clone());

        loop {
            used_links.insert(link_index);

            let points = links[link_index].points();
            let first_point = points.first().unwrap();
            let last_point = points.last().unwrap();
            let next_index;

            if link_point == first_point {
                result.extend(points.iter().skip(1).cloned());

                next_index = next_link(&adjacent, last_point, link_index);
                link_point = last_point;
            } else if link_point == last_point {
                result.extend(points.iter().rev().skip(1).cloned());

                next_index = next_link(&adjacent, first_point, link_index);
                link_point = first_point;
            } else {
                next_index = None;
            }

            if let Some(next_index) = next_index {
                link_index = next_index;
            } else {
                unreachable!();
            }

            if start_point == link_point {
                break;
            }
        }
    }

    if used_links.len() == links.len() {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::collect_polygon;
    use crate::database::MapLink;
    use crate::database::MapPoint;

    fn bounded_with(link: &Vec<MapPoint>, a: &MapPoint, b: &MapPoint) -> bool {
        match (link.first(), link.last()) {
            (Some(first), Some(last)) if first == a && last == b => true,
            (Some(first), Some(last)) if first == b && last == a => true,
            _ => false,
        }
    }

    fn assert_link(actual_link: &Vec<MapPoint>, expected_link: &Vec<MapPoint>) {
        let found_fwd = expected_link
            .iter()
            .zip(actual_link.iter())
            .all(|(expected_point, actual_point)| expected_point == actual_point);
        let found_rev = expected_link
            .iter()
            .zip(actual_link.iter().rev())
            .all(|(expected_point, actual_point)| expected_point == actual_point);

        assert!(found_fwd || found_rev);
    }

    #[test]
    fn should_return_none_when_empty_link_given() {
        let links = vec![];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_none_when_one_link_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let links = vec![MapLink::new(link_1.clone())];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_none_when_two_separated_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_4.clone(), point_5.clone()];
        let links = vec![MapLink::new(link_1.clone()), MapLink::new(link_2.clone())];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_none_when_two_adjacent_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_3.clone(), point_4.clone(), point_5.clone()];
        let links = vec![MapLink::new(link_1.clone()), MapLink::new(link_2.clone())];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_none_when_three_adjacent_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let point_6 = MapPoint::new(600, 100);
        let point_7 = MapPoint::new(700, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_3.clone(), point_4.clone(), point_5.clone()];
        let link_3 = vec![point_5.clone(), point_6.clone(), point_7.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
        ];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_single_loop_when_two_looped_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_3.clone(), point_4.clone(), point_1.clone()];
        let links = vec![MapLink::new(link_1.clone()), MapLink::new(link_2.clone())];
        let result = collect_polygon(&links);

        assert!(result.is_some());

        let result = result.unwrap();
        let link;

        if bounded_with(&result, &point_1, &point_1) {
            link = vec![
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
                point_1.clone(),
            ];
        } else if bounded_with(&result, &point_3, &point_3) {
            link = vec![
                point_3.clone(),
                point_4.clone(),
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
            ];
        } else {
            unreachable!();
        }

        assert_link(&result, &link);
    }

    #[test]
    fn should_return_single_loop_when_three_looped_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let point_6 = MapPoint::new(600, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_3.clone(), point_4.clone(), point_5.clone()];
        let link_3 = vec![point_5.clone(), point_6.clone(), point_1.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
        ];
        let result = collect_polygon(&links);

        assert!(result.is_some());

        let result = result.unwrap();
        let link;

        if bounded_with(&result, &point_1, &point_1) {
            link = vec![
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
                point_5.clone(),
                point_6.clone(),
                point_1.clone(),
            ];
        } else if bounded_with(&result, &point_3, &point_3) {
            link = vec![
                point_3.clone(),
                point_4.clone(),
                point_5.clone(),
                point_6.clone(),
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
            ];
        } else if bounded_with(&result, &point_5, &point_5) {
            link = vec![
                point_5.clone(),
                point_6.clone(),
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
                point_5.clone(),
            ];
        } else {
            unreachable!();
        }

        assert_link(&result, &link);
    }

    #[test]
    fn should_return_none_when_three_loop_and_link_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let link_1 = vec![point_1.clone(), point_2.clone()];
        let link_2 = vec![point_2.clone(), point_3.clone()];
        let link_3 = vec![point_3.clone(), point_4.clone()];
        let link_4 = vec![point_4.clone(), point_2.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
            MapLink::new(link_4.clone()),
        ];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_none_when_two_loops_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let point_6 = MapPoint::new(600, 100);
        let link_1 = vec![point_1.clone(), point_2.clone()];
        let link_2 = vec![point_2.clone(), point_3.clone()];
        let link_3 = vec![point_3.clone(), point_1.clone()];
        let link_4 = vec![point_4.clone(), point_5.clone()];
        let link_5 = vec![point_5.clone(), point_6.clone()];
        let link_6 = vec![point_6.clone(), point_4.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
            MapLink::new(link_4.clone()),
            MapLink::new(link_5.clone()),
            MapLink::new(link_6.clone()),
        ];
        let result = collect_polygon(&links);

        assert_eq!(result, None);
    }
}
