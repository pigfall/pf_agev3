attribute lowp vec2 attrb_pos;
varying lowp vec2 texCoord;
void main(){
  gl_Position = vec4(attrb_pos,0.0,1.0);
  texCoord = vec2(1.0,1.0);
}
