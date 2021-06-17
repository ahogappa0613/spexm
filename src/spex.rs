use crate::chex::Chex;
use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr, Not};

type Sid = usize;

enum Kind {
    Blank,
    Whole,
    Other,
}
struct Spex {
    pub accepts_sids: HashSet<Sid>,
    pub tns_dict: HashMap<Sid, Vec<Transition>>,
    pub kind: Kind,
}

impl Spex {
    pub fn new(tn_list: Vec<Transition>, accepts_sids: HashSet<Sid>) -> Self {
        let mut tns_dict: HashMap<Sid, Vec<Transition>> = HashMap::new();

        for tn in tn_list {
            let fr_sid = tn.fr_sid;
            if !tns_dict.contains_key(&fr_sid) {
                tns_dict.insert(fr_sid, vec![]);
            }

            tns_dict.get_mut(&fr_sid).unwrap().push(tn);
        }

        let kind = if accepts_sids.len() == 0 {
            Kind::Blank // 空集合
        } else if accepts_sids.len() == tns_dict.len() - 1 {
            Kind::Whole // 全集合
        } else {
            Kind::Other // それ以外？
        };

        Self {
            accepts_sids,
            tns_dict,
            kind,
        }
    }

    pub fn blank(&self) -> bool {
        match self.kind {
            Kind::Blank => true,
            _ => false,
        }
    }

    pub fn whole(&self) -> bool {
        match self.kind {
            Kind::Whole => true,
            _ => false,
        }
    }

    pub fn calc_and_or(
        new_fr_sid: Sid,
        new_tn_list: Vec<Transition>,
        new_accepts_sids: HashSet<Sid>,
        spex1: Self,
        spex2: Self,
        spex1_sid: Sid,
        spex2_sid: Sid,
        sid_gen: SidGen,
        sid_dict_by_skey: HashMap<Sid, Vec<Transition>>,
        ope_kind: usize,
    ) {
    }
}

impl Not for &Spex {
    type Output = Spex;

    fn not(self) -> Self::Output {
        let mut tn_list: Vec<Transition> = vec![];
        let mut accepts_sids: HashSet<usize> = HashSet::new();

        for (sid, _trn) in &self.tns_dict {
            if *sid != 0 && !self.accepts_sids.contains(&sid) {
                accepts_sids.insert(*sid);
            }
            tn_list.extend_from_slice(self.tns_dict.get(&sid).unwrap());
        }

        Spex::new(tn_list, accepts_sids)
    }
}

impl BitOr for Spex {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        match self.kind {
            Kind::Blank => other,
            Kind::Whole => self,
            Kind::Other => match other.kind {
                Kind::Blank => self,
                Kind::Whole => other,
                Kind::Other => {
                    let new_tn_list: Vec<Transition> = vec![];
                    let new_accepts_sids: HashSet<Sid> = HashSet::new();
                    Spex::new(new_tn_list, new_accepts_sids)
                }
            },
        }
    }
}

#[derive(Clone)]
struct Transition {
    fr_sid: Sid,
    to_sid: Sid,
    chex: Chex,
}

impl Transition {
    pub fn new(fr_sid: usize, to_sid: usize, chex: Chex) -> Self {
        Self {
            fr_sid,
            to_sid,
            chex,
        }
    }
}

struct SidGen {
    pub sid: usize,
}

impl SidGen {
    pub fn new() -> Self {
        Self { sid: 0 }
    }

    pub fn get(&mut self) -> usize {
        self.sid += 1;
        self.sid
    }
}
