use super::adjacent_links;
use super::loop_link;
use super::next_link;
use super::starting_link;
use crate::database::MapLink;
use crate::database::MapPoint;
use std::collections::HashSet;

pub fn collect_lines(links: &Vec<MapLink>) -> Vec<Vec<MapPoint>> {
    let mut used_links = HashSet::default();
    let mut result = Vec::default();

    loop {
        let adjacent = adjacent_links(links, &used_links);

        if let Some((start_point, start_index)) = starting_link(&adjacent) {
            let mut result_line = Vec::default();
            let mut link_index = start_index;
            let mut link_point = start_point;

            result_line.push(link_point.clone());

            loop {
                used_links.insert(link_index);

                let points = links[link_index].points();
                let first_point = points.first().unwrap();
                let last_point = points.last().unwrap();
                let next_index;

                if link_point == first_point {
                    result_line.extend(points.iter().skip(1).cloned());

                    next_index = next_link(&adjacent, last_point, link_index);
                    link_point = last_point;
                } else if link_point == last_point {
                    result_line.extend(points.iter().rev().skip(1).cloned());

                    next_index = next_link(&adjacent, first_point, link_index);
                    link_point = first_point;
                } else {
                    next_index = None;
                }

                if let Some(next_index) = next_index {
                    link_index = next_index;
                } else {
                    break;
                }
            }

            result.push(result_line);
        } else {
            break;
        }
    }

    loop {
        let adjacent = adjacent_links(links, &used_links);

        if let Some((start_point, start_index)) = loop_link(&adjacent) {
            let mut result_line = Vec::default();
            let mut link_index = start_index;
            let mut link_point = start_point;

            result_line.push(link_point.clone());

            loop {
                used_links.insert(link_index);

                let points = links[link_index].points();
                let first_point = points.first().unwrap();
                let last_point = points.last().unwrap();
                let next_index;

                if link_point == first_point {
                    result_line.extend(points.iter().skip(1).cloned());

                    next_index = next_link(&adjacent, last_point, link_index);
                    link_point = last_point;
                } else if link_point == last_point {
                    result_line.extend(points.iter().rev().skip(1).cloned());

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

            result.push(result_line);
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::collect_lines;
    use crate::database::MapLink;
    use crate::database::MapPoint;

    fn bounded_with(link: &Vec<MapPoint>, a: &MapPoint, b: &MapPoint) -> bool {
        match (link.first(), link.last()) {
            (Some(first), Some(last)) if first == a && last == b => true,
            (Some(first), Some(last)) if first == b && last == a => true,
            _ => false,
        }
    }

    fn contains(link: &Vec<MapPoint>, point: &MapPoint) -> bool {
        link.iter().any(|p| p == point)
    }

    fn assert_link(actual_links: &Vec<Vec<MapPoint>>, expected_link: &Vec<MapPoint>) {
        let mut found = false;

        for actual_link in actual_links {
            if expected_link
                .iter()
                .zip(actual_link.iter())
                .all(|(expected_point, actual_point)| expected_point == actual_point)
            {
                found = true;

                break;
            }

            if expected_link
                .iter()
                .zip(actual_link.iter().rev())
                .all(|(expected_point, actual_point)| expected_point == actual_point)
            {
                found = true;

                break;
            }
        }

        assert!(found);
    }

    #[test]
    fn should_return_empty_link_when_empty_link_given() {
        let links = vec![];
        let result = collect_lines(&links);

        assert!(result.is_empty());
    }

    #[test]
    fn should_return_single_link_when_one_link_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let links = vec![MapLink::new(link_1.clone())];
        let result = collect_lines(&links);

        assert_eq!(result.len(), 1);
        assert_link(&result, &link_1);
    }

    #[test]
    fn should_return_two_link_when_two_separated_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_4.clone(), point_5.clone()];
        let links = vec![MapLink::new(link_1.clone()), MapLink::new(link_2.clone())];
        let result = collect_lines(&links);

        assert_eq!(result.len(), 2);
        assert_link(&result, &link_1);
        assert_link(&result, &link_2);
    }

    #[test]
    fn should_return_long_link_when_two_adjacent_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_3.clone(), point_4.clone(), point_5.clone()];
        let links = vec![MapLink::new(link_1.clone()), MapLink::new(link_2.clone())];
        let result = collect_lines(&links);

        let link_long = vec![
            point_1.clone(),
            point_2.clone(),
            point_3.clone(),
            point_4.clone(),
            point_5.clone(),
        ];

        assert_eq!(result.len(), 1);
        assert_link(&result, &link_long);
    }

    #[test]
    fn should_return_long_link_when_three_adjacent_links_given() {
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
        let result = collect_lines(&links);

        let link_long = vec![
            point_1.clone(),
            point_2.clone(),
            point_3.clone(),
            point_4.clone(),
            point_5.clone(),
            point_6.clone(),
            point_7.clone(),
        ];

        assert_eq!(result.len(), 1);
        assert_link(&result, &link_long);
    }

    #[test]
    fn should_return_three_links_when_three_star_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let point_5 = MapPoint::new(500, 100);
        let point_6 = MapPoint::new(600, 100);
        let point_7 = MapPoint::new(700, 100);
        let link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
        let link_2 = vec![point_1.clone(), point_4.clone(), point_5.clone()];
        let link_3 = vec![point_1.clone(), point_6.clone(), point_7.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
        ];
        let result = collect_lines(&links);

        assert_eq!(result.len(), 2);

        let link_1;
        let link_2;

        if bounded_with(&result[0], &point_1, &point_3) {
            link_1 = vec![point_1.clone(), point_2.clone(), point_3.clone()];
            link_2 = vec![
                point_5.clone(),
                point_4.clone(),
                point_1.clone(),
                point_6.clone(),
                point_7.clone(),
            ];
        } else if bounded_with(&result[0], &point_1, &point_5) {
            link_1 = vec![point_1.clone(), point_4.clone(), point_5.clone()];
            link_2 = vec![
                point_3.clone(),
                point_2.clone(),
                point_1.clone(),
                point_6.clone(),
                point_7.clone(),
            ];
        } else if bounded_with(&result[0], &point_1, &point_7) {
            link_1 = vec![point_1.clone(), point_6.clone(), point_7.clone()];
            link_2 = vec![
                point_3.clone(),
                point_2.clone(),
                point_1.clone(),
                point_4.clone(),
                point_5.clone(),
            ];
        } else {
            unreachable!();
        }

        assert_link(&result, &link_1);
        assert_link(&result, &link_2);
    }

    #[test]
    fn should_return_one_link_when_three_looped_links_given() {
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
        let result = collect_lines(&links);

        assert_eq!(result.len(), 1);

        let link;

        if bounded_with(&result[0], &point_1, &point_1) {
            link = vec![
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
                point_5.clone(),
                point_6.clone(),
                point_1.clone(),
            ];
        } else if bounded_with(&result[0], &point_3, &point_3) {
            link = vec![
                point_3.clone(),
                point_4.clone(),
                point_5.clone(),
                point_6.clone(),
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
            ];
        } else if bounded_with(&result[0], &point_5, &point_5) {
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
    fn should_return_two_links_when_pin_looped_links_given() {
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
        let result = collect_lines(&links);

        assert_eq!(result.len(), 2);

        let link_1 = vec![point_1.clone(), point_2.clone()];
        let link_2;

        if bounded_with(&result[0], &point_2, &point_2)
            || bounded_with(&result[1], &point_2, &point_2)
        {
            link_2 = vec![
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
                point_2.clone(),
            ];
        } else if bounded_with(&result[0], &point_3, &point_3)
            || bounded_with(&result[1], &point_3, &point_3)
        {
            link_2 = vec![
                point_3.clone(),
                point_4.clone(),
                point_2.clone(),
                point_3.clone(),
            ];
        } else if bounded_with(&result[0], &point_4, &point_4)
            || bounded_with(&result[1], &point_4, &point_4)
        {
            link_2 = vec![
                point_4.clone(),
                point_2.clone(),
                point_3.clone(),
                point_4.clone(),
            ];
        } else {
            unreachable!();
        }

        assert_link(&result, &link_1);
        assert_link(&result, &link_2);
    }

    #[test]
    fn should_return_two_links_when_double_looped_links_given() {
        let point_1 = MapPoint::new(100, 100);
        let point_2 = MapPoint::new(200, 100);
        let point_3 = MapPoint::new(300, 100);
        let point_4 = MapPoint::new(400, 100);
        let link_1 = vec![point_1.clone(), point_2.clone()];
        let link_2 = vec![point_1.clone(), point_3.clone()];
        let link_3 = vec![point_3.clone(), point_2.clone()];
        let link_4 = vec![point_1.clone(), point_4.clone()];
        let link_5 = vec![point_4.clone(), point_2.clone()];
        let links = vec![
            MapLink::new(link_1.clone()),
            MapLink::new(link_2.clone()),
            MapLink::new(link_3.clone()),
            MapLink::new(link_4.clone()),
            MapLink::new(link_5.clone()),
        ];
        let result = collect_lines(&links);

        assert_eq!(result.len(), 2);

        let link_1;
        let link_2;

        if result[0].len() == 2 && bounded_with(&result[1], &point_1, &point_1) {
            link_1 = vec![point_1.clone(), point_2.clone()];
            link_2 = vec![
                point_1.clone(),
                point_3.clone(),
                point_2.clone(),
                point_4.clone(),
                point_1.clone(),
            ];
        } else if result[0].len() == 2 && bounded_with(&result[1], &point_2, &point_2) {
            link_1 = vec![point_1.clone(), point_2.clone()];
            link_2 = vec![
                point_2.clone(),
                point_4.clone(),
                point_1.clone(),
                point_3.clone(),
                point_2.clone(),
            ];
        } else if result[0].len() == 2 && bounded_with(&result[1], &point_3, &point_3) {
            link_1 = vec![point_1.clone(), point_2.clone()];
            link_2 = vec![
                point_3.clone(),
                point_2.clone(),
                point_4.clone(),
                point_1.clone(),
                point_3.clone(),
            ];
        } else if result[0].len() == 2 && bounded_with(&result[1], &point_4, &point_4) {
            link_1 = vec![point_1.clone(), point_2.clone()];
            link_2 = vec![
                point_4.clone(),
                point_1.clone(),
                point_3.clone(),
                point_2.clone(),
                point_4.clone(),
            ];
        } else if contains(&result[0], &point_3) && bounded_with(&result[1], &point_1, &point_1) {
            link_1 = vec![point_1.clone(), point_3.clone(), point_2.clone()];
            link_2 = vec![
                point_1.clone(),
                point_2.clone(),
                point_4.clone(),
                point_1.clone(),
            ];
        } else if contains(&result[0], &point_3) && bounded_with(&result[1], &point_2, &point_2) {
            link_1 = vec![point_1.clone(), point_3.clone(), point_2.clone()];
            link_2 = vec![
                point_2.clone(),
                point_4.clone(),
                point_1.clone(),
                point_2.clone(),
            ];
        } else if contains(&result[0], &point_3) && bounded_with(&result[1], &point_4, &point_4) {
            link_1 = vec![point_1.clone(), point_3.clone(), point_2.clone()];
            link_2 = vec![
                point_4.clone(),
                point_1.clone(),
                point_2.clone(),
                point_4.clone(),
            ];
        } else if contains(&result[0], &point_4) && bounded_with(&result[1], &point_1, &point_1) {
            link_1 = vec![point_1.clone(), point_4.clone(), point_2.clone()];
            link_2 = vec![
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
                point_1.clone(),
            ];
        } else if contains(&result[0], &point_4) && bounded_with(&result[1], &point_2, &point_2) {
            link_1 = vec![point_1.clone(), point_4.clone(), point_2.clone()];
            link_2 = vec![
                point_2.clone(),
                point_3.clone(),
                point_1.clone(),
                point_2.clone(),
            ];
        } else if contains(&result[0], &point_4) && bounded_with(&result[1], &point_3, &point_3) {
            link_1 = vec![point_1.clone(), point_4.clone(), point_2.clone()];
            link_2 = vec![
                point_3.clone(),
                point_1.clone(),
                point_2.clone(),
                point_3.clone(),
            ];
        } else {
            println!("{:?}", result);

            unreachable!();
        }

        assert_link(&result, &link_1);
        assert_link(&result, &link_2);
    }
}
