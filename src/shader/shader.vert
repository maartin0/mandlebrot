#version 300 es

in vec4 position;
out vec2 vPos;

void main() {
    vPos = vec2(position.x, position.y);
    gl_Position = position;
}