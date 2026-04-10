/*!
# cuda-skill

Skill system for agents.

Skills are different from goals (what to do) and reflexes (automatic reactions).
Skills are LEARNED CAPABILITIES that improve with practice.

This crate provides:
- Skill definition with proficiency tracking
- Skill tree with prerequisites
- Practice sessions that improve proficiency
- Skill sharing between agents
- Skill decay from disuse
- Skill synergy (complementary skills boost each other)
*/

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Skill proficiency level
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Proficiency {
    Novice = 0,
    Beginner = 1,
    Competent = 2,
    Proficient = 3,
    Expert = 4,
    Master = 5,
}

impl Proficiency {
    pub fn label(&self) -> &'static str {
        match self {
            Proficiency::Novice => "novice",
            Proficiency::Beginner => "beginner",
            Proficiency::Competent => "competent",
            Proficiency::Proficient => "proficient",
            Proficiency::Expert => "expert",
            Proficiency::Master => "master",
        }
    }

    pub fn from_progress(p: f64) -> Proficiency {
        match p {
            p if p < 0.1 => Proficiency::Novice,
            p if p < 0.25 => Proficiency::Beginner,
            p if p < 0.5 => Proficiency::Competent,
            p if p < 0.75 => Proficiency::Proficient,
            p if p < 0.95 => Proficiency::Expert,
            _ => Proficiency::Master,
        }
    }

    /// Fractional progress [0, 1]
    pub fn progress(&self) -> f64 {
        match self {
            Proficiency::Novice => 0.0,
            Proficiency::Beginner => 0.15,
            Proficiency::Competent => 0.4,
            Proficiency::Proficient => 0.65,
            Proficiency::Expert => 0.85,
            Proficiency::Master => 1.0,
        }
    }
}

/// A skill
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,         // combat, navigation, social, craft, etc.
    pub progress: f64,            // [0, 1] continuous
    pub practice_count: u32,
    pub total_practice_time_ms: u64,
    pub last_practiced: u64,
    pub prerequisites: Vec<String>,
    pub synergies: Vec<String>,    // skills that boost this one
    pub difficulty: f64,          // [0, 1] how hard to learn
    pub decay_rate: f64,          // how fast progress decays without practice
}

impl Skill {
    pub fn new(id: &str, name: &str, category: &str) -> Self {
        Skill { id: id.to_string(), name: name.to_string(), description: String::new(), category: category.to_string(), progress: 0.0, practice_count: 0, total_practice_time_ms: 0, last_practiced: 0, prerequisites: vec![], synergies: vec![], difficulty: 0.5, decay_rate: 0.001 }
    }

    pub fn proficiency(&self) -> Proficiency { Proficiency::from_progress(self.progress) }

    /// Practice the skill — improve progress with diminishing returns
    pub fn practice(&mut self, duration_ms: u64, synergy_bonus: f64) {
        self.practice_count += 1;
        self.total_practice_time_ms += duration_ms;
        self.last_practiced = now();

        // Power law of practice: improvement is proportional to 1/(practice+1)
        let base_improvement = 0.1 / (self.practice_count as f64 + 1.0);
        let synergy_factor = 1.0 + synergy_bonus * 0.2; // synergies boost learning by up to 20%
        let difficulty_factor = 1.0 - self.difficulty * 0.5; // hard skills improve slower

        let improvement = base_improvement * synergy_factor * difficulty_factor;
        self.progress = (self.progress + improvement).min(1.0);
    }

    /// Decay from disuse
    pub fn decay(&mut self, current_time: u64) {
        if self.last_practiced == 0 { return; }
        let elapsed = current_time.saturating_sub(self.last_practiced) as f64 / 3_600_000.0; // hours
        if elapsed > 0.0 {
            self.progress = (self.progress - elapsed * self.decay_rate * self.progress).max(0.0);
        }
    }

    /// Is this skill usable at all?
    pub fn is_usable(&self) -> bool { self.progress >= 0.1 }

    /// Effectiveness modifier based on proficiency
    pub fn effectiveness(&self) -> f64 {
        // Skill effectiveness scales non-linearly with progress
        self.progress * self.progress * 1.5 // quadratic: low skill = very ineffective, high skill = very effective
    }
}

/// Skill tree — prerequisites and progression
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTree {
    pub skills: HashMap<String, Skill>,
    pub categories: HashSet<String>,
}

impl SkillTree {
    pub fn new() -> Self { SkillTree { skills: HashMap::new(), categories: HashSet::new() } }

    pub fn add(&mut self, skill: Skill) {
        self.categories.insert(skill.category.clone());
        self.skills.insert(skill.id.clone(), skill);
    }

    /// Can this skill be learned? (prerequisites met)
    pub fn can_learn(&self, skill_id: &str) -> bool {
        let skill = match self.skills.get(skill_id) { Some(s) => s, None => return false };
        skill.prerequisites.iter().all(|pre| {
            self.skills.get(pre).map_or(false, |p| p.is_usable())
        })
    }

    /// Available skills to learn next (prerequisites met, not yet mastered)
    pub fn available(&self) -> Vec<&Skill> {
        self.skills.values()
            .filter(|s| self.can_learn(&s.id) && s.progress < 1.0)
            .collect()
    }

    /// Skills at a proficiency level
    pub fn at_level(&self, level: Proficiency) -> Vec<&Skill> {
        self.skills.values().filter(|s| s.proficiency() == level).collect()
    }

    /// Practice a skill if prerequisites are met
    pub fn practice(&mut self, skill_id: &str, duration_ms: u64) -> bool {
        if !self.can_learn(skill_id) { return false; }
        let synergy_bonus = self.synergy_bonus(skill_id);
        if let Some(skill) = self.skills.get_mut(skill_id) {
            skill.practice(duration_ms, synergy_bonus);
            return true;
        }
        false
    }

    /// Calculate synergy bonus from related skills
    fn synergy_bonus(&self, skill_id: &str) -> f64 {
        let skill = match self.skills.get(skill_id) { Some(s) => s, None => return 0.0 };
        skill.synergies.iter()
            .filter_map(|sid| self.skills.get(sid))
            .map(|s| s.effectiveness())
            .sum::<f64>()
            .min(1.0)
    }

    /// Decay all skills
    pub fn decay_all(&mut self, current_time: u64) {
        for skill in self.skills.values_mut() { skill.decay(current_time); }
    }

    /// Summary
    pub fn summary(&self) -> SkillSummary {
        let total = self.skills.len();
        let usable: usize = self.skills.values().filter(|s| s.is_usable()).count();
        let mastered: usize = self.skills.values().filter(|s| s.progress >= 0.95).count();
        let avg_progress = if total > 0 {
            self.skills.values().map(|s| s.progress).sum::<f64>() / total as f64
        } else { 0.0 };
        SkillSummary { total, usable, mastered, avg_progress, categories: self.categories.len() }
    }
}

#[derive(Clone, Debug)]
pub struct SkillSummary {
    pub total: usize,
    pub usable: usize,
    pub mastered: usize,
    pub avg_progress: f64,
    pub categories: usize,
}

/// Skill sharing — export/import between agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillShare {
    pub skill_id: String,
    pub shared_progress: f64,     // how much to transfer
    pub required_proficiency: Proficiency, // minimum to share
    pub transfer_rate: f64,       // fraction of progress transferred
}

impl SkillShare {
    /// Create a share from an agent's skill
    pub fn from_skill(skill: &Skill) -> Option<Self> {
        if skill.proficiency() < Proficiency::Competent { return None; }
        Some(SkillShare {
            skill_id: skill.id.clone(),
            shared_progress: skill.progress * 0.5, // share half knowledge
            required_proficiency: Proficiency::Competent,
            transfer_rate: 0.3, // receiver gets 30% of shared
        })
    }

    /// Apply shared knowledge to a receiving agent's skill
    pub fn apply(&self, skill: &mut Skill) {
        let transfer = self.shared_progress * self.transfer_rate;
        skill.progress = (skill.progress + transfer * (1.0 - skill.progress)).min(1.0);
        // Learning from others is less effective than direct practice
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let s = Skill::new("fishing", "Fishing", "survival");
        assert_eq!(s.proficiency(), Proficiency::Novice);
        assert!(!s.is_usable());
    }

    #[test]
    fn test_skill_practice() {
        let mut s = Skill::new("fish", "Fish", "survival");
        s.practice(1000, 0.0);
        assert!(s.progress > 0.0);
        assert_eq!(s.practice_count, 1);
    }

    #[test]
    fn test_proficiency_progression() {
        let mut s = Skill::new("x", "X", "y");
        for _ in 0..200 { s.practice(1000, 0.0); }
        assert!(s.proficiency() >= Proficiency::Competent);
    }

    #[test]
    fn test_skill_decay() {
        let mut s = Skill::new("x", "X", "y");
        for _ in 0..50 { s.practice(1000, 0.0); }
        let before = s.progress;
        s.decay(now() + 86400_000 * 10); // 10 days later
        assert!(s.progress < before);
    }

    #[test]
    fn test_effectiveness_quadratic() {
        let mut s = Skill::new("x", "X", "y");
        s.progress = 0.1;
        let low = s.effectiveness();
        s.progress = 0.9;
        let high = s.effectiveness();
        assert!(high > low * 10); // quadratic scaling
    }

    #[test]
    fn test_skill_tree_prerequisites() {
        let mut tree = SkillTree::new();
        tree.add(Skill::new("basic", "Basic", "nav"));
        tree.add({ let mut s = Skill::new("advanced", "Advanced", "nav"); s.prerequisites = vec!["basic".into()]; s });
        assert!(tree.can_learn("basic"));
        assert!(!tree.can_learn("advanced")); // basic not usable yet
    }

    #[test]
    fn test_available_after_prereq() {
        let mut tree = SkillTree::new();
        tree.add(Skill::new("basic", "Basic", "x"));
        tree.add({ let mut s = Skill::new("adv", "Adv", "x"); s.prerequisites = vec!["basic".into()]; s });
        tree.practice("basic", 5000); // make basic usable
        assert!(tree.available().iter().any(|s| s.id == "adv"));
    }

    #[test]
    fn test_synergy_bonus() {
        let mut tree = SkillTree::new();
        tree.add({ let mut s = Skill::new("a", "A", "x"); s.synergies = vec!["b".into()]; s });
        tree.add({ let mut s = Skill::new("b", "B", "x"); s.progress = 0.8; s }); // high b
        tree.practice("a", 1000);
        // a should benefit from b's synergy
    }

    #[test]
    fn test_skill_share() {
        let mut sharer = Skill::new("fish", "Fish", "survival");
        for _ in 0..100 { sharer.practice(1000, 0.0); }
        let share = SkillShare::from_skill(&sharer);
        assert!(share.is_some());
    }

    #[test]
    fn test_skill_share_apply() {
        let mut sharer = Skill::new("fish", "Fish", "survival");
        for _ in 0..100 { sharer.practice(1000, 0.0); }
        let share = SkillShare::from_skill(&sharer).unwrap();
        let mut receiver = Skill::new("fish", "Fish", "survival");
        let before = receiver.progress;
        share.apply(&mut receiver);
        assert!(receiver.progress > before);
    }

    #[test]
    fn test_no_share_novice() {
        let s = Skill::new("fish", "Fish", "survival"); // novice
        assert!(SkillShare::from_skill(&s).is_none());
    }

    #[test]
    fn test_tree_summary() {
        let mut tree = SkillTree::new();
        tree.add(Skill::new("a", "A", "cat1"));
        tree.add(Skill::new("b", "B", "cat2"));
        let s = tree.summary();
        assert_eq!(s.total, 2);
        assert_eq!(s.categories, 2);
    }

    #[test]
    fn test_practice_blocked() {
        let mut tree = SkillTree::new();
        tree.add({ let mut s = Skill::new("adv", "Adv", "x"); s.prerequisites = vec!["basic".into()]; s });
        assert!(!tree.practice("adv", 1000)); // prereq not met
    }
}
