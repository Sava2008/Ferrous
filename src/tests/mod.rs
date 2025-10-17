mod board_tests;
mod converters_tests;

#[cfg(test)]
pub mod tests {

    use crate::{constants, helper_functions};
    use ggez::mint::Point2;
    #[test]
    fn coords_to_index_test() -> () {
        assert_eq!(
            0,
            helper_functions::coords_to_index(Point2 { x: 0., y: 0. }).unwrap()
        );
        assert_eq!(
            35,
            helper_functions::coords_to_index(Point2 {
                x: 3. * constants::SQUARE_SIDE,
                y: 4. * constants::SQUARE_SIDE
            })
            .unwrap()
        );
        assert_eq!(
            63,
            helper_functions::coords_to_index(Point2 {
                x: 7. * constants::SQUARE_SIDE,
                y: 7. * constants::SQUARE_SIDE
            })
            .unwrap()
        );
    }

    #[test]
    fn index_to_coords_test() -> () {
        assert_eq!((4, 3), helper_functions::index_to_coords(35));
        assert_eq!((1, 6), helper_functions::index_to_coords(14));
        assert_eq!((6, 5), helper_functions::index_to_coords(53));
    }

    #[test]
    fn i8_coords_to_index_test() -> () {
        assert_eq!(7, helper_functions::i8_coords_to_index((0, 7)));
        assert_eq!(9, helper_functions::i8_coords_to_index((1, 1)));
        assert_eq!(37, helper_functions::i8_coords_to_index((4, 5)));
    }

    #[test]
    fn is_line_test() -> () {
        assert!(helper_functions::is_line(26, 29));
        assert!(helper_functions::is_line(56, 63));
        assert!(helper_functions::is_line(11, 43));
        assert!(!helper_functions::is_line(1, 30));
        assert!(!helper_functions::is_line(36, 40));
        assert!(!helper_functions::is_line(18, 27));
    }

    #[test]
    fn is_diagonal_test() -> () {
        assert!(helper_functions::is_diagonal(0, 63));
        assert!(helper_functions::is_diagonal(6, 20));
        assert!(helper_functions::is_diagonal(43, 16));
        assert!(!helper_functions::is_diagonal(10, 12));
        assert!(!helper_functions::is_diagonal(23, 41));
        assert!(!helper_functions::is_diagonal(60, 52));
    }

    #[test]
    fn is_adjancent_file_test() -> () {
        assert!(helper_functions::is_adjancent_file(33, 2));
        assert!(helper_functions::is_adjancent_file(61, 62));
        assert!(helper_functions::is_adjancent_file(3, 11));
        assert!(!helper_functions::is_adjancent_file(5, 7));
        assert!(!helper_functions::is_adjancent_file(39, 40));
        assert!(!helper_functions::is_adjancent_file(57, 44));
    }
}
