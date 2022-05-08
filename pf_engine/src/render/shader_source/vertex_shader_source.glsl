attribute lowp vec2 attrb_pos;
void main(){
  gl_Position = vec4(attrb_pos,0.0,1.0);
}
