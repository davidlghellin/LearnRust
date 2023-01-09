use users::{get_group_by_gid, get_user_by_uid, gid_t, uid_t};

pub fn get_user_name(uid: u32) -> Option<String> {
    let user = get_user_by_uid(uid as uid_t)?;

    Some(user.name().to_str()?.to_string())
}

pub fn get_group_name(gid: u32) -> Option<String> {
    let group = get_group_by_gid(gid as gid_t)?;
    
    Some(group.name().to_str()?.to_string())
}
