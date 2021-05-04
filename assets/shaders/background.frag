#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec4 fragPos;
layout(location = 1) in vec2 in_uv;

layout(set = 2, binding = 0) uniform BackgroundMaterial_color {
    vec4 color;
};

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float star(vec2 p) {
    float d;
    float m = 0.0;

    d = length(p);
    m += smoothstep(0.0, 0.5, 0.005 / d);

    return m;
}

void main() {
    vec3 col = vec3(0.0);

    vec2 uv = in_uv;
    uv *= 20.;
    uv = fract(uv);
    uv -= 0.5;

    vec2 gv = fract(uv) - 0.5;
    vec2 id = floor(uv);

    col += star(gv);

    o_Target = vec4(col, 1.0);
}
