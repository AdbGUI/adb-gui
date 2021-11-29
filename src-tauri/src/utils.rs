use std::fs::{create_dir_all, set_permissions, File, Permissions};
use std::io::copy;
use zip::ZipArchive;

pub fn unzip_file(zip_file: File, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut zip = ZipArchive::new(zip_file).unwrap();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}
