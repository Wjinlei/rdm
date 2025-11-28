use clap::Parser;
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

extern crate dirs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    /// tag name
    #[arg(short, long)]
    tag: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.tag {
        Some(tag_name) => create_dotfile(args.file, tag_name),
        None => create_dotfile(args.file, String::new()),
    }
}

// 创建.dotfiles目录
fn create_dotfile(path: String, tag_name: String) {
    let dotfiles_dir = build_dotfiles_path(&path, &tag_name);
    move_files_to_dotfiles(&path, &dotfiles_dir);
}

// 生成最终在.dotfiles目录中的保存路径
fn build_dotfiles_path(path: &String, tag_name: &str) -> PathBuf {
    let home_dir = dirs::home_dir().expect("build_dotfiles_path: 没有找到Home目录!");
    let mut target_dir = home_dir.clone();
    target_dir.push(".dotfiles");
    if !tag_name.is_empty() {
        target_dir.push(format!("tag-{}", tag_name));
    }
    let parent_path = get_relative_path_of_the_parent_directory(&home_dir, path);
    remove_config_path_with_dot(&parent_path, &mut target_dir);
    fs::create_dir_all(&target_dir).expect("build_dotfiles_path: 创建目录失败!");
    target_dir
}

// 基于基准目录，获取父目录的相对路径
fn get_relative_path_of_the_parent_directory(base_dir: &Path, src_path: &String) -> PathBuf {
    let absolute_path = fs::canonicalize(src_path)
        .expect("get_relative_path_of_the_parent_directory: 获取绝对路径失败!");
    let parent_path = absolute_path
        .parent()
        .expect("get_relative_path_of_the_parent_directory: 获取父目录失败!");
    let relative_path = parent_path
        .strip_prefix(base_dir)
        .expect("get_relative_path_of_the_parent_directory: 父目录不在基准目录内!");
    relative_path.to_path_buf()
}

// 移动文件到.dotfiles目录
fn move_files_to_dotfiles(source_path: &str, target_dir: &Path) {
    let source_path = Path::new(source_path);
    let file_name = source_path
        .file_name()
        .expect("move_files_to_dotfiles: 获取文件名失败!");
    let target_path = target_dir.join(file_name);
    fs::rename(source_path, &target_path).expect("move_files_to_dotfiles: 重命名失败!");
    symlink(&target_path, source_path).expect("move_files_to_dotfiles: 创建软链接失败!");
}

// 路径中第一级目录如果是隐藏文件则改成非隐藏
fn remove_config_path_with_dot(src_path: &Path, dest_path: &mut PathBuf) {
    let mut components = src_path.components();
    let component = components
        .next()
        .expect("remove_config_path_with_dot: 获取第一级路径失败");
    let component_str = component.as_os_str().to_string_lossy();
    if !component_str.starts_with('.') {
        dest_path.push(component);
    } else {
        dest_path.push(&component_str[1..]);
    }
}
