use anyhow;
use git2;
use tera;
use std::io;
use std::path::Path;

const GLAM_ROOT: &str = "..";

fn generate_file(repo: &git2::Repository, tera: &tera::Tera, template_path: &str, output_path: &str) -> anyhow::Result<()> {
    match repo.status_file(&Path::new(output_path)) {
        Ok(status) => {
            if status.is_wt_modified() {
                panic!("output file is modified! {}", output_path);
            }
        }
        Err(e) => {
            if e.code() != git2::ErrorCode::NotFound {
                panic!("error getting git status: {}", e);
            }
        }
    }

    let context = tera::Context::new();
    tera.render_to(template_path, &context, &mut io::stdout().lock()).unwrap();
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let repo = match git2::Repository::open(GLAM_ROOT) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open git repo: {}", e),
    };

    let tera = match tera::Tera::new("templates/**/*.rs") {
        Ok(t) => t,
        Err(e) => panic!("Parsing error(s): {}", e),
    };

    generate_file(&repo, &tera, "swizzles/traits.rs", "src/swizzles/vec_traits.rs")
}
