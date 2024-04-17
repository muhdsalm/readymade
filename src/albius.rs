use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// todo: rewrite this shit and kill albius

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub setup: Vec<DiskOperation>,
    pub mountpoints: Vec<Mountpoint>,
    pub installation: Installation,
    pub post_installation: Vec<PostInstallation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskOperationType {
    Label,
    Mkpart,
    Rm,
    Resizepart,
    Namepart,
    Setflag,
    Format,
    LuksFormat,
    Pvcreate,
    Pvresize,
    Pvremove,
    Vgcreate,
    Vgrename,
    Vgextend,
    Vgreduce,
    Vgremove,
    Lvcreate,
    Lvrename,
    Lvremove,
    MakeThinPool,
    LvcreateThin,
    LvmFormat,
    LvmLuksFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskOperation {
    pub disk: PathBuf,
    pub operation: DiskOperationType,
    pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mountpoint {
    pub partition: PathBuf,
    pub mountpoint: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Method {
    Unsquashfs,
    Oci,
    Source,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installation {
    pub method: Method,
    pub source: PathBuf,
    pub initramfs_pre: Vec<String>,
    pub initramfs_post: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostInstallationOperation {
    Adduser,
    Timezone,
    Shell,
    Pkgremove,
    Hostname,
    Locale,
    Swapon,
    Keyboard,
    GrubInstall,
    GrubDefaultConfig,
    GrubAddScript,
    GrubRemoveScript,
    GrubMkconfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInstallation {
    pub chroot: bool,
    pub operation: PostInstallationOperation,
    pub params: Vec<String>,
}