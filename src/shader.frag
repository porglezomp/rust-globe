#version 140

in vec3 attr;
out vec4 color;

void main() {
  color = vec4(attr + 0.5, 1.0);
}
