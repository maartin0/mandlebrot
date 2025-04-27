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
    float hue = mod(pow(ratio * 360.0, 1.5), 360.0);
    float saturation = 100.0;
    float value = ratio * 100.0;
    float chroma = value * saturation;
    float x = chroma * (1.0 - abs(mod(hue / (60.0 / 360.0), 2.0) - 1.0));
    float m = value - chroma;
    outColor = vec4(
        tenary(
            hue < 60.0 || hue >= 300.0,
            chroma,
            tenary(
                hue < 120.0 || hue >= 240.0,
                x,
                0.0
            )
        ) + m,
        tenary(
            hue < 60.0 || (hue >= 180.0 && hue < 240.0),
            x,
            tenary(
                hue < 180.0,
                chroma,
                0.0
            )
        ) + m,
        tenary(
            hue < 120.0,
            0.0,
            tenary(
                hue < 180.0 || hue >= 300.0,
                x,
                chroma
            )
        ) + m,
        1.0
    );
}