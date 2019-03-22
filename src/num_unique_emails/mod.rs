pub struct Solution929{}
use std::collections::HashSet;

impl Solution929 {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut v: HashSet<_> = HashSet::new();

        for (i, s) in emails.iter().enumerate() {
            let mut v1: Vec<&str> = s.splitn(2, "@").collect();
            let v2: Vec<&str> = v1[0].splitn(2, "+").collect();

            let s = v2[0].replace(".", "");
            v1[0] = &s;

            v.insert(v1.join("@").to_string());
        }

        v.len() as i32
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn exploration() {
        use super::*;
        let v1 = vec![
            "fg.r.u.uzj+o.pw@kziczvh.com",
            "r.cyo.g+d.h+b.ja@tgsg.z.com",
            "fg.r.u.uzj+o.f.d@kziczvh.com",
            "r.cyo.g+ng.r.iq@tgsg.z.com",
            "fg.r.u.uzj+lp.k@kziczvh.com",
            "r.cyo.g+n.h.e+n.g@tgsg.z.com",
            "fg.r.u.uzj+k+p.j@kziczvh.com",
            "fg.r.u.uzj+w.y+b@kziczvh.com",
            "r.cyo.g+x+d.c+f.t@tgsg.z.com",
            "r.cyo.g+x+t.y.l.i@tgsg.z.com",
            "r.cyo.g+brxxi@tgsg.z.com",
            "r.cyo.g+z+dr.k.u@tgsg.z.com",
            "r.cyo.g+d+l.c.n+g@tgsg.z.com",
            "fg.r.u.uzj+vq.o@kziczvh.com",
            "fg.r.u.uzj+uzq@kziczvh.com",
            "fg.r.u.uzj+mvz@kziczvh.com",
            "fg.r.u.uzj+taj@kziczvh.com",
            "fg.r.u.uzj+fek@kziczvh.com"];
        let mut v = toStringVec(v1);

        assert_eq!(2, Solution929::num_unique_emails(v));
    }


    fn toStringVec (v: Vec<&str>) -> Vec<String>{
        let mut v1: Vec<String> = Vec::new();

        for s in &v {
            //let s1 = String::from(s);
            v1.push(s.to_string());
        }

        v1

    }
}