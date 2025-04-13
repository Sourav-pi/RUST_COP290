use cores;
use cores::Sheet;
// #[test]
// fn test_avg() {
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("AVG(A2:D5)"));
//     test_sheet.update_cell_data(2,2, String::from("500"));
//     println!("{}", test_sheet.get_value(1, 1));
// }
// #[test]
// fn test_sum() {
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("SUM(A2:D5)"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1, 2, String::from("5"));
//     test_sheet.update_cell_data(3, 1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 52);

// }
// #[test]
// fn test_max() {
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("MAX(A2:D5)"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1, 2, String::from("-5"));
//     test_sheet.update_cell_data(3, 1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 11);
// }
// #[test]
// fn test_min(){
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("MIN(A2:D5)"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1,2, String::from("-5"));
//     test_sheet.update_cell_data(3,1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 0);
// }

// #[test]
// fn test_stdev(){
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("STDEV(A2:D5)"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1,2, String::from("-5"));
//     test_sheet.update_cell_data(3,1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 3);
// }

// #[test]
// fn test_multiply(){
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("A2*B2"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1,2, String::from("-5"));
//     test_sheet.update_cell_data(3,1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 55);
// }

// #[test]
// fn test_divide(){
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("A2/B2"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1,2, String::from("-5"));
//     test_sheet.update_cell_data(3,1, String::from("6"));
//     assert_eq!(test_sheet.get_value(1, 1), 0);
// }

// #[test]
// fn test_large_cell(){
//     let mut test_sheet = Sheet::new(703, 703);
//     test_sheet.update_cell_data(1,1, String::from("ZZ29"));
//     test_sheet.update_cell_data(29,702, String::from("29"));
//     println!("{}", test_sheet.get_value(1, 1));
//     assert!(test_sheet.get_value(1, 1) == 29);
// }
// #[test]
// fn check_cycle(){
//     let mut test_sheet = Sheet::new(6, 6); 
//     test_sheet.update_cell_data(1,1, String::from("A2"));
//     test_sheet.update_cell_data(2,1,String::from("B1+B5"));
//     test_sheet.update_cell_data(2,2,String::from("A2+A3"));
//     test_sheet.update_cell_data(5,2,String::from("10"));
//     test_sheet.update_cell_data(1,2, String::from("-5"));
//     test_sheet.update_cell_data(3,1, String::from("6"));
    
// }
#[test]
fn error_detected1(){
    let mut test_sheet = Sheet::new(6, 6); 
    test_sheet.update_cell_data(1,1, String::from("A2+A3"));
    test_sheet.update_cell_data(2,1,String::from("90"));
    test_sheet.update_cell_data(3,1,String::from("50"));
    test_sheet.update_cell_data(3,1,String::from("A1+A2"));
    test_sheet.update_cell_data(3,1, String::from("-5"));
    test_sheet.update_cell_data(3,1, String::from("6"));
    assert!(test_sheet.get_value(1, 1) == 96);
}
