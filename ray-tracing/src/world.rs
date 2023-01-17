use crate::ray::Ray;

const ERROR_MARGIN: f64 = 0.005;
//Setting this to > 0.1 helps a lot, i guess its not accurate enough to deal with
// > 0.0 or > 0.000001 altough those would be mathematical correct.
// I am now experementing with the value.
// lower is better mathematical.
// > 0.001 not working correctly.

/*pub struct Plane {
    pub centre: [f64; 3],

    //if theese vectors are not orthogonal we wont get a plane with 90 deg angles, however
    //this might not be a problem, the plane's centre might just not be the centre.
    // i could use gramm-schmidt to solve that problem. 
    pub vector_span: ([f64; 3], [f64; 3]),  //a structure in which .0 is the first vector and .1 is the second vector
                                            // each vector is simply an array of f64's
                                            // but they also works as a stop
                                            // meaning a point on the plane can be written as
                                            // centre + t*vector_span.0 + s*vector_span.1
                                            // with the addition that -1 <= t <= 1
                                            // and for s also         -1 <= s <= 1
    pub colour: [f64; 3]
}

impl Plane {
    /// Using lin_alg maths to compute the scalar in which the passed vector ends at the plane.
    pub fn intersecting_distance(&self, _ray: &Ray) -> (f64, bool) {
        //we are interesting in finding if there exists a solution to 
        /*
         c_1*vector_span.0 + c_2*vector_span.1 = t*_ray
         such that t >= 0, abs(c) <= 1, abs(s) <= 1
        Its the same as solving A * c = 0
        where the columns of A is the vectors and c is the constants eg.
        [ v_1 u_1 w_1      [ c_1
          v_2 u_2 w_2   *    c_2 = 0
          v_3 u_3 w_3]       t ]
         where v = vector_span.0, u = vector_span.1 , w = _ray

         we can also calculate the closest distance between _ray and v and then see if the distance is within the distance of u

          */
    }
}*/

pub struct Sphere {
    pub radius: f64,
    pub centre: [f64; 3],
    pub colour: [f64; 3],
}

impl Sphere {

    /// if there is an intersection between the array and the sphere, return scalar in which the passed ray achieves the end point on the edge of the sphere.
    /// If not returns -1/or negative.
    /// Also returns true of false depending if it was valid at all
    /// Returns a tuple (f64, bool) where the f64 is the scalar, if scalar is < 0 assume it doesnt intersects.
    /// and the bool is used for for checking if it intersects at all.
    pub fn intersecting_distance(&self, _ray: &Ray) -> (f64, bool) {
        //this is the vector_from_origin
        let (x,y,z) = (_ray.vector_direction[0], _ray.vector_direction[1], _ray.vector_direction[2]);
        
        let (a,b,c) = (self.centre[0] - _ray.origin_coordinates[0], self.centre[1] - _ray.origin_coordinates[1], self.centre[2] - _ray.origin_coordinates[2]);
        
        //now we need to find if ||vector_from_origin... - t*_ray.vector_direction|| <= radius for any t.
        //but we can we know that if that has a solution, so does -||-... = radius.
        //so calculate that and using P-Q:formula we get the first root. 
        if x == 0.0 && y == 0.0 && z == 0.0 {
            panic!("How could this happen?!?!? The vector is somehow 0?");
        }
        let negative_p_half = (a*x + b*y + c*z)/(x*x + y*y + z*z);

        //maths
        let the_sq_root = (negative_p_half*negative_p_half - (a*a + b*b + c*c - self.radius*self.radius)/(x*x + y*y + z*z)).sqrt();
        if the_sq_root.is_nan() { // doesnt intersect
            return (-1.0, false);
        }
        let t = negative_p_half - the_sq_root;
        /*if t == 0.0 {
            //ignore this since we are already on the edge.
            return -1.0
        }*/
        //if its negative thats good. That would indicate we are in the wrong direction. 
        //we return the scalar so we can compute how much we needs to extend the given ray to intersect.
        //also returns a true or false if the second intersect( the one that happens on the other side) is within an error margin
        //because otherwise there is a mathbug. 
        (t, (negative_p_half + the_sq_root) > ERROR_MARGIN)
                                        
    }
}