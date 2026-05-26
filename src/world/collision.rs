use super::generation::World;
use super::blocks::BlockData;

struct VoxelRaycastHit {
    //coordinates of the voxel that was hit.
    x: i64, y: i64, z: i64,

    //normal of the face that was hit. could technically be an int since
    //each value will only ever be 0, -1, or 1.
    normal_x: f64, normal_y: f64, normal_z: f64,
    
    //where the hit was on the face.
    u: f64, v: f64
}

fn voxel_rayast(
    world: &World,
    x: f64, y: f64, z: f64,
    dx: f64, dy: f64, dz: f64,
    max_distance: Option<f64>
) -> Option<VoxelRaycastHit> {
    const ROOT2: f64 = 1.4143;
    const MAX_STEPS: i64 = 10_000;
    
    let max_steps = if let Some(md) = max_distance {
        (ROOT2 * md) as i64 + 1
    } else {
        MAX_STEPS
    };

    let direction = [dx, dy, dz];
    let inverse_direction= [ 1. / dx, 1. / dy, 1. / dz ];

    let mut pos = [x, y, z];
    let mut voxel = [x.floor(), y.floor(), z.floor()];

    for _ in 0..max_steps {
        let mut lowest_scale_factor = f64::INFINITY;
        let mut closest_axis = 0usize;


        for axis in 0..3 {
            // block edge that we're "moving toward" on this axis
            let target_coordinate =
                if direction[axis] > 0. { voxel[axis] + 1. }
                else { voxel[axis] };

            let target_dist = target_coordinate - pos[axis];

            // how much do we have to scale the vector by to reach this edge
            let scale_factor = target_dist * inverse_direction[axis];

            if scale_factor > 0. && scale_factor < lowest_scale_factor {
                lowest_scale_factor = scale_factor;
                closest_axis = axis;
            }
        }

        let step = [
            dx * lowest_scale_factor,
            dy * lowest_scale_factor,
            dz * lowest_scale_factor
        ];

        //step position
        for axis in 0..3 { pos[axis] += step[axis]; }
        
        //step voxel
        voxel[closest_axis] +=
            if direction[closest_axis] > 0. { 1. }
            else { -1. };

        let (vx, vy, vz) = (voxel[0] as i64, voxel[1] as i64, voxel[2] as i64);
        let block = world.get_block_data(vx, vy, vz);

    }

    None
}
