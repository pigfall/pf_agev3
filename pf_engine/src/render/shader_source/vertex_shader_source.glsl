attribute lowp vec2 attrb_pos;
attribute lowp vec3 vertexNormal;
attribute lowp vec2 attrib_text_coord;
varying lowp vec2 texCoord;
void main(){
  gl_Position = vec4(attrb_pos,0.0,1.0);
  texCoord = attrib_text_coord;
}
