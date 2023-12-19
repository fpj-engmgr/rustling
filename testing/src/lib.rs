pub struct Rectangle {
    width: f64,
    height: f64
}

pub struct Circle {
    radius: f64
}

pub enum Feature {
    Area,
    Perimeter
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {width, height}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {radius}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]     // Unit tests for Rectangle
    fn utst_rectangle_new() {  // new method verification
        let width  = 6.3;
        let height = 2.4;

        let tst_rect = Rectangle::new(width, height);
        assert_eq!(tst_rect.width,  width);
        assert_eq!(tst_rect.height, height);
    }

    #[test]     // Unit tests for Rectangle
    fn utst_rectangle_calc_area() {  // calc_area method verification
        let width  = 6.3;
        let height = 2.4;
        let exp_area = width * height;

        let tst_rect = Rectangle::new(width, height);
        let tst_area = tst_rect.calc_area();

        assert_eq!(exp_area, tst_area);
    }
    
    #[test]     // Unit tests for Rectangle
    fn utst_rectangle_calc_perimeter() {  // calc_perimeter method verification
        let width  = 6.3;
        let height = 2.4;
        let exp_perimeter = 2.0 * (width + height);

        let tst_rect = Rectangle::new(width, height);
        let tst_perimeter = tst_rect.calc_perimeter();

        assert_eq!(exp_perimeter, tst_perimeter);
    }
    
    #[test]     // Unit tests for Rectangle
    fn utst_rectangle_area_feature() {  // get_feature (Area) method verification
        let width  = 6.3;
        let height = 2.4;
        let exp_area = width * height;

        let tst_rect = Rectangle::new(width, height);
        let tst_area = tst_rect.get_feature(Feature::Area);

        assert_eq!(exp_area, tst_area);
    }
    
    #[test]     // Unit tests for Rectangle
    fn utst_rectangle_perimeter_feature() {  // get_feature (Perimeter) method verification
        let width  = 6.3;
        let height = 2.4;
        let exp_perimeter = 2.0 * (width + height);

        let tst_rect = Rectangle::new(width, height);
        let tst_perimeter = tst_rect.get_feature(Feature::Perimeter);

        assert_eq!(exp_perimeter, tst_perimeter);
    }
    
    #[test]     // Unit tests for Circle
    fn utst_circle_new() {  // new method verification
        let radius  = 6.3;

        let tst_circ = Circle::new(radius);
        assert_eq!(tst_circ.radius,  radius);
    }

    #[test]     // Unit tests for Circle
    fn utst_circle_calc_area() {  // calc_area method verification
        let radius  = 6.3;
        let exp_area = std::f64::consts::PI * radius * radius;

        let tst_circ = Circle::new(radius);
        let tst_area = tst_circ.calc_area();

        assert_eq!(exp_area, tst_area);
    }
    
    #[test]     // Unit tests for Circle
    fn utst_circle_calc_perimeter() {  // calc_perimeter method verification
        let radius  = 6.3;
        let exp_peri = 2.0 * std::f64::consts::PI * radius;

        let tst_circ = Circle::new(radius);
        let tst_peri = tst_circ.calc_perimeter();

        assert_eq!(exp_peri, tst_peri);
    }
     
    #[test]     // Unit tests for Circle
    fn utst_circle_area_feature() {  // get_feature (Area) method verification
        let radius  = 6.3;
        let exp_area = std::f64::consts::PI * radius * radius;

        let tst_circ = Circle::new(radius);
        let tst_area = tst_circ.get_feature(Feature::Area);

        assert_eq!(exp_area, tst_area);
    }
    
    #[test]     // Unit tests for Circle
    fn utst_circle_perimeter_feature() {  // get_feature (Perimeter) method verification
        let radius  = 6.3;
        let exp_peri = 2.0 * std::f64::consts::PI * radius;

        let tst_circ = Circle::new(radius);
        let tst_peri = tst_circ.get_feature(Feature::Perimeter);

        assert_eq!(exp_peri, tst_peri);
    }
    
}