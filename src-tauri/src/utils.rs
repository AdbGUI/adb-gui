use std::{
    fs::{create_dir_all, remove_file, set_permissions, File, Permissions},
    io::copy,
    path::Path,
};
use zip::ZipArchive;

pub fn get_so_name() -> String {
    if cfg!(target_os = "windows") {
        return String::from("windows");
    } else if cfg!(target_os = "macos") {
        return String::from("darwin");
    }
    String::from("linux")
}

pub async fn unzip_file(filename: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let zip_file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => return Err(Box::new(why)),
    };
    let mut zip = ZipArchive::new(zip_file).unwrap();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let path_file_name = path.to_str()
                    .unwrap()
                    .split("/")
                    .next()
                    .unwrap();
                Path::new(dest).join(path_file_name)
            }
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
    remove_file(filename).unwrap();
    Ok(())
}
