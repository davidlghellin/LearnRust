use std::fmt;

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

const R: char = 'r';
const W: char = 'w';
const X: char = 'x';
const D: char = 'd';

type Permission = bool;

struct Group(Permission, Permission, Permission);
impl Group {
    fn new(mode: u32, read_per: u32, write_per: u32, exec_per: u32) -> Group {
        let (check_r, check_w, check_x) = (is_set(read_per), is_set(write_per), is_set(exec_per));
        Group(check_r(mode), check_w(mode), check_x(mode))
    }
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (read, write, exec) = (if_set(R), if_set(W), if_set(X));
        write!(f, "{}{}{}", read(self.0), write(self.1), exec(self.2))
    }
}

// rw-rwx-r--
pub struct Section {
    user: Group,
    group: Group,
    others: Group,
}
impl Section {
    pub fn new(mode: u32) -> Section {
        Section {
            user: Group::new(mode, S_IRUSR, S_IWUSR, S_IXUSR),
            group: Group::new(mode, S_IRGRP, S_IWGRP, S_IXGRP),
            others: Group::new(mode, S_IROTH, S_IWOTH, S_IXOTH),
        }
    }
}
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.user, self.group, self.others)
    }
}

fn if_set(c: char) -> impl Fn(bool) -> char {
    move |b: bool| {
        if b {
            c
        } else {
            D
        }
    }
}
fn is_set(permission: u32) -> impl Fn(u32) -> bool {
    move |mode: u32| mode & permission > 0
}
