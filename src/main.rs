// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};

use slint::{Image, ModelRc, SharedString, VecModel};

use file_downloader::Downloader;
use git_info::GitInfo;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let git_info = GitInfo::anonymous();

    let git_info = Rc::new(git_info);

    let downloader = Downloader::new("images");

    let downloader = Rc::new(downloader);

    // let users = ModelRc::new();
    let users_model = Rc::new(VecModel::from(vec![]));

    let ui = AppWindow::new()?;

    ui.set_users(ModelRc::from(users_model.clone()));

    let git_info_clone = Rc::clone(&git_info);
    let downloader_clone = Rc::clone(&downloader);

    ui.on_find_user({
        let ui_clone = ui.as_weak();

        move |user_name: SharedString| {
            ui_clone.upgrade().unwrap().set_is_loading(true);

            println!("User name: {}", user_name);

            let user = git_info_clone.user(user_name.as_str());

            if let Ok(user) = user {
                println!("User found: {:?}", user);

                let avatar_url = downloader_clone.download(&user.avatar).unwrap();

                let avatar_url = avatar_url.file;

                println!("Avatar URL: {:?}", avatar_url);

                //ui_clone.upgrade().unwrap()

                let user = UIUser {
                    name: SharedString::from(user.name),
                    avatar_url: Image::load_from_path(&avatar_url).unwrap(),
                };

                users_model.push(user);

                ui_clone.upgrade().unwrap().set_is_loading(false);
            } else {
                println!("User not found");
            }
        }
    });

    ui.run()?;

    Ok(())
}
