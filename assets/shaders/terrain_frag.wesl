import bevy_pbr::render::forward_io::{VertexOutput, FragmentOutput};

fn elevation(pos: vec3<f32>, freq: f32, width_px: f32) -> f32 {
    // Elevation
    let h = pos.y * freq;
    
    // Distance to nearest elevation line
    let dist_to_h = abs(fract(h + 0.5) - 0.5);

    // Screen space gradient (dist / pix)
    let dist_per_px = max(length(vec2(dpdx(h), dpdy(h))), 0.0001); 

    // dist / (dist / pix) = pixels to nearest elevation line!
    let px_to_h = dist_to_h / dist_per_px;

    let line = 1.0 - smoothstep(
        width_px * 0.5 - 0.5, 
        width_px * 0.5 + 0.5,
        px_to_h
    );

    let px_per_dist = 1.0 / dist_per_px; 
    let fade = smoothstep(2.0, 12.0, px_per_dist);

    return line * fade;

    // DEBUG
    // return 1.0 / gradient;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var out: FragmentOutput;
    let pos: vec4<f32> = in.world_position;
    
    let elevation_line = elevation(pos.xyz, 5, 2.0);

    out.color = vec4(0.0, elevation_line, 0.0, 1.0);
    return out;
}