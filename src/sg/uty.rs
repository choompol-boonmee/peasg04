use crate::sg::{dcl::DaVa, wk5};
use crate::web::wk5g;
use regex::{Captures, Regex};
use thousands::Separable;

pub trait NumForm {
    fn form(&self) -> String;
}
fn comma0(cm: String) -> String {
    let re = Regex::new(r"([0-9,]+)\.([0-9]+)").unwrap();
    let cm = re.replace(cm.as_str(), |caps: &Captures| {
        format!(
            "{}.{}",
            &caps[1],
            if caps[2].len() > 2 {
                format!("{}", &caps[2][0..2])
            } else {
                if caps[2].len() < 2 {
                    format!("{}0", &caps[2][0..1])
                } else {
                    format!("{}", &caps[2])
                }
            }
        )
    });
    format!("{}", cm)
}
impl NumForm for f32 {
    fn form(&self) -> String {
        comma0(self.separate_with_commas())
    }
}
impl NumForm for f64 {
    fn form(&self) -> String {
        comma0(self.separate_with_commas())
    }
}
impl NumForm for i32 {
    fn form(&self) -> String {
        comma0(self.separate_with_commas())
    }
}
impl NumForm for i64 {
    fn form(&self) -> String {
        comma0(self.separate_with_commas())
    }
}
impl NumForm for usize {
    fn form(&self) -> String {
        comma0(self.separate_with_commas())
    }
}

pub fn repo_sum(repo: &mut wk5g::Report, ssv: &Vec<wk5::Substation>) {
    if repo.rows.len() > 0 {
        repo.sums[0] = DaVa::None;
        for ci in 1..repo.cols.len() {
            repo.sums[ci] = match repo.dava(ssv, 0, ci) {
                DaVa::F32(_) => DaVa::F32(0.0),
                DaVa::F64(_) => DaVa::F64(0.0),
                DaVa::I32(_) => DaVa::I32(0),
                DaVa::I64(_) => DaVa::I64(0),
                DaVa::USZ(_) => DaVa::USZ(0),
                _ => DaVa::None,
            };
        }
        let mut txno = 0;
        for (ri, rr) in repo.rows.iter().enumerate() {
            if let DaVa::USZ(v) = repo.dava(ssv, ri, 5) {
                txno += v;
            }

            for ci in 0..repo.cols.len() {
                repo.sums[ci] = match repo.dava(ssv, ri, ci) {
                    DaVa::F32(v1) => {
                        if let DaVa::F32(v2) = repo.sums[ci] {
                            DaVa::F32(v1 + v2)
                        } else {
                            DaVa::F32(0.0)
                        }
                    }
                    DaVa::F64(v1) => {
                        if let DaVa::F64(v2) = repo.sums[ci] {
                            DaVa::F64(v1 + v2)
                        } else {
                            DaVa::F64(0.0)
                        }
                    }
                    DaVa::I32(v1) => {
                        if let DaVa::I32(v2) = repo.sums[ci] {
                            DaVa::I32(v1 + v2)
                        } else {
                            DaVa::I32(0)
                        }
                    }
                    DaVa::I64(v1) => {
                        if let DaVa::I64(v2) = repo.sums[ci] {
                            DaVa::I64(v1 + v2)
                        } else {
                            DaVa::I64(0)
                        }
                    }
                    DaVa::USZ(v1) => {
                        if let DaVa::USZ(v2) = repo.sums[ci] {
                            DaVa::USZ(v1 + v2)
                        } else {
                            DaVa::USZ(0)
                        }
                    }
                    _ => DaVa::None,
                };
            }
        }
    }
}
