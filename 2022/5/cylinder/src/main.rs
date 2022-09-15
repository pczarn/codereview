use std::iter;

fn print_cylinder_vertices(
    radius_bottom: f32, 
    radius_top: f32, 
    height: f32, 
    radial_segments: u32
) {
    // The vertices and indices of the cylinder barrel.
    let mut verts = Vec::new();
    let mut inds = Vec::new();

    // Helper variables.
    let half_height = height / 2f32;

    // Calculate the slope so that the normals can be easily derived.
    let slope = (radius_bottom - radius_top) / height;

    let height_range = 0 ..= HEIGHT_SEGMENTS;
    let radial_segments_x_and_diff = (
        (0 .. radial_segments).zip(iter::repeat(0.5))
    ).chain(
        (1 .. radial_segments + 1).zip(iter::repeat(-0.5))
    );
    let get_radius = |y| y as f32 * (radius_bottom - radius_top) + radius_top;

    verts.extend(
        height_range.flat_map(|y| {
            radial_segments_x_and_diff.clone().zip(iter::repeat((y, get_radius(y))))
        }).map(|((x, normal_diff), (y, radius))| {
            let u = x as f32 / radial_segments as f32;
            let u1 = (x as f32 + normal_diff) / radial_segments as f32;

            let theta = u * THETA_END + THETA_START;
            let theta1 = u1 * THETA_END + THETA_START;

            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            let sin_theta1 = theta1.sin();
            let cos_theta1 = theta1.cos();

            Vertex {
                position: [
                    radius * sin_theta,
                    -(y as f32) * height + half_height,
                    radius * cos_theta,
                ],
                normal: [sin_theta1, slope, cos_theta1],
            }
        })
    );

    for i in 0..radial_segments {
        let a = i;
        let b = i + radial_segments;
        let c = i + radial_segments * 3;
        let d = i + radial_segments * 2;

        // The first triangle of the radial segment.
        inds.push(b);
        inds.push(a);
        inds.push(d);

        // The second triangle of the radial segment.
        inds.push(c);
        inds.push(b);
        inds.push(d);
    }

    println!("{:.1?}", verts);
    println!("{:?}", inds);
}

const THETA_START: f32 = 0f32;
const THETA_END: f32 = 2f32 * std::f32::consts::PI;

const HEIGHT_SEGMENTS: u32 = 1;

#[derive(Debug)]
struct Vertex {
    #[allow(unused)]
    position: [f32; 3],

    #[allow(unused)]
    normal: [f32; 3],
}

fn main() {
    print_cylinder_vertices(1f32, 1f32, 1f32, 3);
}
