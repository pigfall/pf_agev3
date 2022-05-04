#version 330 core
attribute vec2 input_position;
void main(){
  gl_Position = vec4(input_position,0.0,1.0);
}
