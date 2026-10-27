struct Uniforms { deep: vec4<f32>, middle: vec4<f32>, bright: vec4<f32>, size: vec2<f32>, time: f32, theme: u32 };
@group(0) @binding(0) var<uniform> u: Uniforms;

struct VsOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };

@vertex
fn vs(@builtin(vertex_index) id: u32) -> VsOut {
    // one oversized triangle: corners (0,0) (2,0) (0,2)
    let corner = vec2<f32>(f32((id << 1u) & 2u), f32(id & 2u));
    var o: VsOut;
    o.pos = vec4<f32>(corner * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(corner.x, 1.0 - corner.y);   // uv origin top-left, y down (uv.y = 1 at the bottom of the screen)
    return o;
}

fn sstep(e0: f32, e1: f32, x: f32) -> f32 {        // Metal-style smoothstep, valid for e0 > e1
    let t = clamp((x - e0) / (e1 - e0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

fn pcg(v: u32) -> u32 {
    let state = v * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

// Random 0..1 for an integer cell. Integer arithmetic, so neighbouring pixels that reach the same
// cell from different sides always agree (a float `fract(p * big)` hash does not once the compiler
// fuses the multiply, and every disagreement shows up as a hard seam).
fn hash(cell: vec2<f32>, seed: u32) -> f32 {
    let c = bitcast<vec2<u32>>(vec2<i32>(floor(cell)));
    return f32(pcg(c.x ^ pcg(c.y ^ seed)) >> 8u) / 16777216.0;
}

// Gradient noise in 0..1 with a quintic fade: no flat spots or creases at the lattice.
fn lattice_gradient(cell: vec2<f32>) -> vec2<f32> {
    let angle = hash(cell, 0u) * 6.2831853;
    return vec2<f32>(cos(angle), sin(angle));
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let w = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let a = dot(lattice_gradient(i), f);
    let b = dot(lattice_gradient(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0));
    let c = dot(lattice_gradient(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0));
    let d = dot(lattice_gradient(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0));
    return clamp(0.5 + 0.75 * mix(mix(a, b, w.x), mix(c, d, w.x), w.y), 0.0, 1.0);
}

fn fbm(p_in: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var p = p_in;
    for (var octave = 0; octave < 5; octave++) {
        value += amplitude * noise(p);
        p = p * 2.03 + vec2<f32>(11.7);
        amplitude *= 0.5;
    }
    return value;
}

// Colour blends in Oklab, so a gradient between two hues does not dip into grey or brown on the way.
fn to_linear(c: vec3<f32>) -> vec3<f32> {
    let k = max(c, vec3<f32>(0.0));
    return select(pow((k + vec3<f32>(0.055)) / 1.055, vec3<f32>(2.4)), k / 12.92, k <= vec3<f32>(0.04045));
}

fn to_display(c: vec3<f32>) -> vec3<f32> {
    let k = max(c, vec3<f32>(0.0));
    return select(1.055 * pow(k, vec3<f32>(1.0 / 2.4)) - vec3<f32>(0.055), k * 12.92, k <= vec3<f32>(0.0031308));
}

fn oklab(display: vec3<f32>) -> vec3<f32> {
    let c = to_linear(display);
    let l = pow(0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b + 1e-7, 1.0 / 3.0);
    let m = pow(0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b + 1e-7, 1.0 / 3.0);
    let s = pow(0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b + 1e-7, 1.0 / 3.0);
    return vec3<f32>(0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
                     1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
                     0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s);
}

fn from_oklab(lab: vec3<f32>) -> vec3<f32> {
    let l = lab.x + 0.3963377774 * lab.y + 0.2158037573 * lab.z;
    let m = lab.x - 0.1055613458 * lab.y - 0.0638541728 * lab.z;
    let s = lab.x - 0.0894841775 * lab.y - 1.2914855480 * lab.z;
    let l3 = l * l * l;
    let m3 = m * m * m;
    let s3 = s * s * s;
    return to_display(vec3<f32>(4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
                                -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
                                -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3));
}

fn blend(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return from_oklab(mix(oklab(a), oklab(b), t));
}

// Drifting soft discs of light; `rate` moves them upwards (negative: downwards), `density` is the
// share of empty cells (higher = fewer lights).
fn bokeh(p: vec2<f32>, time: f32, cells: f32, rate: f32, seed: u32, density: f32) -> f32 {
    var grid = p * cells;
    grid.y -= time * rate;
    let cell = floor(grid);
    var light = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let here = cell + vec2<f32>(f32(x), f32(y));
            let random = vec2<f32>(hash(here, seed), hash(here, seed + 1u));
            let center = here + vec2<f32>(0.5) + 0.36 * sin(random * 6.2831 + time * (vec2<f32>(0.08) + 0.1 * random.yx));
            let radius = mix(0.08, 0.26, random.x * random.x);
            let dist = length(grid - center);
            let twinkle = 0.8 + 0.2 * sin(time * (0.2 + 0.4 * random.y) + random.x * 40.0);
            let present = step(density, hash(here, seed + 2u));
            light += present * twinkle * sstep(radius, radius * 0.3, dist) * (0.4 + 0.6 * random.y);
        }
    }
    return light;
}

fn cosmic(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    var color = blend(u.middle.rgb * 1.12, u.deep.rgb, sstep(0.0, 1.0, uv.y * 0.9 + 0.05));
    let pool = vec2<f32>((uv.x - 0.82) * aspect, (uv.y - 0.04) * 1.3);
    color += u.bright.rgb * 0.22 * exp(-2.2 * dot(pool, pool));
    // one soft ribbon of light across the lower half: four strands riding the same slow wave,
    // drifting apart and together along the way
    let flow = 0.08 * sin(uv.x * 2.6 + time * 0.07 + 0.8) + 0.03 * sin(uv.x * 5.3 - time * 0.05 + 1.3);
    let fade = sstep(-0.1, 0.4, uv.x) * sstep(1.15, 0.6, uv.x);
    for (var strand = 0; strand < 4; strand++) {
        let s = f32(strand);
        let spread = 0.6 + 0.4 * sin(uv.x * 2.4 + time * 0.05 + s * 1.7);
        let center = 0.6 + flow + (0.016 + 0.014 * s) * spread * (s - 1.5);
        let d = uv.y - center;
        let core = 0.0012 / (abs(d) + 0.003);
        let haze = exp(-d * d * 260.0);
        let tint = mix(u.bright.rgb, vec3<f32>(0.85, 0.92, 1.0), 0.3 + 0.1 * s);
        color += tint * (core * 0.2 + haze * 0.07) * fade;
    }
    let p = vec2<f32>(uv.x * aspect, uv.y);
    color += u.bright.rgb * 0.16 * bokeh(p, time, 4.0, 0.01, 10u, 0.72);
    color += vec3<f32>(0.9, 0.95, 1.0) * 0.08 * bokeh(p, time * 1.2, 9.0, 0.016, 70u, 0.86);
    return color;
}

fn aurora(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    var color = blend(u.deep.rgb * 1.8, u.deep.rgb * 0.7, uv.y);
    let p = vec2<f32>(uv.x * aspect, uv.y);
    for (var layer = 0; layer < 3; layer++) {
        let l = f32(layer);
        let sway = fbm(vec2<f32>(p.x * 0.6 + l * 3.7 + time * 0.015, time * 0.02 + l));
        let center = 0.30 + 0.09 * l + 0.32 * (sway - 0.5) + 0.04 * sin(p.x * 1.5 + time * 0.06 + l * 2.0);
        let thickness = 6.0 + 2.0 * l + 2.0 * sin(p.x * 1.1 + time * 0.04 + l);
        let below = uv.y - center;
        // a crisp lower edge and a long glow upwards, like a real curtain
        let reach = select(thickness * 0.5, thickness * 1.7, below > 0.0);
        let curtain = exp(-below * below * reach * reach);
        // wide, soft folds instead of hard columns
        let fold = 0.65 * noise(vec2<f32>(p.x * 3.0 + l * 7.0, time * 0.05 + l))
                 + 0.35 * noise(vec2<f32>(p.x * 7.3 + l * 3.0, time * 0.08 - l));
        let rays = 0.45 + 0.55 * sstep(0.3, 0.75, fold);
        let tint = mix(u.bright.rgb,
                       mix(vec3<f32>(0.35, 0.55, 1.0), vec3<f32>(0.7, 0.45, 1.0), l * 0.5),
                       sstep(-0.05, 0.14, -below));
        let rim = exp(-below * below * thickness * thickness * 9.0);
        color += tint * curtain * rays * (0.55 - 0.12 * l) + u.bright.rgb * rim * rays * (0.22 - 0.05 * l);
    }
    let starCell = floor(p * 150.0);
    let star = step(0.9975, hash(starCell, 11u)) * (0.75 + 0.25 * sin(time * 0.8 + hash(starCell, 12u) * 30.0));
    let local = fract(p * 150.0) - vec2<f32>(0.5);
    color += star * sstep(0.35, 0.0, length(local)) * sstep(0.75, 0.1, uv.y) * 0.8;
    color += u.middle.rgb * 0.4 * sstep(0.62, 1.0, uv.y) * (0.7 + 0.3 * fbm(vec2<f32>(p.x * 1.2, time * 0.02)));
    return color;
}

fn sunset(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    let top = u.deep.rgb * 1.3;
    let low = mix(u.bright.rgb, vec3<f32>(1.0, 0.5, 0.42), 0.35) * 0.92;
    var color = blend(top, u.middle.rgb, sstep(0.0, 0.62, uv.y));
    color = blend(color, low, sstep(0.48, 1.05, uv.y));
    let p = vec2<f32>(uv.x * aspect, uv.y);
    for (var glow = 0; glow < 5; glow++) {
        let b = f32(glow);
        let center = vec2<f32>((0.18 + 0.19 * b) * aspect + 0.12 * sin(time * (0.035 + 0.01 * b) + b * 2.0),
                               0.5 + 0.2 * sin(time * (0.03 + 0.008 * b) + b * 1.3));
        let radius = 0.24 + 0.06 * sin(b * 4.1);
        let d = p - center;
        color += mix(u.bright.rgb, vec3<f32>(1.0, 0.4, 0.62), fract(b * 0.37)) * exp(-dot(d, d) / (radius * radius)) * 0.16;
    }
    let band = uv.y - 0.78;
    color += mix(u.bright.rgb, vec3<f32>(1.0, 0.82, 0.62), 0.4) * 0.2 * exp(-band * band * 40.0)
             * (0.9 + 0.1 * sin(uv.x * 2.0 + time * 0.1));
    color += vec3<f32>(1.0, 0.88, 0.75) * 0.07 * bokeh(p, time, 6.0, -0.008, 30u, 0.8);
    return color;
}


// ---------------------------------------------------------------------------------------------
// Ember, Tide and Daylight (docs/DESIGN.md 3.16)
//
// These three repeat exactly every LOOP seconds: every moving part is either a sine of a whole
// number of turns per loop, a particle with a life of one loop, or a noise field carried round a
// circle. A presentation can therefore loop one period without a seam, and a reduced-motion
// still is a frame like any other.
// ---------------------------------------------------------------------------------------------

const TAU: f32 = 6.2831853;
const LOOP: f32 = 10.0;

fn loop_phase(time: f32) -> f32 {
    return fract(time / LOOP) * TAU;
}

// A point that goes once round a circle per loop: added to noise coordinates, the field drifts and
// morphs and is back where it started when the loop ends.
fn orbit(ph: f32, radius: f32, offset: f32) -> vec2<f32> {
    return radius * vec2<f32>(cos(ph + offset), sin(ph + offset));
}

// Soft lights that live one loop each: born at one edge, drift across, fade out at the other, so
// the wrap is never seen. `rise` is +1 upwards, -1 downwards; `spread` is the share of the width
// they are born over; `sway` is the sideways swing in uv units.
fn drift_layer(p: vec2<f32>, aspect: f32, ph: f32, count: i32, seed: u32, size: f32, travel: f32,
               sway: f32, rise: f32, spread: f32, young: vec3<f32>, old: vec3<f32>) -> vec3<f32> {
    var light = vec3<f32>(0.0);
    for (var i = 0; i < count; i++) {
        let id = vec2<f32>(f32(i), 3.0);
        let r0 = hash(id, seed);
        let r1 = hash(id, seed + 1u);
        let r2 = hash(id, seed + 2u);
        let r3 = hash(id, seed + 3u);
        let life = fract(ph / TAU + r1);
        let x = (0.5 + (r0 - 0.5) * spread) * aspect + (r0 - 0.5) * 0.16 * life
              + sway * sin(ph * (1.0 + floor(r2 * 2.0)) + r3 * TAU);
        let y = select(-0.04 + life * travel + r3 * 0.08, 1.04 - life * travel - r3 * 0.08, rise > 0.0);
        let appear = sstep(0.0, 0.14, life) * sstep(1.0, 0.5, life);
        let flicker = 0.74 + 0.26 * sin(ph * (2.0 + floor(r1 * 3.0)) + r2 * TAU);
        let radius = size * (0.55 + 0.9 * r2);
        let d = p - vec2<f32>(x, y);
        let d2 = dot(d, d);
        let glow = exp(-d2 / (radius * radius)) + 0.16 * exp(-d2 / (radius * radius * 20.0));
        light += mix(young, old, sstep(0.0, 1.0, life)) * glow * appear * flicker * (0.55 + 0.45 * r3);
    }
    return light;
}

fn ember(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    let ph = loop_phase(time);
    let p = vec2<f32>(uv.x * aspect, uv.y);
    // charcoal above, copper where the hearth is
    var color = blend(u.deep.rgb * 1.2, u.middle.rgb * 0.85, sstep(0.0, 1.0, uv.y));
    // the hearth: one low glow that breathes once per loop
    let breath = 1.0 + 0.10 * sin(ph) + 0.05 * sin(2.0 * ph + 1.3);
    let hearth = vec2<f32>((uv.x - 0.55) * aspect * 0.55, (uv.y - 1.08) * 1.5);
    color += u.bright.rgb * 0.55 * breath * exp(-dot(hearth, hearth) * 4.2);
    // heat haze: warm smoke the fire stirs, carried slowly round
    let sp = vec2<f32>(p.x * 1.1, p.y * 1.5);
    let smoke = 0.65 * noise(sp + orbit(ph, 0.35, 0.0)) + 0.35 * noise(sp * 2.3 + vec2<f32>(5.1, 1.7) + orbit(2.0 * ph, 0.12, 2.0));
    color += u.middle.rgb * 0.9 * sstep(0.42, 0.85, smoke) * sstep(0.10, 0.95, uv.y);
    // embers in three depths: many small slow ones far away, a few large soft ones near
    let amber = vec3<f32>(1.0, 0.80, 0.48);
    let deep_red = u.bright.rgb * vec3<f32>(0.95, 0.42, 0.22);
    color += drift_layer(p, aspect, ph, 30, 100u, 0.0050, 0.60, 0.010, 1.0, 1.0, amber, deep_red) * 0.80;
    color += drift_layer(p, aspect, ph, 16, 200u, 0.0085, 0.85, 0.022, 1.0, 0.95, amber, deep_red) * 1.25;
    color += drift_layer(p, aspect, ph, 6, 300u, 0.0190, 1.00, 0.040, 1.0, 0.80, amber, deep_red) * 0.75;
    return color;
}

// One step of the caustic net: the point is pushed round by waves of the given phase and the net's
// brightness is read where the waves focus (after "Tileable Water Caustic", Dave Hoskins, public domain).
fn caustic_step(i: vec2<f32>, p: vec2<f32>, t: f32) -> vec3<f32> {
    let inten = 0.0065;
    let j = p + vec2<f32>(cos(t - i.x) + sin(t + i.y), sin(t - i.y) + cos(t + i.x));
    let c = 1.0 / length(vec2<f32>(p.x / (sin(j.x + t) / inten), p.y / (cos(j.y + t) / inten)));
    return vec3<f32>(j, c);
}

fn caustic(p: vec2<f32>, ph: f32) -> f32 {
    var i = p;
    var c = 1.0;
    var s = caustic_step(i, p, -2.0 * ph);
    i = s.xy; c += s.z;
    s = caustic_step(i, p, -1.0 * ph);
    i = s.xy; c += s.z;
    s = caustic_step(i, p, 1.0 * ph + 0.7);
    i = s.xy; c += s.z;
    s = caustic_step(i, p, 0.0 * ph + 2.1);
    i = s.xy; c += s.z;
    s = caustic_step(i, p, 1.0 * ph + 4.0);
    c += s.z;
    c = 1.17 - pow(c / 5.0, 1.4);
    return pow(abs(c), 8.0);
}

fn tide(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    let ph = loop_phase(time);
    let p = vec2<f32>(uv.x * aspect, uv.y);
    // light water at the surface, the dark below
    var color = blend(u.middle.rgb * 1.25 + u.bright.rgb * 0.05, u.deep.rgb, sstep(0.0, 1.0, uv.y));
    // slow shafts from the surface, slanted, swaying a little
    let slant = p.x * 0.9 - p.y * 0.4;
    let shaft = 0.6 * noise(vec2<f32>(slant * 2.6, 0.5) + orbit(ph, 0.45, 0.0)) + 0.4 * noise(vec2<f32>(slant * 6.1, 2.5) + orbit(ph, 0.3, 3.0));
    color += u.bright.rgb * 0.07 * sstep(0.50, 0.85, shaft) * exp(-uv.y * 1.8);
    // caustics: the net of light that water throws, strong near the surface and gone in the deep
    let net = pow(clamp(caustic(p * 4.0 - vec2<f32>(250.0), ph), 0.0, 1.6), 1.25);
    color += u.bright.rgb * net * (0.07 + 0.80 * exp(-uv.y * 2.3));
    // plankton: tiny cold lights that sink very slowly
    let pale = u.bright.rgb * vec3<f32>(0.75, 1.0, 1.0);
    color += drift_layer(p, aspect, ph, 12, 400u, 0.0035, 0.45, 0.012, -1.0, 1.1, pale, pale * 0.6) * 0.40;
    color += drift_layer(p, aspect, ph, 5, 500u, 0.0120, 0.30, 0.020, -1.0, 1.1, pale, pale * 0.6) * 0.16;
    return color;
}

fn daylight(uv: vec2<f32>, aspect: f32, time: f32) -> vec3<f32> {
    let ph = loop_phase(time);
    let p = vec2<f32>(uv.x * aspect, uv.y);
    // a cool sky high up, warming toward the bottom
    var color = blend(u.deep.rgb, u.middle.rgb, sstep(0.0, 0.75, uv.y));
    color = blend(color, u.bright.rgb, 0.85 * sstep(0.5, 1.1, uv.y));
    // the sun's bloom, upper right, breathing once per loop
    let sun = vec2<f32>((uv.x - 0.80) * aspect, (uv.y + 0.06) * 1.25);
    let breath = 1.0 + 0.06 * sin(ph);
    color = mix(color, u.bright.rgb, clamp(0.85 * breath * exp(-2.6 * dot(sun, sun)), 0.0, 1.0));
    // luminous cloud and its soft shade, carried round slowly
    let cp = vec2<f32>(p.x * 1.05, p.y * 1.7);
    let cloud = 0.62 * noise(cp + orbit(ph, 0.30, 0.0)) + 0.38 * noise(cp * 2.3 + vec2<f32>(7.3, 2.1) + orbit(ph, 0.16, 2.0));
    color = mix(color, vec3<f32>(1.0, 0.985, 0.955), 0.62 * sstep(0.50, 0.80, cloud) * sstep(1.05, 0.0, uv.y));
    color = mix(color, u.deep.rgb * 0.92, 0.45 * sstep(0.45, 0.20, cloud));
    // a window of light across the room: two slanted bands that drift to and fro
    let slant = p.x * 0.78 + p.y * 0.62;
    let band = exp(-pow((slant - 1.15 - 0.10 * sin(ph)) * 3.2, 2.0)) + 0.6 * exp(-pow((slant - 1.65 - 0.08 * sin(ph + 2.0)) * 4.4, 2.0));
    color = mix(color, u.bright.rgb, 0.55 * band);
    // out-of-focus lights drifting up
    let glint = vec3<f32>(1.0, 0.97, 0.90);
    color = mix(color, glint, clamp(length(drift_layer(p, aspect, ph, 7, 600u, 0.034, 0.32, 0.030, 1.0, 1.0, glint, glint)) * 0.30, 0.0, 0.5));
    return color;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let aspect = u.size.x / max(u.size.y, 1.0);
    let uv = in.uv;
    let time = u.time;
    var color: vec3<f32>;
    if (u.theme == 1u) { color = aurora(uv, aspect, time); }
    else if (u.theme == 2u) { color = sunset(uv, aspect, time); }
    else if (u.theme == 4u) { color = ember(uv, aspect, time); }
    else if (u.theme == 5u) { color = tide(uv, aspect, time); }
    else if (u.theme == 6u) { color = daylight(uv, aspect, time); }
    else { color = cosmic(uv, aspect, time); }
    let centered = (uv - vec2<f32>(0.5)) * vec2<f32>(aspect, 1.0);
    let light_look = u.theme == 6u;
    // vignette: the dark looks fall off at the edges, the light look only just
    color = color * (1.0 - select(0.4, 0.07, light_look) * sstep(0.35, 1.25, length(centered)));
    if (!light_look) {
        color = color / (vec3<f32>(1.0) + color * 0.35);                  // soft tone map
    }
    if (u.theme >= 4u) {
        // the newer looks are smooth gradients: a pixel of noise keeps the 8-bit steps from banding
        color += (hash(in.pos.xy, 77u) - 0.5) / 255.0;
    }
    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
