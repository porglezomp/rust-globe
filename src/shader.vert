#version 140

in vec3 position;
out vec3 attr;

uniform mat4 matrix;
uniform mat4 view;

void main() {
  attr = position;
  gl_Position = view * matrix * vec4(position, 1.0);
}
