#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

float plot(vec2 st, float pct) {
    return smoothstep(pct - 0.01, pct, st.y) - smoothstep(pct, pct + 0.01, st.y);
}

void main() {
    vec2 xy = gl_FragCoord.xy / u_resolution;
    float y = pow(xy.x, 2.);
    vec3 color = vec3(y);

    // Plot a line
    float pct  = plot(xy,y);
    color = (1. - pct) * color + pct * vec3(0., 1., 0.);

    gl_FragColor = vec4(color, 1.);
}