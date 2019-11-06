
use std::io;

pub fn line_dis_formula() {

    // for 2nd point ===========================================================

    println!("\n    please Enter first abscissa and then ordinate respectively for Point_1(x1 , y1)\n          ");

    let mut x_1 = String::new();
    io::stdin().read_line(&mut x_1).expect("no x_1 value is given data");
    let x_1 = x_1.trim().parse::<f32>().unwrap();

    println!("\n        now ordinate please");

    let mut y_1 = String::new();
    io::stdin().read_line(&mut y_1).expect("no y_1 value is given data");
    let y_1 = y_1.trim().parse::<f32>().unwrap(); 

    let point_1 = Point { x : x_1, y : y_1};

    // for 2nd point  =============================================================

    println!("\n    please Enter first abscissa and then ordinate respectively for Point_2(x2 , y2)\n          ");

    let mut x_2 = String::new();
    io::stdin().read_line(&mut x_2).expect("no x_2 value is given data");
    let x_2 = x_2.trim().parse::<f32>().unwrap();

    println!("\n        now ordinate please");

    let mut y_2 = String::new();
    io::stdin().read_line(&mut y_2).expect("no y_2 value is given data");
    let y_2 = y_2.trim().parse::<f32>().unwrap(); 

    let point_2 = Point { x : x_2, y : y_2};

    // merhod distance is being called here

    let distance = point_1.distance(point_2);
    println!("\n          ========================================================= ");
    println!("\n          The distance between the points P_1 and P_2 is : {}\n          ", distance);
    println!("          ========================================================= ");

    pub struct Point<T>  {
        x : T,
        y : T
    }
    
    impl Point<f32> {
        pub fn distance(self, other : Point<f32>) -> f32 {
            return ((self.x-other.x).powi(2) + (self.y-other.y).powi(2)).sqrt(); 
        }
    }

    // pub fn ()
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
