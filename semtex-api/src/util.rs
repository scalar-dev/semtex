use xdg::BaseDirectories;


pub fn xdg_dirs() -> BaseDirectories {
    xdg::BaseDirectories::with_prefix("semtex").unwrap()
}