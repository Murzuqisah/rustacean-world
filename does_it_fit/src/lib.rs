pub mod areas_volumes;
pub use areas_volumes as av;
pub use av::*;


pub fn area_fit(
	x: usize,
	y: usize,
	objects: av::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    match objects {
		av::GeometricalShapes::Square => {
			(av::square_area(a) * times) <= av::rectangle_area(x, y)
		}
		av::GeometricalShapes::Circle => {
			(av::circle_area(a) * times as f64) <= av::rectangle_area(x, y) as f64
		}
		av::GeometricalShapes::Rectangle => {
			(av::rectangle_area(a, b) * times) <= av::rectangle_area(x, y)
		}
		av::GeometricalShapes::Triangle => {
			(av::triangle_area(a, b) * times as f64) <= av::rectangle_area(x, y) as f64
		}
	}

}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: av::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    match objects {
		av::GeometricalVolumes::Cube => {
			(av::cube_volume(a) * times) <= av::parallelepiped_volume(x, y, z)
		}
		av::GeometricalVolumes::Sphere => {
			(av::sphere_volume(a) * times as f64) <= av::parallelepiped_volume(x, y, z) as f64
		}
		av::GeometricalVolumes::Cone => {
			(av::cone_volume(a, b) * times as f64) <= av::parallelepiped_volume(x, y, z) as f64
		}
		av::GeometricalVolumes::Pyramid => {
			(av::triangular_pyramid_volume(av::triangle_area(a, b), c) * times as f64) <= av::parallelepiped_volume(x, y, z) as f64
		}
		av::GeometricalVolumes::Parallelepiped => {
			(av::parallelepiped_volume(a, b, c) * times) <= av::parallelepiped_volume(x, y, z)
		}
	}

}