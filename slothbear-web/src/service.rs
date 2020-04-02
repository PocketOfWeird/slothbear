use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::process::Command;
use std::fs;

use crate::models::{Render, RenderResponse};


fn prepend_rendershare_dir(path: &str) -> String {
    format!("\\\\art-render\\RenderShare\\_Cloud\\Missouri State University\\Animation - RenderShare\\{}", path)
}

pub fn send_to_rp(render: Render) -> RenderResponse {
    
    let mut rp_control = Command::new("rprccmd");
    
    // Create internal render job id
    let job_id: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    // Match the Renderers to the renderpal renderer syntax 
    let renderer = match render.renderer {
        ref x if x == "Arnold" => "Arnold Renderer/Default version",
        _ => "Arnold Renderer/Default version"
    };

    // Create the renderset file needed by renderpal
    let renderset_path = format!("C:\\ProgramData\\slothbear\\job_{}.rset", job_id);
    let renderset_data = format!(
        "<RenderSet><Renderer>{renderer}</Renderer><Values><camera><Value>{camera}</Value></camera><frames><Value>{frames}</Value></frames><fstep><Value>{fstep}</Value></fstep><height><Value>{height}</Value></height><of><Value>exr</Value></of><outdir><Value>{outdir}</Value></outdir><outfile><Value>{outfile}</Value></outfile><projdir><Value>{projdir}</Value></projdir><rt><Value>0</Value></rt><scene><Value>{scene}</Value></scene><verbosity><Value>1</Value></verbosity><width><Value>{width}</Value></width></Values></RenderSet>",
        renderer = renderer,
        projdir = prepend_rendershare_dir(&render.path_project),
        outdir = prepend_rendershare_dir(&render.path_output),
        outfile = render.output_file_name,
        camera = render.camera,
        frames = render.frames,
        fstep = render.frame_step,
        scene = prepend_rendershare_dir(&render.path_scene),
        width = render.frame_width,
        height = render.frame_height,
    );
    fs::write(&renderset_path, &renderset_data.as_bytes()).expect("Unable to write to the renderset file");

    rp_control.arg("-compact");
    rp_control.arg(format!("-nj_splitmode \"2,{}\"", render.split_chunks));
    rp_control.args(&["-nj_renderer", &renderer]);
    rp_control.args(&["-importset", &renderset_path]);
    
    if render.rp_user.is_some() {
        rp_control.args(&["-nj_emailusers \"{}\"", &render.rp_user.unwrap()]);
    }
    rp_control.arg(prepend_rendershare_dir(&render.path_scene));
    
    println!("rp_control: {:?}", rp_control);

    // Execute the command and return the RenderResponse 
    match rp_control.output() {
        Ok(output) => return RenderResponse {
                        job_id: Some("abc123".to_owned()),
                        status: output.status.to_string(),
                        stdout: Some(String::from_utf8(output.stdout).unwrap_or_default()),
                        stderr: Some(String::from_utf8(output.stderr).unwrap_or_default()),
                    },
        Err(err) => return RenderResponse {
                        job_id: None,
                        status: err.to_string(),
                        stdout: None,
                        stderr: None,
                    } 
    }
}