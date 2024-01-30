#version 300 es

precision highp float;

in vec2 vPos;

uniform int depth;
uniform mat3 transform;

out vec4 outColor;

vec2 square(vec2 im) {
    return vec2(
        pow(im.x, 2.0) - pow(im.y, 2.0),
        2.0 * im.x * im.y
    );
}

float pixel(vec2 pos) {
    vec2 z = vec2(0, 0);
    for (int i = 0; i < depth; i++) {
        z = square(z) + pos;
        if (length(z) > 2.0) return float(i) / float(depth);
    }
    return 1.0;
}

float tenary(bool predicate, float if_true, float if_false) {
    if (predicate) return if_true;
    return if_false;
}

void main() {
    float ratio = pixel((vec3(vPos.x, vPos.y, 1.0) * transform).xy);
    outColor = vec4(
        tenary(ratio < 0.33, ratio / 0.33, 1.0),
        tenary(ratio < 0.66, ratio / 0.66, 1.0),
        ratio,
        1.0
    );
}