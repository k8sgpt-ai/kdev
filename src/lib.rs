use std::fs;

const K8SGPT_DEV_FOLDER_NAME: &str = "k8sgpt-dev";


pub fn setup() {
    // Check if the dev folder exists
    fs::create_dir(K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
       println!("! {:?}", why);
    });
}

pub fn run() {

}

pub fn teardown() {

    fs::remove_dir_all(K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
       println!("! {:?}", why);
    });
}