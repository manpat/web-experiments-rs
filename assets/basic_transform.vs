attribute vec3 position;

uniform mat4 u_proj;

void main() {
	vec4 world_pos = vec4(position, 1.0);
	gl_Position = u_proj * world_pos;
}
