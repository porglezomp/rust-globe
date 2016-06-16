#version 140

in vec2 position;

uniform float t;

void main() {
    gl_Position = vec4(position * sin(t) * 0.5, 0.0, 1.0);
}