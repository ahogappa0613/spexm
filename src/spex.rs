use crate::chex::Chex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, Not};

type Sid = isize;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// 0
    Blank,
    /// 1
    Whole,
    /// 2
    Other,
}

#[derive(Debug, Clone)]
pub struct Spex {
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

    pub fn new_blank() -> Self {
        Self::build_by_chex(&Chex::new_blank())
    }

    pub fn new_whole() -> Self {
        Self::buid_whole()
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

    pub fn include(&self, other: &Self) -> bool {
        match self.kind {
            Kind::Blank => false,
            Kind::Whole => true,
            Kind::Other => match other.kind {
                Kind::Blank => true,
                Kind::Whole => false,
                Kind::Other => (&!self & other).blank(),
            },
        }
    }

    pub fn build_by_chex(chex: &Chex) -> Self {
        if chex.blank() {
            Self::new(
                vec![
                    Transition::new(0, 1, Chex::new_whole()),
                    Transition::new(1, 1, Chex::new_whole()),
                ],
                HashSet::from_iter([]),
            )
        } else {
            Self::new(
                vec![
                    Transition::new(0, 1, chex.clone()),
                    Transition::new(0, 2, !chex),
                    Transition::new(1, 2, Chex::new_whole()),
                    Transition::new(2, 2, Chex::new_whole()),
                ],
                HashSet::from_iter([1]),
            )
        }
    }

    pub fn buid_whole() -> Self {
        Self::new(
            vec![
                Transition::new(0, 1, Chex::new_whole()),
                Transition::new(1, 1, Chex::new_whole()),
            ],
            HashSet::from_iter([1]),
        )
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut new_tn_list: Vec<Transition> = vec![];
        let mut new_accepts_sids: HashSet<Sid> = HashSet::new();

        Spex::calc_concat(
            0,
            &mut new_tn_list,
            &mut new_accepts_sids,
            self,
            other,
            0,
            HashSet::new(),
            &mut SidGen::new(),
            &mut HashMap::new(),
        );

        Spex::new(new_tn_list, new_accepts_sids)
    }

    pub fn repeat(&self) -> Self {
        let mut new_tn_list: Vec<Transition> = vec![];
        let mut new_accepts_sids: HashSet<Sid> = HashSet::new();
        let mut sids = HashSet::new();
        sids.insert(0);

        Spex::calc_repeat(
            0,
            &mut new_tn_list,
            &mut new_accepts_sids,
            self,
            sids,
            &mut SidGen::new(),
            &mut HashMap::new(),
        );

        Spex::new(new_tn_list, new_accepts_sids)
    }

    pub fn mermaid(&self) -> String {
        let mut ret = String::from("```mermaid\ngraph LR\n");
        for sid in self.tns_dict.keys() {
            match sid {
                0 => ret += "    0(( ))\n",
                -1 => ret += "    -1(( ))\n",
                -2 => ret += "    -2(( ))\n",
                _ => ret += &format!("    {}( )\n", sid),
            }
        }

        for sid in self.tns_dict.keys() {
            match sid {
                0 => ret += "    style 0 fill:#000,stroke-width:0px\n",
                -1 => ret += "    style -1 fill:#adb5bd,stroke-width:0px\n",
                -2 => ret += "    style -2 fill:#adb5bd,stroke:#dc3545,stroke-width:4px\n",
                _ => {
                    if self.accepts_sids.contains(sid) {
                        ret += &format!("    style {} stroke:#dc3545,stroke-width:4px\n", sid)
                    }
                }
            }
        }

        for tns in self.tns_dict.values() {
            for tn in tns {
                ret += &format!("    {} -- \"{}\" --> {}\n", tn.fr_sid, tn.chex, tn.to_sid)
            }
        }

        ret += "```";

        ret
    }

    pub fn calc_and_or(
        new_fr_sid: Sid,
        new_tn_list: &mut Vec<Transition>,
        new_accepts_sids: &mut HashSet<Sid>,
        spex1: &Self,
        spex2: &Self,
        spex1_sid: Sid,
        spex2_sid: Sid,
        sid_gen: &mut SidGen,
        sid_dict_by_skey: &mut HashMap<String, Sid>,
        ope_kind: usize,
    ) {
        let mut chex_pattern = vec![Chex::new_whole()];
        let mut tmp_chex_pattern: Vec<Chex> = vec![];
        for ref target_chex in chex_pattern {
            for tn in &spex1.tns_dict[&spex1_sid] {
                let and_chex = target_chex & &tn.chex;
                if !&and_chex.blank() {
                    tmp_chex_pattern.push(and_chex);
                }
            }
        }
        chex_pattern = tmp_chex_pattern;

        tmp_chex_pattern = vec![];
        for ref target_chex in chex_pattern {
            for tn in &spex2.tns_dict[&spex2_sid] {
                let and_chex = target_chex & &tn.chex;
                if !&and_chex.blank() {
                    tmp_chex_pattern.push(and_chex);
                }
            }
        }
        chex_pattern = tmp_chex_pattern;

        for target_chex in chex_pattern {
            // 2-1
            // 必ず見つかる
            let next_spex1_sid = spex1.tns_dict[&spex1_sid]
                .iter()
                .find(|tn| tn.chex.include(&target_chex))
                .unwrap()
                .to_sid;

            let next_spex2_sid = spex2.tns_dict[&spex2_sid]
                .iter()
                .find(|tn| tn.chex.include(&target_chex))
                .unwrap()
                .to_sid;

            // 2-2
            let skey = format!("{}/{}", next_spex1_sid, next_spex2_sid);
            let new_to_sid: Sid;
            if sid_dict_by_skey.contains_key(&skey) {
                // 2-2-1
                new_to_sid = sid_dict_by_skey[&skey];
            } else {
                // 2-2-2
                // 2-2-2-1
                new_to_sid = sid_gen.get();

                // 2-2-2-2
                sid_dict_by_skey.insert(skey, new_to_sid);
                // 2-2-2-3
                if ope_kind == 0 {
                    if spex1.accepts_sids.contains(&next_spex1_sid)
                        || spex2.accepts_sids.contains(&next_spex2_sid)
                    {
                        new_accepts_sids.insert(new_to_sid);
                    }
                } else if ope_kind == 1 {
                    if spex1.accepts_sids.contains(&next_spex1_sid)
                        && spex2.accepts_sids.contains(&next_spex2_sid)
                    {
                        new_accepts_sids.insert(new_to_sid);
                    }
                } else {
                    unreachable!()
                }

                // 2-2-2-4
                Self::calc_and_or(
                    new_to_sid,
                    new_tn_list,
                    new_accepts_sids,
                    spex1,
                    spex2,
                    next_spex1_sid,
                    next_spex2_sid,
                    sid_gen,
                    sid_dict_by_skey,
                    ope_kind,
                );
            }
            // 2-3
            new_tn_list.push(Transition::new(new_fr_sid, new_to_sid, target_chex));
        }
    }

    pub fn calc_concat(
        new_fr_sid: Sid,
        new_tn_list: &mut Vec<Transition>,
        new_accepts_sids: &mut HashSet<Sid>,
        spex1: &Self,
        spex2: &Self,
        spex1_sid: Sid,
        spex2_sids: HashSet<Sid>,
        sid_gen: &mut SidGen,
        sid_dict_by_skey: &mut HashMap<String, Sid>,
    ) {
        let mut chex_pattern = vec![Chex::new_whole()];
        let mut tmp_chex_pattern: Vec<Chex> = vec![];
        for ref target_chex in chex_pattern {
            for tn in &spex1.tns_dict[&spex1_sid] {
                let and_chex = target_chex & &tn.chex;
                if !&and_chex.blank() {
                    tmp_chex_pattern.push(and_chex);
                }
            }
        }
        chex_pattern = tmp_chex_pattern;

        if spex1.accepts_sids.contains(&spex1_sid) {
            tmp_chex_pattern = vec![];
            for ref target_chex in chex_pattern {
                for tn in spex2.tns_dict.get(&0).unwrap() {
                    let and_chex = target_chex & &tn.chex;
                    if !&and_chex.blank() {
                        tmp_chex_pattern.push(and_chex);
                    }
                }
            }
            chex_pattern = tmp_chex_pattern;
        }

        for spex2_sid in &spex2_sids {
            tmp_chex_pattern = vec![];
            for ref target_chex in chex_pattern {
                for tn in spex2.tns_dict.get(spex2_sid).unwrap() {
                    let and_chex = target_chex & &tn.chex;
                    if !&and_chex.blank() {
                        tmp_chex_pattern.push(and_chex);
                    }
                }
            }
            chex_pattern = tmp_chex_pattern;
        }

        // 2
        //let mut next_spex1_sid: Sid;
        for target_chex in chex_pattern {
            // 2-1
            let next_spex1_sid = spex1.tns_dict[&spex1_sid]
                .iter()
                .find(|tn| tn.chex.include(&target_chex))
                .unwrap()
                .to_sid;

            let mut next_spex2_sids: HashSet<Sid> = HashSet::new();

            if spex1.accepts_sids.contains(&spex1_sid) {
                // spex1が受理状態の場合は、spex2の最初の遷移を考慮する
                for tn in spex2.tns_dict.get(&0).unwrap() {
                    if tn.chex.include(&target_chex) {
                        next_spex2_sids.insert(tn.to_sid);
                        break;
                    }
                }
            }
            for spex2_sid in &spex2_sids {
                for tn in spex2.tns_dict.get(spex2_sid).unwrap() {
                    if tn.chex.include(&target_chex) {
                        next_spex2_sids.insert(tn.to_sid);
                        break;
                    }
                }
            }

            // 2-2
            let skey = if next_spex2_sids.len() == 0 {
                format!("{}/", next_spex1_sid)
            } else {
                let mut vec_sid: Vec<Sid> = next_spex2_sids.clone().into_iter().collect();
                vec_sid.sort();
                format!(
                    "{}/{}",
                    next_spex1_sid,
                    vec_sid
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("-")
                )
            };

            let new_to_sid: Sid;
            if sid_dict_by_skey.contains_key(&skey) {
                // 2-2-1
                new_to_sid = sid_dict_by_skey[&skey];
            } else {
                // 2-2-2
                // 2-2-2-1
                new_to_sid = sid_gen.get();
                // 2-2-2-2
                sid_dict_by_skey.insert(skey, new_to_sid);
                // 2-2-2-3
                for next_spex2_sid in &next_spex2_sids {
                    if spex2.accepts_sids.contains(next_spex2_sid) {
                        new_accepts_sids.insert(new_to_sid);
                        break;
                    }
                }
                // 2-2-2-4
                Self::calc_concat(
                    new_to_sid,
                    new_tn_list,
                    new_accepts_sids,
                    spex1,
                    spex2,
                    next_spex1_sid,
                    next_spex2_sids,
                    sid_gen,
                    sid_dict_by_skey,
                )
            }
            new_tn_list.push(Transition::new(new_fr_sid, new_to_sid, target_chex));
        }
    }

    pub fn calc_repeat(
        new_fr_sid: Sid,
        new_tn_list: &mut Vec<Transition>,
        new_accepts_sids: &mut HashSet<Sid>,
        spex: &Self,
        spex_sids: HashSet<Sid>,
        sid_gen: &mut SidGen,
        sid_dict_by_skey: &mut HashMap<String, Sid>,
    ) {
        let mut chex_pattern = vec![Chex::new_whole()];

        for spex_sid in spex_sids.iter() {
            let mut tmp_chex_pattern: Vec<Chex> = vec![];
            for target_chex in &chex_pattern {
                for tn in &spex.tns_dict[spex_sid] {
                    let and_chex = target_chex & &tn.chex;
                    if !&and_chex.blank() {
                        tmp_chex_pattern.push(and_chex);
                    }
                }
            }
            chex_pattern = tmp_chex_pattern.clone();
        }
        for spex_sid in spex_sids.iter() {
            if spex.accepts_sids.contains(&spex_sid) {
                let mut tmp_chex_pattern: Vec<Chex> = vec![];
                for target_chex in &chex_pattern {
                    for tn in spex.tns_dict.get(&0).unwrap() {
                        let and_chex = target_chex & &tn.chex;
                        if !&and_chex.blank() {
                            tmp_chex_pattern.push(and_chex);
                        }
                    }
                }
                chex_pattern = tmp_chex_pattern.clone();
                break;
            }
        }
        // 2
        for target_chex in chex_pattern {
            let mut next_spex_sids: HashSet<Sid> = HashSet::new();
            for spex_sid in spex_sids.iter() {
                if spex.accepts_sids.contains(spex_sid) {
                    // 受理状態の場合は、最初の遷移を考慮する
                    for tn in spex.tns_dict.get(&0).unwrap() {
                        if tn.chex.include(&target_chex) {
                            next_spex_sids.insert(tn.to_sid);
                            break;
                        }
                    }
                    break;
                }
            }
            for spex_sid in spex_sids.iter() {
                for tn in spex.tns_dict.get(&spex_sid).unwrap() {
                    if tn.chex.include(&target_chex) {
                        next_spex_sids.insert(tn.to_sid);
                        break;
                    }
                }
            }
            let mut vec_sid: Vec<Sid> = next_spex_sids.clone().into_iter().collect();
            vec_sid.sort();
            let skey = vec_sid
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("-");

            let new_to_sid: Sid;
            if sid_dict_by_skey.contains_key(&skey) {
                new_to_sid = sid_dict_by_skey[&skey]
            } else {
                // 2-2-2
                // 2-2-2-1
                new_to_sid = sid_gen.get();
                // 2-2-2-2
                sid_dict_by_skey.insert(skey, new_to_sid);
                // 2-2-2-3
                for next_spex_sid in &next_spex_sids {
                    if spex.accepts_sids.contains(next_spex_sid) {
                        new_accepts_sids.insert(new_to_sid);
                        break;
                    }
                }
                // 2-2-2-4
                Self::calc_repeat(
                    new_to_sid,
                    new_tn_list,
                    new_accepts_sids,
                    spex,
                    next_spex_sids,
                    sid_gen,
                    sid_dict_by_skey,
                )
            }
            new_tn_list.push(Transition::new(new_fr_sid, new_to_sid, target_chex))
        }
    }
}

impl PartialEq for Spex {
    fn eq(&self, other: &Self) -> bool {
        if self.kind == other.kind {
            if self.kind != Kind::Other {
                return true;
            } else {
                return self.include(other) && other.include(self);
            }
        } else {
            return false;
        }
    }
}

impl Not for &Spex {
    type Output = Spex;

    fn not(self) -> Self::Output {
        let mut tn_list: Vec<Transition> = vec![];
        let mut accepts_sids: HashSet<Sid> = HashSet::new();

        for (sid, _trn) in &self.tns_dict {
            if *sid != 0 && !self.accepts_sids.contains(&sid) {
                accepts_sids.insert(*sid);
            }
            tn_list.extend_from_slice(&self.tns_dict[&sid]);
        }

        Spex::new(tn_list, accepts_sids)
    }
}

impl BitOr for &Spex {
    type Output = Spex;

    fn bitor(self, other: Self) -> Self::Output {
        match self.kind {
            Kind::Blank => other.clone(),
            Kind::Whole => self.clone(),
            Kind::Other => match other.kind {
                Kind::Blank => self.clone(),
                Kind::Whole => other.clone(),
                Kind::Other => {
                    let mut new_tn_list: Vec<Transition> = vec![];
                    let mut new_accepts_sids: HashSet<Sid> = HashSet::new();
                    Spex::calc_and_or(
                        0,
                        &mut new_tn_list,
                        &mut new_accepts_sids,
                        &self,
                        &other,
                        0,
                        0,
                        &mut SidGen::new(),
                        &mut HashMap::new(),
                        0,
                    );

                    Spex::new(new_tn_list, new_accepts_sids)
                }
            },
        }
    }
}

impl BitAnd for &Spex {
    type Output = Spex;

    fn bitand(self, other: Self) -> Self::Output {
        match self.kind {
            Kind::Blank => self.clone(),
            Kind::Whole => other.clone(),
            Kind::Other => match other.kind {
                Kind::Blank => other.clone(),
                Kind::Whole => self.clone(),
                Kind::Other => {
                    let mut new_tn_list: Vec<Transition> = vec![];
                    let mut new_accepts_sids: HashSet<Sid> = HashSet::new();
                    Spex::calc_and_or(
                        0,
                        &mut new_tn_list,
                        &mut new_accepts_sids,
                        &self,
                        &other,
                        0,
                        0,
                        &mut SidGen::new(),
                        &mut HashMap::new(),
                        1,
                    );

                    Spex::new(new_tn_list, new_accepts_sids)
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    fr_sid: Sid,
    to_sid: Sid,
    chex: Chex,
}

impl Transition {
    pub fn new(fr_sid: Sid, to_sid: Sid, chex: Chex) -> Self {
        Self {
            fr_sid,
            to_sid,
            chex,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SidGen {
    pub sid: Sid,
}

impl SidGen {
    pub fn new() -> Self {
        Self { sid: 0 }
    }

    pub fn get(&mut self) -> isize {
        self.sid += 1;
        self.sid
    }
}
