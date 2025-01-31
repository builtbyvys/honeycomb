struct Uniforms {
    camera_position: vec3<f32>,
    screen_dimensions: vec2<f32>,
    time: f32,
}

@binding(0) @group(0) var<uniform> uniforms: Uniforms;

struct Output {
    pixels: array<vec4<f32>>,
}
@binding(1) @group(0) var<storage, read_write> output: Output;

const MAX_STEPS: i32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.01;

fn sdf_sphere(p: vec3<f32>, center: vec3<f32>, radius: f32) -> f32 {
    return length(p - center) - radius;
}

fn sdf_box(p: vec3<f32>, center: vec3<f32>, size: vec3<f32>) -> f32 {
    let q: vec3<f32> = abs(p - center) - size;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn get_dist(p: vec3<f32>) -> f32 {
    // a sphere at (0,1,6) with radius 1
    let sphere = sdf_sphere(p, vec3<f32>(0.0, 1.0, 6.0), 1.0);
    
    // a box at (0,-1,6) with size (1,1,1)
    let box = sdf_box(p, vec3<f32>(0.0, -1.0, 6.0), vec3<f32>(1.0));
    
    // return the minimum distance between objects
    return min(sphere, box);
}

fn ray_march(ro: vec3<f32>, rd: vec3<f32>) -> f32 {
    var dist_origin = 0.0;
    
    for(var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        let p = ro + rd * dist_origin;
        let dist_scene = get_dist(p);
        dist_origin = dist_origin + dist_scene;
        
        if (dist_origin > MAX_DIST || dist_scene < SURF_DIST) {
            break;
        }
    }
    
    return dist_origin;
}

fn get_normal(p: vec3<f32>) -> vec3<f32> {
    let e = vec2<f32>(0.01, 0.0);
    let n = vec3<f32>(
        get_dist(p + vec3<f32>(e.x, e.y, e.y)) - get_dist(p - vec3<f32>(e.x, e.y, e.y)),
        get_dist(p + vec3<f32>(e.y, e.x, e.y)) - get_dist(p - vec3<f32>(e.y, e.x, e.y)),
        get_dist(p + vec3<f32>(e.y, e.y, e.x)) - get_dist(p - vec3<f32>(e.y, e.y, e.x))
    );
    return normalize(n);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let resolution = uniforms.screen_dimensions;
    let pixel_pos = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let uv = (pixel_pos - 0.5 * resolution) / resolution.y;
    
    let ro = uniforms.camera_position;
    
    let rd = normalize(vec3<f32>(uv.x, uv.y, 1.0));
    
    let d = ray_march(ro, rd);
    
    let p = ro + rd * d;
    let n = get_normal(p);
    
    let light_pos = vec3<f32>(2.0, 4.0, -3.0);
    let l = normalize(light_pos - p);
    let diff = max(dot(n, l), 0.0);
    
    var color = vec3<f32>(diff);
    
    color = mix(color, vec3<f32>(0.6, 0.7, 0.8), 1.0 - exp(-0.0008 * d * d));
    
    let index = global_id.y * u32(resolution.x) + global_id.x;
    output.pixels[index] = vec4<f32>(color, 1.0);
}
