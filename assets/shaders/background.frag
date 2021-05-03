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

void main() {
    vec3 col = vec3(0.0);
    float d;
    float m = 0.0;

    for (float y = -1.; y < 2; y++) {
        for (float x = -1; x < 2; x++) {
            vec2 uv = in_uv;
            uv *= 20.;
            uv = fract(uv);
            uv -= 0.5;
            vec2 id = floor(uv) + vec2(x, y);
            d = length(uv);
            m += smoothstep(0.0, 0.5, 0.001 / d);
        }
    }

    
    col += vec3(m);

    o_Target = vec4(col, 1.0);
}
