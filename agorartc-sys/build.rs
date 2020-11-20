use std::env;
use error_chain::error_chain;
use std::fs::File;
use std::fs;
use std::io;
use std::path::Path;
use std::io::Write;
use tokio::runtime::Runtime;
use std::ops::Add;
use find_folder::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Header(reqwest::header::ToStrError);
    }
}

async fn download_zip(path_str: String) -> Result<()> {
    let target = "https://download.agora.io/demo/release/rust-vender.zip";
    let response = reqwest::get(target).await?;

    let path_st = path_str.add("/vender.zip");
    let path = Path::new(&path_st);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    println!("Starting download ... \nIt may take some time. Please wait.");
    let content = response.bytes().await?;
    file.write_all(&content)?;
    println!("Congregations! Download succeeded.");
    Ok(())
}

fn unzip_win() {
    let fname = std::path::Path::new("vender.zip");
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        #[allow(deprecated)]
            let outpath = file.sanitized_name();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if find_folder::check_dir("vender", std::path::Path::new(manifest_dir.clone().as_str())).is_err() {
        let _res = Runtime::new().expect("Download failed.").block_on(download_zip(manifest_dir.clone()));

        if cfg!(target_os = "windows") {
            unzip_win();
            let output = std::process::Command::new("cmd").args(&["/C", "rm -r vender.zip __MACOSX"]).output().expect("failed");
            let info = output.stdout;
            println!("{}", std::result::Result::ok(std::str::from_utf8(&info)).unwrap());
        } else {
            let output = std::process::Command::new("sh").arg("-c").arg("unzip vender.zip").output().expect("failed");
            let info = output.stdout;
            println!("{}", std::result::Result::ok(std::str::from_utf8(&info)).unwrap());
            let output = std::process::Command::new("sh").arg("-c").arg("rm -r vender.zip && rm -rf __MACOSX").output().expect("failed");
            let info = output.stdout;
            println!("{}", std::result::Result::ok(std::str::from_utf8(&info)).unwrap());
        }
    }


    if cfg!(windows) {
        println!("cargo:rustc-flags=-L {}/vender -l static=agorartc -L ={}/vender -l dylib=agora_rtc_sdk", manifest_dir, manifest_dir);
    } else if cfg!(unix) {
        println!("cargo:rustc-flags=-L {}/vender -l static=agorartc -L framework={}/vender -l framework=AgoraRtcKit -l dylib=c++", manifest_dir, manifest_dir);
    }
}