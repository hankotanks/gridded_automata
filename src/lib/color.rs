use std::{
    ops::RangeInclusive, 
    borrow::Cow
};

pub fn color_shader(cr: Vec<Coloring>) -> Cow<'static, str> {
    let mut color_function = "".to_string();
    color_function.push_str("fn get_color(state: u32) -> vec3<f32> {");
    for rule in cr { color_function.push_str(&rule.cs); }

    color_function.push_str("return vec3<f32>(0.0, 0.0, 0.0);");
    color_function.push('}');

    color_function.into()
}

#[derive(Clone)]
pub struct Coloring { cs: String }

pub fn lerp(range: RangeInclusive<u32>, start: [f32; 3], end: [f32; 3]) -> Coloring {
    Coloring { cs: format!("
        if state >= {}u && state <= {}u {{ \
            let s = f32(state - {}u) / f32({});
            return mix(
                vec3<f32>({:?}, {:?}, {:?}), 
                vec3<f32>({:?}, {:?}, {:?}),
                vec3<f32>(s, s, s)
            );
        }}",
        range.start(), range.end(), 
        range.start(), range.end() - range.start(),
        start[0], start[1], start[2], end[0], end[1], end[2]
    ) }
}

pub fn alive(color: [f32; 3]) -> Coloring {
    Coloring { cs: format!("
        if state != 0u {{ return vec3<f32>({:?}, {:?}, {:?}); }}",
        color[0], color[1], color[2]
    ) }
}

pub fn map(state: u32, color: [f32; 3]) -> Coloring {
    Coloring { cs: format!(
        "if state == {}u {{ return vec3<f32>({:?}, {:?}, {:?}); }}",
        state, color[0], color[1], color[2]
    ) }
}

pub fn map_range(range: RangeInclusive<u32>, color: [f32; 3]) -> Coloring {
    Coloring { cs: format!("
        if(state >= {}u && state <= {}u) {{
            return vec3<f32>({:?}, {:?}, {:?});
        }}",
        range.start(), range.end(), color[0], color[1], color[2]
    ) }
}