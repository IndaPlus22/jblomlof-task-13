/*
Fresnel equation
for calculating mix of refraction and reflection */


/*
Now instead try to make shadow edges smoother, maybe check
how "far away" they are from actually making it to the lightsource
'

SOLVED :CHECKMARK:
It works better now, actually shades some what, however when the light source is far far away it seems to make everything bright
Dont know why and its weird, figure out why.
SOLVED :CHECKMARK:

*/

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const PIC_SIZE: (usize, usize) = (WIDTH, HEIGHT);
const CAMERA_POS: (f64, f64, f64) = (0.0, 0.0, -200.0);
const IMAGE_DISTANCE_FROM_CAMERA: f64 = 50.0;
const IMAGE_DIRECTION: (f64, f64, f64) = (0.0, 0.0, 1.0); // do not change, this is hard coded which direction the camera will face.
                                                          // please have this normalized, its fine if its not but just makes it easier
const PIXEL_SIZE: f64 = 0.01;
pub const LIGHT_SOURCE: (f64, f64, f64) = (100.0, 60.0, -50.0);

use std::fs::File;
use std::io::Write;
use std::path::Path;

use ray::Ray;
use world::Sphere;

mod ray;
mod world;
fn main() {
    /*let ray = ray::Ray::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let sphere = world::Sphere {
        radius: 2.0,
        centre: [0.0, 0.0, 0.0],
        colour: [1.0, 1.0, 1.0],
    };
    println!(
        "{}, {}",
        ray.length_of_vec(),
        sphere.intersecting_distance(&ray).0
    );*/

    render(&mut get_file("picture.ppm"));
}

///create a new file and write the header information
fn get_file(file_name: &str) -> File {
    let path = Path::new(file_name);
    let mut file = File::create(path).expect("FUCK FUCK, CANT OPEN FILE");
    //inspired from Eskil's raytracer that was provided in the task.
    file.write(format!("P3 {} {} 255\n", PIC_SIZE.0, PIC_SIZE.1).as_bytes())
        .expect("FUCK I CANT WRITE");
    file
}

fn render(file: &mut File) {
    let mut sphere_obj: Vec<Sphere> = vec![];
    
    //floor ball - a sphere as floor.
    sphere_obj.push(Sphere {
        radius: 500.0,
        centre: [0.0, -400.0, 700.0],
        colour: [0.3, 0.1, 0.2],
    });

    sphere_obj.push(Sphere {
        radius: 10.5,
        centre: [0.0, 0.0, 200.0],
        colour: [1.0, 0.0, 0.0],
    });
    sphere_obj.push(Sphere {
        radius: 19.5,
        centre: [-30.1, -10.0, 200.0],
        colour: [0.0, 1.0, 0.0],
    });
        sphere_obj.push(Sphere {
        radius: 7.5,
        centre: [10.1, 15.0, 100.0],
        colour: [0.0, 0.5, 0.5],
    });

    let mut shoot_rays: Vec<Ray> = vec![];
    let x_offset = PIXEL_SIZE * PIC_SIZE.0 as f64 / 2.0;
    let y_offset = PIXEL_SIZE * PIC_SIZE.1 as f64 / 2.0;
    for y in (0..PIC_SIZE.0).rev() {
        for x in 0..PIC_SIZE.1 {
            let x_component = x as f64 * PIXEL_SIZE - x_offset;
            let y_component =  y as f64 * PIXEL_SIZE - y_offset;
            let z_component = IMAGE_DIRECTION.2 * IMAGE_DISTANCE_FROM_CAMERA;
            //println!("{} {} {}", x_component, y_component, z_component);
            shoot_rays.push(Ray::new(
                x_component,
                y_component,
                z_component,
                CAMERA_POS.0,
                CAMERA_POS.1,
                CAMERA_POS.2,
            ))
        }
    }

    for mut _ray in shoot_rays {
        let mut closest = (0, 0.0);
        let mut flag = true;
        for (i, obj) in sphere_obj.iter().enumerate() {
            let t_value = obj.intersecting_distance(&_ray).0;
            //println!("{}", t_value);
            if t_value > 0.0 {
                //println!("debug");
                if flag || (t_value < closest.1) {
                    flag = false;
                    closest.0 = i;
                    closest.1 = t_value
                }
            }
        }
        if closest.1 > 0.0 {
            _ray = _ray.bounce_to_light(closest.1, sphere_obj.get(closest.0).unwrap().colour);
            for obj in &sphere_obj {
                let intersects = obj.intersecting_distance(&_ray).1;
                if intersects {
                    _ray.colour_value = [0.0, 0.0, 0.0];
                    break;
                }
            }
        }
        let mut inverse_square = 1.0;
        if _ray.distance_travelled > 100.0 {
            inverse_square = 1.0 / (_ray.distance_travelled/100.0).sqrt();
        }
        if inverse_square != 1.0 {
        
       // println!("{}", inverse_square);
        }
        _ray.colour_value[0] *= 255.0;
        _ray.colour_value[0] *= inverse_square;
        _ray.colour_value[0] = _ray.colour_value[0].round();
        _ray.colour_value[1] *= 255.0;
        _ray.colour_value[1] *= inverse_square;
        _ray.colour_value[1] = _ray.colour_value[1].round();
        _ray.colour_value[2] *= 255.0;
        _ray.colour_value[2] *= inverse_square;
        _ray.colour_value[2] = _ray.colour_value[2].round();

        file.write(
            format!(
                "{} {} {} \n",
                _ray.colour_value[0], _ray.colour_value[1], _ray.colour_value[2]
            )
            .as_bytes(),
        ).expect("Couldn't write");
    }
}
