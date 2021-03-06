use {
    lfs_core::*,
    serde_json::{
        json,
        Value,
    },
};

pub fn output_value(mounts: &[Mount]) -> Value {
    let mut jmounts = Vec::new();
    for mount in mounts {
        let stats = mount.stats.as_ref().map(|s| json!({
            "bsize": s.bsize,
            "blocks": s.blocks,
            "bavail": s.bavail,
            "bfree": s.bfree,
            "size": file_size::fit_4(s.size()),
            "used": file_size::fit_4(s.used()),
            "used-percent": format!("{:.0}%", 100.0*s.use_share()),
            "available": file_size::fit_4(s.available()),
        }));
        let disk = mount.disk.as_ref().map(|d| json!({
            "type": d.disk_type(),
            "rotational": d.rotational,
            "removable": d.removable,
        }));
        jmounts.push(json!({
            "id": mount.info.id,
            "dev": {
                "major": mount.info.dev.major,
                "minor": mount.info.dev.minor,
            },
            "fs": mount.info.fs,
            "fs-type": mount.info.fs_type,
            "mount-point": mount.info.mount_point,
            "disk": disk,
            "stats": stats,
        }));
    }
    Value::Array(jmounts)
}
