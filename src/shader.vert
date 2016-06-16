#version 140

in vec2 position;
out vec2 attr;

uniform mat4 matrix;

void main() {
  attr = position;
  gl_Position = matrix * vec4(position, 0.0, 1.0);
}
