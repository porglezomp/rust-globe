#version 140

out vec4 color;
in float progress;

uniform float time;

void main() {
  float factor = smoothstep(0.0, 1.0, clamp((time - progress) * 0.16, 0.0, 1.0));
  vec4 potential_color = mix(vec4(0.9, 1.0, 0.9, 1.0), vec4(0.2, 0.8, 0.2, 0.7), factor);
  color = (time > progress) ? potential_color : vec4(0.0);
}
