
pub struct Ray {
    pub colour_value: [f64; 3],
    pub distance_travelled: f64, // used with inverse-square law to make "long travel light" appear more dim than "short travel light"
    pub vector_direction: [f64; 3], //3-dimension
    pub origin_coordinates: [f64; 3],
}

impl Ray {
    pub fn new(x_component: f64, y_component: f64, z_component: f64, x_start: f64, y_start: f64, z_start: f64 ) -> Ray {
        Ray {
            colour_value: [1.0, 1.0, 1.0],
            distance_travelled: 0.0,
            vector_direction: [x_component, y_component, z_component],
            origin_coordinates: [x_start, y_start, z_start],
        }
    }
    pub fn bounce(&mut self) -> Vec<Ray> {
        let new_vectors: Vec<Ray> = vec![];
        new_vectors 
    }

    
    pub fn bounce_to_light(&self, t_value: f64, _colour_change: [f64; 3]) -> Ray {
        let current_coord = [self.origin_coordinates[0] + self.vector_direction[0] * t_value,
                                        self.origin_coordinates[1] + self.vector_direction[1] * t_value,
                                        self.origin_coordinates[2] + self.vector_direction[2] * t_value];

        let new_vector = [crate::LIGHT_SOURCE.0 - current_coord[0],
                                    crate::LIGHT_SOURCE.1 - current_coord[1],
                                    crate::LIGHT_SOURCE.2 - current_coord[2]];
        Ray {
            colour_value: [self.colour_value[0] * _colour_change[0], self.colour_value[1] * _colour_change[1], self.colour_value[2] * _colour_change[2]],
            distance_travelled: self.distance_travelled + t_value * self.length_of_vec(),
            vector_direction: new_vector,
            origin_coordinates: current_coord,
        }
    }

    pub fn length_of_vec(&self) -> f64 {
        //yes its hard coded, deal with it
        //hmm should always be positive so no need to deal with NaN
        (self.vector_direction[0].powi(2) + self.vector_direction[1].powi(2) + self.vector_direction[2].powi(2)).sqrt()
    }
}
