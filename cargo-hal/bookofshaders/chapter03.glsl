#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
    vec2 xy = (gl_FragCoord.xy - u_mouse) / (u_resolution );
    // gl_FragColor = vec4(abs(sin(u_time)), .0, .0, 1.);
    gl_FragColor = vec4(xy.y, xy.x, .0, 1.);
}