// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod uc_find_user;

use std::{error::Error, sync::Arc};

use file_downloader::UreqDownloader as Downloader;
use git_info::GitInfoAnonymousUReq as GitInfo;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let git_info = GitInfo::anonymous();

    let git_info = Arc::new(git_info);

    let downloader = Downloader::new("images");

    let downloader = Arc::new(downloader);

    let ui = AppWindow::new()?;

    uc_find_user::UCFindUser::register(
        ui.as_weak(),
        Arc::clone(&downloader),
        Arc::clone(&git_info),
    );

    ui.run()?;

    Ok(())
}
