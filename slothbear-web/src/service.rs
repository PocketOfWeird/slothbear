use std::process::Command;
use crate::models::{Render, RenderResponse};

fn prepend_rendershare_dir(path: &str) -> String {
    format!("\\\\art-render\\RenderShare\\Missouri State University\\Animation - RenderShare\\{}", path)
}

pub fn send_to_rp(render: Render) -> RenderResponse {
    // Point to the renderpal remote control command executable
    let mut rp_control = Command::new("C:\\Program Files (x86)\\RenderPal V2\\CmdRC\\rprccmd");

    // Match the Renderers to the renderpal renderer syntax 
    let renderer = match render.renderer {
        ref x if x == "Arnold" => "Arnold Renderer/Default version",
        _ => "Arnold Renderer/Default version"
    };

    // Add the render job arguments
    rp_control.arg(format!("-nj_renderer {}", renderer))
        .arg(format!("-nj_splitmode \"2,{}\"", render.split_chunks));

    // Add the Arnold renderer arguments
    rp_control.arg(format!("-projdir \"{}\"", prepend_rendershare_dir(&render.path_project)))
        .arg(format!("-outdir \"{}\"", prepend_rendershare_dir(&render.path_output)))
        .arg(format!("-outfile \"{}\"", render.output_file_name))
        .arg(format!("-camera \"{}\"", render.camera))
        .arg(format!("-frames \"{}\"", render.frames))
        .arg(format!("-fstep \"{}\"", render.frame_step))
        .arg(format!("-width \"{}\"", render.frame_width))
        .arg(format!("-height \"{}\"", render.frame_height))
        .arg(format!("-height \"{}\"", render.frame_height));
    
    // Add the optional renderpal user for notifications, if provided
    if render.rp_user.is_some() {
        rp_control.arg(format!("-nj_emailusers \"{}\"", render.rp_user.unwrap()));
    }

    // Add the final scene path argument
    rp_control.arg(format!("\"{}\"", prepend_rendershare_dir(&render.path_scene)));
    

    // Execute the command and return the RenderResponse 
    match rp_control.output() {
        Ok(output) => return RenderResponse {
                        job_id: Some("abc123".to_owned()),
                        status: output.status.to_string(),
                    },
        Err(err) => return RenderResponse {
                        job_id: None,
                        status: err.to_string(),
                    } 
    }
}