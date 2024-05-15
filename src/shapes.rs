use std::f32::consts::PI;


#[allow(dead_code)]
/// Generate points on a circle 
pub fn generate_circle(center_x:f32, center_y:f32, radius: f32, num_bullets: usize) -> Vec<(f32,f32)>{
    let mut vec = Vec::new();
    for i in 0..num_bullets {
        let angle = i as f32 / num_bullets as f32 * std::f32::consts::PI * 2.0;
        let bullet_x = center_x + radius * angle.cos();
        let bullet_y = center_y + radius * angle.sin();
        vec.push((bullet_x,bullet_y));
    }

    vec
}

#[allow(dead_code)]
pub fn generate_square(center_x: f32, center_y: f32, side_length: f32, num_bullets_per_side: usize) -> Vec<(f32,f32)> {
    let mut vec = Vec::new();
    
    let half = side_length / 2.0;
    let spacing = side_length / num_bullets_per_side as f32;

    for i in 0..num_bullets_per_side {
        let x = center_x - half + (i as f32 * spacing);
        let y_top = center_y + half;
        let y_bot: f32 = center_y - half;
        vec.push((x,y_top));
        vec.push((x,y_bot));
    }

    for i in 1..num_bullets_per_side {
        let y = center_y - half + (i as f32 * spacing);
        let x_right = center_x + half;
        let x_left: f32 = center_x - half;
        vec.push((x_right,y));
        vec.push((x_left,y));
    }
    vec
}

/// Generate a line providing start, and end points, as well as the total number of points(bullets)
pub fn generate_line(x1: f32, y1: f32, x2: f32, y2: f32, num_bullets: usize) -> Vec<(f32,f32)> {
    let mut vec = Vec::new();
    let dx = (x2 - x1) / num_bullets as f32;
    let dy = (y2-y1) / num_bullets as f32;

    for i in 0..num_bullets {
        let bx = x1 + (i as f32 * dx);
        let by = y1 + (i as f32 * dy);
        vec.push((bx,by));
    }
    vec
}

/// Generate a triangle based on 3  points ()
pub fn generate_triangle(p1: (f32,f32), p2: (f32,f32), p3: (f32, f32), num_bullets: usize) -> Result<Vec<(f32,f32)>, &'static str> {
    let mut vec: Vec<(f32, f32)> = Vec::new();
    
    
    let line_0 = generate_line(p1.0, p1.1, p2.0, p2.1, num_bullets);
    let line_1 = generate_line(p1.0, p1.1, p3.0, p3.1, num_bullets);
    let line_2 = generate_line(p3.0, p3.1, p2.0, p2.1, num_bullets);

    vec.extend(line_0);
    vec.extend(line_1);
    vec.extend(line_2);

    Ok(vec)
}



/* fn angle_between(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let angle = (dy).atan2(dx);

    let degrees = angle * 180. / PI;
    let degrees = degrees % 360.;
    let degrees = if degrees < 0. {degrees + 360.} else {degrees};

    degrees
} */