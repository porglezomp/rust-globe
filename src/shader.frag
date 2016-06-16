#version 140

in vec2 attr;
out vec4 color;

void main() {
  color = vec4(1.0, attr, 1.0);
}
