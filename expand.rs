impl Vector<(u32, u32)> {
    fn length(&self) -> f32 {
        let coords = &self.0;
        let squares = {
            let f_coords = coords.0 as u32;
            f_coords * f_coords
        } + {
            let f_coords = coords.1 as u32;
            f_coords * f_coords
        };
        (squares as f32).sqrt()
    }
}
impl Vector<(u32, u32, u32)> {
    fn length(&self) -> f32 {
        let coords = &self.0;
        let squares = {
            let f_coords = coords.0 as u32;
            f_coords * f_coords
        } + {
            let f_coords = coords.1 as u32;
            f_coords * f_coords
        } + {
            let f_coords = coords.2 as u32;
            f_coords * f_coords
        };
        (squares as f32).sqrt()
    }
}
