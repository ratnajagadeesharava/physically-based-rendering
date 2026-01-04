use windows::core::HRESULT;
#[derive(Debug,Clone)]
pub struct Vertex{
    pub pos:[f32;4],
}

pub fn throw_if_failed(hr: HRESULT) -> windows::core::Result<()> {
    if hr.is_err() {
        println!("Error:{:?}", hr.0);
        Err(hr.into())
    } else {
        Ok(())
    }
}
