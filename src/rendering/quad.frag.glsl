#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

void main() {
  float alpha = texture(sampler2D(u_texture, u_sampler), v_uv).a;
  target0 = vec4(63 / 255., 81 / 255., 181 / 255., alpha);
}
