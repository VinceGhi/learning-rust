pub struct Player {
    pub points: u8,
    pub health: u16,
    pub str: u8,
    pub vit: u8,
    pub class: String
}

// public stuff
impl Player {
    pub fn upgrade_stat(&mut self, skill_type: SkillType) -> bool {
        if self.points <= 0 {
            return false;
        }
        
        match skill_type {
            SkillType::Vitality => self.vit += 1,
            SkillType::Strength => self.str += 1
        }
        
        self.points -= 1;

        return true;
    }

    pub fn set_stats(&mut self, class: String, str: u8, vit: u8) {
        self.class = class;
        self.str = str;
        self.vit = vit;
    }
}

// private stuff
impl Player {
    fn max_health(&self) -> u16{
        return 100 + (self.vit as u16 * 5);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { points: 5, health: 100, str: 0, vit: 0, class: "None".to_string() }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String{
        return format!("\n\tAvailable Points: {}\n\tClass: {}\n\tStrength: {}\n\tVitality: {}\n\tMax Health: {}/{}", self.points, self.class, self.str, self.vit, self.health, self.max_health());
    }
}

pub enum SkillType {
    Vitality,
    Strength
}