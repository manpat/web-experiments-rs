precision mediump float;

varying vec2 v_pos;

uniform sampler2D u_tex;
uniform float u_aspect;
uniform float u_time;

void main() {
	float ang = atan(v_pos.y, v_pos.x * u_aspect) / 3.14159 * 0.5 + 0.5;
	float dist = length(v_pos * vec2(u_aspect, 1.0));

	float sections = 5.0;

	float seg_ang = mod(ang * sections * 2.0, 2.0);
	if(seg_ang >= 1.0)
		seg_ang = 2.0 - seg_ang;

	seg_ang = mod(seg_ang, 1.0);

	if(dist >= 1.0)
		dist = 2.0 - dist;

	float resamp_ang = seg_ang / sections * 3.14159 + u_time / 3.0;
	vec2 dir = vec2(cos(resamp_ang)/u_aspect, sin(resamp_ang));

	vec2 uv = dir * dist * 0.5 + 0.5;
	// vec2 uv = v_pos * 0.5 + 0.5;

	vec4 col = texture2D(u_tex, uv);

	// gl_FragColor = vec4(uv, 0.0, 1.0);
	gl_FragColor = vec4(col.rgb, 1.0);
}
