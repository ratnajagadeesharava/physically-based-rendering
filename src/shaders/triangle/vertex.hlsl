struct VSOut{
    float4 position : SV_POSITION;
};
VSOut main(float4 position : POSITION){
    VSOut output;
    output.position = position;
    return output;
}