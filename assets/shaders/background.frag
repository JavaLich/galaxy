#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec4 fragPos;
layout(location = 1) in vec2 in_uv;

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float star(vec2 p) {
    float d;
    float m = 0.0;

    d = length(p);
    m += smoothstep(0.01, 0.5, 0.005 / d);

    return m;
}

void main() {
    vec3 col = vec3(0.0);

    vec3 fragSphere = normalize(fragPos.xyz);
    vec3 d = -fragSphere;
    vec2 uv = vec2(0.5 - atan(d.x, d.y)/(2. * 3.1415), 0.5 - asin(d.y)/3.1415);

    //vec2 uv = in_uv;
    uv *= 20.;

    vec2 gv = fract(uv) - 0.5;
    vec2 id = floor(uv);

    for (float y = -1.; y < 2.; y++) {
        for (float x = -1.; x < 2.; x++) {
            vec2 offset = vec2(x, y);

            float n = rand(id + offset);
            col += star(gv - offset -vec2(n, fract(n*34.)-.5));
        }
    }

    o_Target = vec4(uv, 0.0, 1.0);
    o_Target = vec4(col, 1.0);
}
