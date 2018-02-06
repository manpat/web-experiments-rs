func float calc_field(vec2 pos) {
	float term = 1.0 + dot(pos, pos)/0.25;
	return 1.0 / (term * term);
}

float field = calc_field(v_pos);
field += calc_field(v_pos - vec2(sin(u_time/3.0)*u_aspect*0.8, 0.0));

// vec3 color = vec3(1.0 - step(field, 0.5));
vec3 color = vec3(smoothstep(0.49, 0.5, field));
// vec3 color = vec3(field);

gl_FragColor = vec4(color, 1.0);