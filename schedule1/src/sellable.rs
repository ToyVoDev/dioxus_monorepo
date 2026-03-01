use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OneTimeIngredient {
    PGR,
    Fertilizer,
    SpeedGrow,
}

#[derive(Clone, Default, PartialEq)]
pub struct MixState {
    pub ingredients: HashSet<OneTimeIngredient>,
    pub soil_quality: Quality,
    pub pseudo_quality: Quality,
    /// in oposition to using a grow tent
    pub use_pot: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sellable {
    pub base: Product,
    pub name: String,
    pub effects: HashSet<Effect>,
    pub ingredients: Vec<Ingredient>,
}

impl Sellable {
    pub fn sell_price(&self) -> f32 {
        let multiplier_sum = self
            .effects
            .iter()
            .map(|effect| effect.multiplier())
            .sum::<f32>();
        let price = self.base.sell_price() * (1. + multiplier_sum);
        // mimicking unity roundToInt which on 0.5, rounds towards the even number.
        if (price.fract() - 0.5).abs() < 0.001 {
            let int = price.floor();
            if int % 2. == 0. {
                int
            } else {
                int + 1.
            }
        } else {
            price.round()
        }
    }
    pub fn from_product(product: Product) -> Self {
        let (effects, name) = if let Product::Marijuana(effect) = product {
            (
                HashSet::from([effect]),
                match effect {
                    Effect::Calming => "OG Kush".to_string(),
                    Effect::Refreshing => "Sour Diesel".to_string(),
                    Effect::Energizing => "Green Crack".to_string(),
                    Effect::Sedating => "Granddaddy Purple".to_string(),
                    _ => unreachable!("{effect:?} is not a valid starting effect for marijuana"),
                },
            )
        } else {
            (HashSet::new(), format!("{:?}", product))
        };
        Sellable {
            base: product,
            effects,
            ingredients: Vec::new(),
            name,
        }
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> Self {
        let new_effect = ingredient.effect();
        let mut reaction_list = vec![];
        // let vec = new_effect.mix_direction() * new_effect.mix_megnitude();
        // let mixer_map = MixerMap::from_drug_type(self.drug_type());
        for effect in self.effects.iter() {
            // let point =  mixer_map.get_effect(effect).position + vec;
            // if let Some(replacement) = mixer_map.get_effect_at(point) {
            //     reaction_list.push((effect.clone(), replacement));
            // }
            match (effect, new_effect) {
                (Effect::AntiGravity, Effect::CalorieDense) => {
                    reaction_list.push((Effect::AntiGravity, Effect::Slippery))
                }
                (Effect::AntiGravity, Effect::LongFaced) => {
                    reaction_list.push((Effect::AntiGravity, Effect::Calming))
                }
                (Effect::AntiGravity, Effect::Spicy) => {
                    reaction_list.push((Effect::AntiGravity, Effect::TropicThunder))
                }
                (Effect::Athletic, Effect::Foggy) => {
                    reaction_list.push((Effect::Athletic, Effect::Laxative))
                }
                (Effect::Athletic, Effect::Sedating) => {
                    reaction_list.push((Effect::Athletic, Effect::Munchies))
                }
                (Effect::Athletic, Effect::Spicy) => {
                    reaction_list.push((Effect::Athletic, Effect::Euphoric))
                }
                (Effect::Athletic, Effect::TropicThunder) => {
                    reaction_list.push((Effect::Athletic, Effect::Sneaky))
                }
                (Effect::Balding, Effect::CalorieDense) => {
                    reaction_list.push((Effect::Balding, Effect::Sneaky))
                }
                (Effect::Calming, Effect::Balding) => {
                    reaction_list.push((Effect::Calming, Effect::AntiGravity))
                }
                (Effect::Calming, Effect::Foggy) => {
                    reaction_list.push((Effect::Calming, Effect::Glowing))
                }
                (Effect::Calming, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Calming, Effect::Sneaky))
                }
                (Effect::Calming, Effect::Jennerising) => {
                    reaction_list.push((Effect::Calming, Effect::Balding))
                }
                (Effect::Calming, Effect::Sedating) => {
                    reaction_list.push((Effect::Calming, Effect::BrightEyed))
                }
                (Effect::Calming, Effect::Sneaky) => {
                    reaction_list.push((Effect::Calming, Effect::Slippery))
                }
                (Effect::CalorieDense, Effect::Balding) => {
                    reaction_list.push((Effect::CalorieDense, Effect::Sneaky))
                }
                (Effect::CalorieDense, Effect::CalorieDense) => {
                    reaction_list.push((Effect::CalorieDense, Effect::Explosive))
                }
                (Effect::CalorieDense, Effect::Jennerising) => {
                    reaction_list.push((Effect::CalorieDense, Effect::Gingeritis))
                }
                (Effect::Cyclopean, Effect::BrightEyed) => {
                    reaction_list.push((Effect::Cyclopean, Effect::Glowing))
                }
                (Effect::Cyclopean, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Cyclopean, Effect::ThoughtProvoking))
                }
                (Effect::Cyclopean, Effect::Sedating) => {
                    reaction_list.push((Effect::Cyclopean, Effect::Foggy))
                }
                (Effect::Disorienting, Effect::Athletic) => {
                    reaction_list.push((Effect::Disorienting, Effect::Electrifying))
                }
                (Effect::Disorienting, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Disorienting, Effect::Focused))
                }
                (Effect::Disorienting, Effect::Toxic) => {
                    reaction_list.push((Effect::Disorienting, Effect::Glowing))
                }
                (Effect::Disorienting, Effect::TropicThunder) => {
                    reaction_list.push((Effect::Disorienting, Effect::Toxic))
                }
                (Effect::Electrifying, Effect::BrightEyed) => {
                    if !self.effects.contains(&Effect::Zombifying) {
                        reaction_list.push((Effect::Electrifying, Effect::Euphoric))
                    }
                }
                (Effect::Electrifying, Effect::Sedating) => {
                    reaction_list.push((Effect::Electrifying, Effect::Refreshing))
                }
                (Effect::Electrifying, Effect::Sneaky) => {
                    reaction_list.push((Effect::Electrifying, Effect::Athletic))
                }
                (Effect::Electrifying, Effect::Toxic) => {
                    reaction_list.push((Effect::Electrifying, Effect::Disorienting))
                }
                (Effect::Energizing, Effect::Foggy) => {
                    if !self.effects.contains(&Effect::ThoughtProvoking) {
                        reaction_list.push((Effect::Energizing, Effect::Cyclopean))
                    }
                }
                (Effect::Energizing, Effect::Gingeritis) => {
                    if !self.effects.contains(&Effect::Cyclopean) {
                        reaction_list.push((Effect::Energizing, Effect::ThoughtProvoking))
                    }
                }
                (Effect::Energizing, Effect::Slippery) => {
                    reaction_list.push((Effect::Energizing, Effect::Munchies))
                }
                (Effect::Energizing, Effect::Sneaky) => {
                    if !self.effects.contains(&Effect::Munchies) {
                        reaction_list.push((Effect::Energizing, Effect::Paranoia));
                    }
                    // if !self.effects.contains(&Effect::Paranoia) {
                    //     reaction_list.push((Effect::Energizing, Effect::Balding));
                    // }
                }
                (Effect::Energizing, Effect::Toxic) => {
                    reaction_list.push((Effect::Energizing, Effect::Euphoric))
                }
                (Effect::Euphoric, Effect::Athletic) => {
                    reaction_list.push((Effect::Euphoric, Effect::Energizing))
                }
                (Effect::Euphoric, Effect::BrightEyed) => {
                    if !self.effects.contains(&Effect::Electrifying) {
                        reaction_list.push((Effect::Euphoric, Effect::Zombifying))
                    }
                }
                (Effect::Euphoric, Effect::Energizing) => {
                    reaction_list.push((Effect::Euphoric, Effect::Laxative))
                }
                (Effect::Euphoric, Effect::Jennerising) => {
                    reaction_list.push((Effect::Euphoric, Effect::SeizureInducing))
                }
                (Effect::Euphoric, Effect::Sedating) => {
                    reaction_list.push((Effect::Euphoric, Effect::Toxic))
                }
                (Effect::Euphoric, Effect::Slippery) => {
                    reaction_list.push((Effect::Euphoric, Effect::Sedating))
                }
                (Effect::Euphoric, Effect::Toxic) => {
                    if !self.effects.contains(&Effect::Energizing) {
                        reaction_list.push((Effect::Euphoric, Effect::Spicy))
                    }
                }
                (Effect::Euphoric, Effect::TropicThunder) => {
                    reaction_list.push((Effect::Euphoric, Effect::BrightEyed))
                }
                (Effect::Explosive, Effect::Balding) => {
                    reaction_list.push((Effect::Explosive, Effect::Sedating))
                }
                (Effect::Explosive, Effect::ThoughtProvoking) => {
                    reaction_list.push((Effect::Explosive, Effect::Euphoric))
                }
                (Effect::Focused, Effect::Athletic) => {
                    reaction_list.push((Effect::Focused, Effect::Shrinking))
                }
                (Effect::Focused, Effect::Balding) => {
                    reaction_list.push((Effect::Focused, Effect::Jennerising))
                }
                (Effect::Focused, Effect::CalorieDense) => {
                    reaction_list.push((Effect::Focused, Effect::Euphoric))
                }
                (Effect::Focused, Effect::Foggy) => {
                    reaction_list.push((Effect::Focused, Effect::Disorienting))
                }
                (Effect::Focused, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Focused, Effect::SeizureInducing))
                }
                (Effect::Focused, Effect::Sedating) => {
                    reaction_list.push((Effect::Focused, Effect::Calming))
                }
                (Effect::Focused, Effect::Sneaky) => {
                    reaction_list.push((Effect::Focused, Effect::Gingeritis))
                }
                (Effect::Foggy, Effect::Athletic) => {
                    reaction_list.push((Effect::Foggy, Effect::Laxative))
                }
                (Effect::Foggy, Effect::Energizing) => {
                    reaction_list.push((Effect::Foggy, Effect::Cyclopean))
                }
                (Effect::Foggy, Effect::Jennerising) => {
                    reaction_list.push((Effect::Foggy, Effect::Paranoia))
                }
                (Effect::Foggy, Effect::Slippery) => {
                    reaction_list.push((Effect::Foggy, Effect::Toxic))
                }
                (Effect::Foggy, Effect::Sneaky) => {
                    reaction_list.push((Effect::Foggy, Effect::Calming))
                }
                (Effect::Foggy, Effect::ThoughtProvoking) => {
                    reaction_list.push((Effect::Foggy, Effect::Energizing))
                }
                (Effect::Gingeritis, Effect::Energizing) => {
                    reaction_list.push((Effect::Gingeritis, Effect::ThoughtProvoking))
                }
                (Effect::Gingeritis, Effect::LongFaced) => {
                    reaction_list.push((Effect::Gingeritis, Effect::Refreshing))
                }
                (Effect::Gingeritis, Effect::Toxic) => {
                    reaction_list.push((Effect::Gingeritis, Effect::Smelly))
                }
                (Effect::Glowing, Effect::Athletic) => {
                    reaction_list.push((Effect::Glowing, Effect::Disorienting))
                }
                (Effect::Glowing, Effect::Sneaky) => {
                    reaction_list.push((Effect::Glowing, Effect::Toxic))
                }
                (Effect::Glowing, Effect::ThoughtProvoking) => {
                    reaction_list.push((Effect::Glowing, Effect::Refreshing))
                }
                (Effect::Jennerising, Effect::CalorieDense) => {
                    reaction_list.push((Effect::Jennerising, Effect::Gingeritis))
                }
                (Effect::Jennerising, Effect::Foggy) => {
                    reaction_list.push((Effect::Jennerising, Effect::Paranoia))
                }
                (Effect::Jennerising, Effect::Toxic) => {
                    reaction_list.push((Effect::Jennerising, Effect::Sneaky))
                }
                (Effect::Laxative, Effect::BrightEyed) => {
                    reaction_list.push((Effect::Laxative, Effect::CalorieDense))
                }
                (Effect::Laxative, Effect::Sedating) => {
                    reaction_list.push((Effect::Laxative, Effect::Euphoric))
                }
                (Effect::Laxative, Effect::Spicy) => {
                    reaction_list.push((Effect::Laxative, Effect::LongFaced))
                }
                (Effect::Laxative, Effect::Toxic) => {
                    reaction_list.push((Effect::Laxative, Effect::Foggy))
                }
                (Effect::Laxative, Effect::TropicThunder) => {
                    reaction_list.push((Effect::Laxative, Effect::Calming))
                }
                (Effect::LongFaced, Effect::Gingeritis) => {
                    reaction_list.push((Effect::LongFaced, Effect::Refreshing))
                }
                (Effect::LongFaced, Effect::ThoughtProvoking) => {
                    reaction_list.push((Effect::LongFaced, Effect::Electrifying))
                }
                (Effect::Munchies, Effect::BrightEyed) => {
                    reaction_list.push((Effect::Munchies, Effect::TropicThunder))
                }
                (Effect::Munchies, Effect::Energizing) => {
                    reaction_list.push((Effect::Munchies, Effect::Athletic))
                }
                (Effect::Munchies, Effect::Sedating) => {
                    reaction_list.push((Effect::Munchies, Effect::Slippery))
                }
                (Effect::Munchies, Effect::Slippery) => {
                    if !self.effects.contains(&Effect::Energizing) {
                        reaction_list.push((Effect::Munchies, Effect::Schizophrenic))
                    }
                }
                (Effect::Munchies, Effect::Sneaky) => {
                    reaction_list.push((Effect::Munchies, Effect::AntiGravity))
                }
                (Effect::Munchies, Effect::Spicy) => {
                    reaction_list.push((Effect::Munchies, Effect::Toxic))
                }
                (Effect::Munchies, Effect::Toxic) => {
                    reaction_list.push((Effect::Munchies, Effect::Sedating))
                }
                (Effect::Paranoia, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Paranoia, Effect::Jennerising))
                }
                (Effect::Paranoia, Effect::Slippery) => {
                    reaction_list.push((Effect::Paranoia, Effect::AntiGravity))
                }
                (Effect::Paranoia, Effect::Sneaky) => {
                    reaction_list.push((Effect::Paranoia, Effect::Balding))
                }
                (Effect::Paranoia, Effect::Toxic) => {
                    reaction_list.push((Effect::Paranoia, Effect::Calming))
                }
                (Effect::Refreshing, Effect::Jennerising) => {
                    reaction_list.push((Effect::Refreshing, Effect::ThoughtProvoking))
                }
                (Effect::Schizophrenic, Effect::Athletic) => {
                    reaction_list.push((Effect::Schizophrenic, Effect::Balding))
                }
                (Effect::Sedating, Effect::Athletic) => {
                    reaction_list.push((Effect::Sedating, Effect::Munchies))
                }
                (Effect::Sedating, Effect::ThoughtProvoking) => {
                    reaction_list.push((Effect::Sedating, Effect::Gingeritis))
                }
                (Effect::SeizureInducing, Effect::Foggy) => {
                    reaction_list.push((Effect::SeizureInducing, Effect::Focused))
                }
                (Effect::Shrinking, Effect::BrightEyed) => {
                    reaction_list.push((Effect::Shrinking, Effect::Munchies))
                }
                (Effect::Shrinking, Effect::CalorieDense) => {
                    reaction_list.push((Effect::Shrinking, Effect::Energizing))
                }
                (Effect::Shrinking, Effect::Foggy) => {
                    reaction_list.push((Effect::Shrinking, Effect::Electrifying))
                }
                (Effect::Shrinking, Effect::Sedating) => {
                    reaction_list.push((Effect::Shrinking, Effect::Paranoia))
                }
                (Effect::Shrinking, Effect::Spicy) => {
                    reaction_list.push((Effect::Shrinking, Effect::Refreshing))
                }
                (Effect::Shrinking, Effect::Toxic) => {
                    reaction_list.push((Effect::Shrinking, Effect::Focused))
                }
                (Effect::Slippery, Effect::Energizing) => {
                    reaction_list.push((Effect::Slippery, Effect::Munchies));
                    if self.effects.contains(&Effect::Munchies) {
                        reaction_list.push((Effect::Slippery, Effect::Athletic));
                    }
                }
                (Effect::Slippery, Effect::Foggy) => {
                    reaction_list.push((Effect::Slippery, Effect::Toxic))
                }
                (Effect::Smelly, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Smelly, Effect::AntiGravity))
                }
                (Effect::Sneaky, Effect::Energizing) => {
                    reaction_list.push((Effect::Sneaky, Effect::Paranoia))
                }
                (Effect::Sneaky, Effect::Foggy) => {
                    reaction_list.push((Effect::Sneaky, Effect::Calming))
                }
                (Effect::Sneaky, Effect::Spicy) => {
                    reaction_list.push((Effect::Sneaky, Effect::BrightEyed))
                }
                (Effect::Sneaky, Effect::Toxic) => {
                    reaction_list.push((Effect::Sneaky, Effect::TropicThunder))
                }
                (Effect::Spicy, Effect::Athletic) => {
                    reaction_list.push((Effect::Spicy, Effect::Euphoric))
                }
                (Effect::Spicy, Effect::Sneaky) => {
                    reaction_list.push((Effect::Spicy, Effect::BrightEyed))
                }
                (Effect::ThoughtProvoking, Effect::Foggy) => {
                    reaction_list.push((Effect::ThoughtProvoking, Effect::Cyclopean))
                }
                (Effect::ThoughtProvoking, Effect::LongFaced) => {
                    reaction_list.push((Effect::ThoughtProvoking, Effect::Electrifying))
                }
                (Effect::ThoughtProvoking, Effect::Sedating) => {
                    reaction_list.push((Effect::ThoughtProvoking, Effect::Gingeritis))
                }
                (Effect::Toxic, Effect::Energizing) => {
                    reaction_list.push((Effect::Toxic, Effect::Euphoric))
                }
                (Effect::Toxic, Effect::Gingeritis) => {
                    reaction_list.push((Effect::Toxic, Effect::Smelly))
                }
                (Effect::Toxic, Effect::Jennerising) => {
                    reaction_list.push((Effect::Toxic, Effect::Sneaky))
                }
                (Effect::Toxic, Effect::Sneaky) => {
                    reaction_list.push((Effect::Toxic, Effect::TropicThunder))
                }
                (Effect::TropicThunder, Effect::Athletic) => {
                    reaction_list.push((Effect::TropicThunder, Effect::Sneaky))
                }
                _ => {}
            }
        }
        let mut effects = self.effects.clone();
        for (old, new) in reaction_list {
            if !effects.contains(&new) {
                effects.remove(&old);
                effects.insert(new);
            }
        }
        if !effects.contains(&new_effect) && effects.iter().len() < 8 {
            effects.insert(new_effect);
        }

        if effects == self.effects {
            // if there are no changes, return self
            return self.clone();
        }

        let mut ingredients = self.ingredients.clone();
        ingredients.push(ingredient);
        Sellable {
            base: self.base,
            ingredients,
            effects,
            name: format!("{} + {:?}", self.name, ingredient),
        }
    }

    pub fn with_name(&self, name: String) -> Self {
        let mut new = self.clone();
        new.name = name;
        new
    }

    pub fn unit_price(&self, state: MixState) -> f32 {
        self.base.price(state.clone()) / self.yield_amount(state.clone())
    }

    pub fn price(&self, state: MixState) -> f32 {
        let mut price = self.unit_price(state.clone());
        let soil_price = match state.soil_quality {
            Quality::Low => 10.,
            Quality::Medium => 30.,
            Quality::High => 60.,
        };
        let pgr_price = if state.ingredients.contains(&OneTimeIngredient::PGR) {
            30.
        } else {
            0.
        };
        let fertilizer_price = if state.ingredients.contains(&OneTimeIngredient::Fertilizer) {
            30.
        } else {
            0.
        };
        let speed_grow_price = if state.ingredients.contains(&OneTimeIngredient::SpeedGrow) {
            30.
        } else {
            0.
        };
        price += match (self.base,) {
            (Product::Marijuana(_) | Product::Cocaine,) => {
                (soil_price + pgr_price + fertilizer_price + speed_grow_price)
                    / self.yield_amount(state.clone())
            }
            _ => 0.,
        };
        for ingredient in &self.ingredients {
            price += ingredient.price();
        }
        price
    }

    pub fn addictiveness(&self) -> f32 {
        let mut total_addictiveness = self.base.addictiveness() + self.effects.iter().map(|i| i.addictiveness()).sum::<f32>();
        if let Product::Marijuana(_) = self.base {
            if self.ingredients.is_empty() {
                total_addictiveness -= self.base.addictiveness();
            }
        }
        (total_addictiveness * 100.).clamp(0., 100.).floor()
    }

    pub fn key(&self) -> String {
        let mut key = format!("{:?}", self.base);
        for ingredient in &self.ingredients {
            key.push_str(&format!("{:?}", ingredient));
        }

        key
    }

    pub fn yield_amount(&self, state: MixState) -> f32 {
        match (
            self.base,
            state.use_pot,
            state.ingredients.contains(&OneTimeIngredient::PGR),
        ) {
            (Product::Marijuana(_), false, false) => 8.,
            (Product::Marijuana(_), true, false) => 12.,
            (Product::Marijuana(_), false, true) => 12.,
            (Product::Marijuana(_), true, true) => 16.,
            (Product::Cocaine, false, false) => 6.,
            (Product::Cocaine, true, false) => 9.,
            (Product::Cocaine, false, true) => 11.,
            (Product::Cocaine, true, true) => 16.,
            (Product::Meth, _, _) => 10.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Quality {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    Marijuana(Effect),
    Meth,
    Cocaine,
}

impl Product {
    pub fn price(&self, state: MixState) -> f32 {
        match self {
            Product::Marijuana(Effect::Calming) => 30.,
            Product::Marijuana(Effect::Refreshing) => 35.,
            Product::Marijuana(Effect::Energizing) => 40.,
            Product::Marijuana(Effect::Sedating) => 45.,
            Product::Marijuana(effect) => {
                unreachable!("{effect:?} is not a valid starting effect for marijuana")
            }
            Product::Meth => match state.pseudo_quality {
                Quality::Low => 60.,
                Quality::Medium => 80.,
                Quality::High => 110.,
            },
            Product::Cocaine => 80.,
        }
    }

    pub fn sell_price(&self) -> f32 {
        match self {
            Product::Marijuana(_) => 35.,
            Product::Meth => 70.,
            Product::Cocaine => 150.,
        }
    }

    pub fn addictiveness(&self) -> f32 {
        match self {
            Product::Marijuana(_) => 0.05,
            Product::Meth => 0.6,
            Product::Cocaine => 0.4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ingredient {
    Cuke,
    Banana,
    Paracetamol,
    Donut,
    Viagra,
    MouthWash,
    FluMedicine,
    Gasoline,
    EnergyDrink,
    MotorOil,
    MegaBean,
    Chili,
    Battery,
    Iodine,
    Addy,
    HorseSemen,
}

impl Ingredient {
    pub fn price(&self) -> f32 {
        match self {
            Ingredient::Cuke => 2.,
            Ingredient::Banana => 2.,
            Ingredient::Paracetamol => 3.,
            Ingredient::Donut => 3.,
            Ingredient::Viagra => 4.,
            Ingredient::MouthWash => 4.,
            Ingredient::FluMedicine => 5.,
            Ingredient::Gasoline => 5.,
            Ingredient::EnergyDrink => 6.,
            Ingredient::MotorOil => 6.,
            Ingredient::MegaBean => 7.,
            Ingredient::Chili => 7.,
            Ingredient::Battery => 8.,
            Ingredient::Iodine => 8.,
            Ingredient::Addy => 9.,
            Ingredient::HorseSemen => 9.,
        }
    }
    pub fn effect(&self) -> Effect {
        match self {
            Ingredient::Cuke => Effect::Energizing,
            Ingredient::Banana => Effect::Gingeritis,
            Ingredient::Paracetamol => Effect::Sneaky,
            Ingredient::Donut => Effect::CalorieDense,
            Ingredient::Viagra => Effect::TropicThunder,
            Ingredient::MouthWash => Effect::Balding,
            Ingredient::FluMedicine => Effect::Sedating,
            Ingredient::Gasoline => Effect::Toxic,
            Ingredient::EnergyDrink => Effect::Athletic,
            Ingredient::MotorOil => Effect::Slippery,
            Ingredient::MegaBean => Effect::Foggy,
            Ingredient::Chili => Effect::Spicy,
            Ingredient::Battery => Effect::BrightEyed,
            Ingredient::Iodine => Effect::Jennerising,
            Ingredient::Addy => Effect::ThoughtProvoking,
            Ingredient::HorseSemen => Effect::LongFaced,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    AntiGravity,
    Athletic,
    Balding,
    BrightEyed,
    Calming,
    CalorieDense,
    Cyclopean,
    Disorienting,
    Electrifying,
    Energizing,
    Euphoric,
    Explosive,
    Focused,
    Foggy,
    Gingeritis,
    Glowing,
    Jennerising,
    Laxative,
    Lethal,
    LongFaced,
    Munchies,
    Paranoia,
    Refreshing,
    Schizophrenic,
    Sedating,
    SeizureInducing,
    Shrinking,
    Slippery,
    Smelly,
    Sneaky,
    Spicy,
    ThoughtProvoking,
    Toxic,
    TropicThunder,
    Zombifying,
}

impl Effect {
    pub fn multiplier(&self) -> f32 {
        match self {
            Effect::AntiGravity => 0.54,
            Effect::Athletic => 0.32,
            Effect::Balding => 0.3,
            Effect::BrightEyed => 0.4,
            Effect::Calming => 0.1,
            Effect::CalorieDense => 0.28,
            Effect::Cyclopean => 0.56,
            Effect::Disorienting => 0.,
            Effect::Electrifying => 0.5,
            Effect::Energizing => 0.22,
            Effect::Euphoric => 0.18,
            Effect::Explosive => 0.,
            Effect::Focused => 0.16,
            Effect::Foggy => 0.36,
            Effect::Gingeritis => 0.2,
            Effect::Glowing => 0.48,
            Effect::Jennerising => 0.42,
            Effect::Laxative => 0.,
            Effect::Lethal => 0.,
            Effect::LongFaced => 0.52,
            Effect::Munchies => 0.12,
            Effect::Paranoia => 0.,
            Effect::Refreshing => 0.14,
            Effect::Schizophrenic => 0.,
            Effect::Sedating => 0.26,
            Effect::SeizureInducing => 0.,
            Effect::Shrinking => 0.6,
            Effect::Slippery => 0.34,
            Effect::Smelly => 0.,
            Effect::Sneaky => 0.24,
            Effect::Spicy => 0.38,
            Effect::ThoughtProvoking => 0.44,
            Effect::Toxic => 0.,
            Effect::TropicThunder => 0.46,
            Effect::Zombifying => 0.58,
        }
    }

    pub fn addictiveness(&self) -> f32 {
        match self {
            Effect::AntiGravity => 0.611,
            Effect::Athletic => 0.607,
            Effect::Balding => 0.,
            Effect::BrightEyed => 0.2,
            Effect::Calming => 0.,
            Effect::CalorieDense => 0.1,
            Effect::Cyclopean => 0.1,
            Effect::Disorienting => 0.,
            Effect::Electrifying => 0.235,
            Effect::Energizing => 0.34,
            Effect::Euphoric => 0.235,
            Effect::Explosive => 0.,
            Effect::Focused => 0.104,
            Effect::Foggy => 0.1,
            Effect::Gingeritis => 0.,
            Effect::Glowing => 0.472,
            Effect::Jennerising => 0.343,
            Effect::Laxative => 0.1,
            Effect::Lethal => 0.,
            Effect::LongFaced => 0.607,
            Effect::Munchies => 0.096,
            Effect::Paranoia => 0.,
            Effect::Refreshing => 0.104,
            Effect::Schizophrenic => 0.,
            Effect::Sedating => 0.,
            Effect::SeizureInducing => 0.,
            Effect::Shrinking => 0.336,
            Effect::Slippery => 0.309,
            Effect::Smelly => 0.,
            Effect::Sneaky => 0.327,
            Effect::Spicy => 0.665,
            Effect::ThoughtProvoking => 0.37,
            Effect::Toxic => 0.,
            Effect::TropicThunder => 0.803,
            Effect::Zombifying => 0.598,
        }
    }

    pub fn magnitude(&self) -> f64 {
        match self {
            Effect::AntiGravity => 3.111784,
            Effect::Athletic => 2.304192,
            Effect::Balding => 2.993276,
            Effect::BrightEyed => 3.030264,
            Effect::Calming => 1.030194,
            Effect::CalorieDense => 1.598312,
            Effect::Cyclopean => 2.894996,
            Effect::Disorienting => 2.142825,
            Effect::Electrifying => 3.319427,
            Effect::Energizing => 2.214611,
            Effect::Euphoric => 1.07,
            Effect::Explosive => 3.524826,
            Effect::Focused => 1.041201,
            Effect::Foggy => 2.277828,
            Effect::Gingeritis => 2.085777,
            Effect::Glowing => 2.944164,
            Effect::Jennerising => 3.377129,
            Effect::Laxative => 2.574063,
            Effect::Lethal => 3.200562,
            Effect::LongFaced => 2.936818,
            Effect::Munchies => 1.030437,
            Effect::Paranoia => 1.571369,
            Effect::Refreshing => 1.605148,
            Effect::Schizophrenic => 3.53511,
            Effect::Sedating => 2.137756,
            Effect::SeizureInducing => 2.675257,
            Effect::Shrinking => 3.379305,
            Effect::Slippery => 2.630057,
            Effect::Smelly => 1.69331,
            Effect::Sneaky => 2.115136,
            Effect::Spicy => 2.650019,
            Effect::ThoughtProvoking => 3.039079,
            Effect::Toxic => 2.315211,
            Effect::TropicThunder => 3.20576,
            Effect::Zombifying => 3.182843,
        }
    }

    pub fn direction(&self) -> (f64, f64) {
        match self {
            Effect::AntiGravity => (0.31, -0.95),
            Effect::Athletic => (-0.97, -0.25),
            Effect::Balding => (-0.05, -1.),
            Effect::BrightEyed => (1., -0.01),
            Effect::Calming => (1., 0.02),
            Effect::CalorieDense => (0.69, 0.72),
            Effect::Cyclopean => (-0.52, 0.85),
            Effect::Disorienting => (-0.28, 0.96),
            Effect::Electrifying => (-0.92, 0.39),
            Effect::Energizing => (-0.97, 0.26),
            Effect::Euphoric => (0., 1.),
            Effect::Explosive => (0.68, 0.74),
            Effect::Focused => (-1., 0.05),
            Effect::Foggy => (0.22, 0.97),
            Effect::Gingeritis => (-0.28, -0.96),
            Effect::Glowing => (0.48, 0.88),
            Effect::Jennerising => (-0.43, -0.9),
            Effect::Laxative => (-0.8, 0.59),
            Effect::Lethal => (-1., 0.02),
            Effect::LongFaced => (-0.07, 1.),
            Effect::Munchies => (0.03, -1.),
            Effect::Paranoia => (-0.74, -0.67),
            Effect::Refreshing => (-0.7, 0.71),
            Effect::Schizophrenic => (0.64, -0.77),
            Effect::Sedating => (0.98, -0.19),
            Effect::SeizureInducing => (-0.62, -0.78),
            Effect::Shrinking => (-0.96, -0.26),
            Effect::Slippery => (0.78, -0.63),
            Effect::Smelly => (0.75, -0.66),
            Effect::Sneaky => (0.36, -0.93),
            Effect::Spicy => (0.75, 0.66),
            Effect::ThoughtProvoking => (-0.86, -0.51),
            Effect::Toxic => (0.95, 0.3),
            Effect::TropicThunder => (0.94, -0.35),
            Effect::Zombifying => (0.93, 0.37),
        }
    }

    pub fn color(&self) -> String {
        match self {
            Effect::AntiGravity => String::from("rgb(36, 91, 204)"),
            Effect::Athletic => String::from("rgb(117, 201, 255)"),
            Effect::Balding => String::from("rgb(200, 146, 50)"),
            Effect::BrightEyed => String::from("rgb(191, 248, 255)"),
            Effect::Calming => String::from("rgb(255, 209, 155)"),
            Effect::CalorieDense => String::from("rgb(255, 131, 246)"),
            Effect::Cyclopean => String::from("rgb(255, 193, 117)"),
            Effect::Disorienting => String::from("rgb(255, 117, 81)"),
            Effect::Electrifying => String::from("rgb(85, 201, 255)"),
            Effect::Energizing => String::from("rgb(154, 255, 109)"),
            Effect::Euphoric => String::from("rgb(255, 235, 117)"),
            Effect::Explosive => String::from("rgb(255, 75, 64)"),
            Effect::Focused => String::from("rgb(117, 242, 255)"),
            Effect::Foggy => String::from("rgb(176, 176, 176)"),
            Effect::Gingeritis => String::from("rgb(255, 136, 42)"),
            Effect::Glowing => String::from("rgb(133, 229, 89)"),
            Effect::Jennerising => String::from("rgb(255, 143, 250)"),
            Effect::Laxative => String::from("rgb(118, 60, 37)"),
            Effect::Lethal => String::from("rgb(159, 43, 35)"),
            Effect::LongFaced => String::from("rgb(255, 218, 97)"),
            Effect::Munchies => String::from("rgb(202, 109, 87)"),
            Effect::Paranoia => String::from("rgb(199, 103, 98)"),
            Effect::Refreshing => String::from("rgb(178, 255, 153)"),
            Effect::Schizophrenic => String::from("rgb(100, 90, 255)"),
            Effect::Sedating => String::from("rgb(107, 95, 216)"),
            Effect::SeizureInducing => String::from("rgb(255, 234, 0)"),
            Effect::Shrinking => String::from("rgb(182, 255, 219)"),
            Effect::Slippery => String::from("rgb(162, 223, 255)"),
            Effect::Smelly => String::from("rgb(124, 188, 49)"),
            Effect::Sneaky => String::from("rgb(123, 123, 123)"),
            Effect::Spicy => String::from("rgb(255, 107, 76)"),
            Effect::ThoughtProvoking => String::from("rgb(255, 160, 203)"),
            Effect::Toxic => String::from("rgb(95, 154, 49)"),
            Effect::TropicThunder => String::from("rgb(255, 159, 71)"),
            Effect::Zombifying => String::from("rgb(113, 171, 93)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_1() {
        let mut mix = Sellable::from_product(Product::Meth);
        assert!(mix.effects.is_empty());
        mix = mix.add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]));
        mix = mix.add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking])
        );
        mix = mix.add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking, Effect::Sneaky])
        );
        mix = mix.add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
                Effect::BrightEyed,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
                Effect::BrightEyed,
                Effect::Jennerising,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::BrightEyed,
                Effect::Jennerising,
                Effect::Euphoric,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::BrightEyed,
                Effect::Smelly,
                Effect::Spicy,
                Effect::Toxic,
                Effect::TropicThunder,
            ])
        );
    }

    #[test]
    fn test_og() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming));
        assert_eq!(mix.effects, HashSet::from([Effect::Calming]));
        assert_eq!(mix.sell_price(), 38.);
        assert_eq!(mix.addictiveness(), 0.);
    }
    #[test]
    fn test_sour() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing));
        assert_eq!(mix.effects, HashSet::from([Effect::Refreshing]));
        assert_eq!(mix.sell_price(), 40.);
        assert_eq!(mix.addictiveness(), 10.);
    }
    #[test]
    fn test_green() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing));
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]));
        assert_eq!(mix.sell_price(), 43.);
        assert_eq!(mix.addictiveness(), 34.);
    }
    #[test]
    fn test_purple() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating));
        assert_eq!(mix.effects, HashSet::from([Effect::Sedating]));
        assert_eq!(mix.sell_price(), 44.);
        assert_eq!(mix.addictiveness(), 0.);
    }
    #[test]
    fn test_meth() {
        let mix = Sellable::from_product(Product::Meth);
        assert_eq!(mix.effects, HashSet::new());
        assert_eq!(mix.sell_price(), 70.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_cocaine() {
        let mix = Sellable::from_product(Product::Cocaine);
        assert_eq!(mix.effects, HashSet::new());
        assert_eq!(mix.sell_price(), 150.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_og_addy() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::ThoughtProvoking]),
        );
        assert_eq!(mix.sell_price(), 54.);
        assert_eq!(mix.addictiveness(), 42.);
    }
    #[test]
    fn test_og_addy_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::ThoughtProvoking,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 70.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_addy_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Electrifying, Effect::LongFaced]),
        );
        assert_eq!(mix.sell_price(), 74.);
        assert_eq!(mix.addictiveness(), 89.);
    }
    #[test]
    fn test_sour_addy() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::ThoughtProvoking]),
        );
        assert_eq!(mix.sell_price(), 55.);
        assert_eq!(mix.addictiveness(), 52.);
    }
    #[test]
    fn test_green_addy() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::ThoughtProvoking]),
        );
        assert_eq!(mix.sell_price(), 58.);
        assert_eq!(mix.addictiveness(), 76.);
    }
    #[test]
    fn test_purple_addy() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 42.);
    }
    #[test]
    fn test_meth_addy() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Addy);
        assert_eq!(mix.effects, HashSet::from([Effect::ThoughtProvoking]));
        assert_eq!(mix.sell_price(), 101.);
        assert_eq!(mix.addictiveness(), 97.);
    }
    #[test]
    fn test_meth_addy_horsesemen() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Electrifying, Effect::LongFaced])
        );
        assert_eq!(mix.sell_price(), 141.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_addy() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Addy);
        assert_eq!(mix.effects, HashSet::from([Effect::ThoughtProvoking]));
        assert_eq!(mix.sell_price(), 216.);
        assert_eq!(mix.addictiveness(), 77.);
    }
    #[test]
    fn test_cocaine_addy_flumedicine() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Gingeritis])
        );
        assert_eq!(mix.sell_price(), 219.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_cocaine_addy_flumedicine_paracetamol() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::Sedating, Effect::Sneaky])
        );
        assert_eq!(mix.sell_price(), 255.);
        assert_eq!(mix.addictiveness(), 72.);
    }
    #[test]
    fn test_og_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::Sneaky]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 37.);
    }
    #[test]
    fn test_sour_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Gingeritis]),
        );
        assert_eq!(mix.sell_price(), 47.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_green_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 42.);
    }
    #[test]
    fn test_purple_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Gingeritis]),
        );
        assert_eq!(mix.sell_price(), 51.);
        assert_eq!(mix.addictiveness(), 5.);
    }
    #[test]
    fn test_purple_banana_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Gingeritis, Effect::CalorieDense]),
        );
        assert_eq!(mix.sell_price(), 61.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_purple_banana_donut_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Munchies,
                Effect::Gingeritis,
                Effect::CalorieDense,
                Effect::Athletic
            ]),
        );
        assert_eq!(mix.sell_price(), 67.);
        assert_eq!(mix.addictiveness(), 85.);
    }
    #[test]
    fn test_purple_banana_donut_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Sedating,
                Effect::CalorieDense,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 77.);
        assert_eq!(mix.addictiveness(), 95.);
    }
    #[test]
    fn test_meth_banana() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Banana);
        assert_eq!(mix.effects, HashSet::from([Effect::Gingeritis]));
        assert_eq!(mix.sell_price(), 84.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_cocaine_banana() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Banana);
        assert_eq!(mix.effects, HashSet::from([Effect::Gingeritis]));
        assert_eq!(mix.sell_price(), 180.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_og_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::BrightEyed]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_sour_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::BrightEyed]),
        );
        assert_eq!(mix.sell_price(), 54.);
        assert_eq!(mix.addictiveness(), 35.);
    }
    #[test]
    fn test_green_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::BrightEyed]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 59.);
    }
    #[test]
    fn test_purple_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed]),
        );
        assert_eq!(mix.sell_price(), 58.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_meth_battery() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Battery);
        assert_eq!(mix.effects, HashSet::from([Effect::BrightEyed]));
        assert_eq!(mix.sell_price(), 98.);
        assert_eq!(mix.addictiveness(), 80.);
    }
    #[test]
    fn test_cocaine_battery() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Battery);
        assert_eq!(mix.effects, HashSet::from([Effect::BrightEyed]));
        assert_eq!(mix.sell_price(), 210.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_og_chili() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Calming, Effect::Spicy]));
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 71.);
    }
    #[test]
    fn test_sour_chili() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Spicy]),
        );
        assert_eq!(mix.sell_price(), 53.);
        assert_eq!(mix.addictiveness(), 81.);
    }
    #[test]
    fn test_green_chili() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Spicy]),
        );
        assert_eq!(mix.sell_price(), 56.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_chili() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Spicy]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 71.);
    }
    #[test]
    fn test_meth_chili() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Spicy]));
        assert_eq!(mix.sell_price(), 97.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_chili() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Spicy]));
        assert_eq!(mix.sell_price(), 207.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_chili_addy() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Chili)
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Spicy, Effect::ThoughtProvoking])
        );
        assert_eq!(mix.sell_price(), 273.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_chili_addy_battery() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Chili)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Spicy, Effect::BrightEyed, Effect::ThoughtProvoking])
        );
        assert_eq!(mix.sell_price(), 333.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Energizing]),
        );
        assert_eq!(mix.sell_price(), 46.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_sour_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Energizing]),
        );
        assert_eq!(mix.sell_price(), 48.);
        assert_eq!(mix.addictiveness(), 49.);
    }
    #[test]
    fn test_purple_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Energizing]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_meth_cuke() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]));
        assert_eq!(mix.sell_price(), 85.);
        assert_eq!(mix.addictiveness(), 94.);
    }
    #[test]
    fn test_meth_cuke_motoroil() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Slippery, Effect::Munchies])
        );
        assert_eq!(mix.sell_price(), 102.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Slippery, Effect::Schizophrenic])
        );
        assert_eq!(mix.sell_price(), 94.);
        assert_eq!(mix.addictiveness(), 90.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Munchies, Effect::Schizophrenic])
        );
        assert_eq!(mix.sell_price(), 94.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Energizing,
                Effect::Munchies,
                Effect::Slippery,
                Effect::Schizophrenic
            ])
        );
        assert_eq!(mix.sell_price(), 118.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Munchies,
                Effect::Toxic,
                Effect::Foggy,
                Effect::Schizophrenic,
                Effect::Cyclopean
            ])
        );
        assert_eq!(mix.sell_price(), 143.);
        assert_eq!(mix.addictiveness(), 89.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil_megabean_addy() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Energizing,
                Effect::Munchies,
                Effect::Toxic,
                Effect::ThoughtProvoking,
                Effect::Schizophrenic,
                Effect::Cyclopean,
            ])
        );
        assert_eq!(mix.sell_price(), 164.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil_megabean_addy_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Energizing,
                Effect::Munchies,
                Effect::Schizophrenic,
                Effect::Foggy,
                Effect::Toxic,
                Effect::Cyclopean,
                Effect::ThoughtProvoking,
            ])
        );
        assert_eq!(mix.sell_price(), 189.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil_megabean_addy_megabean_motoroil() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Munchies,
                Effect::Toxic,
                Effect::Energizing,
                Effect::Slippery,
                Effect::Foggy,
                Effect::Schizophrenic,
                Effect::ThoughtProvoking,
                Effect::Cyclopean,
            ])
        );
        assert_eq!(mix.sell_price(), 213.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_motoroil_motoroil_cuke_motoroil_megabean_addy_megabean_motoroil_paracetamol()
    {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Calming,
                Effect::Slippery,
                Effect::Schizophrenic,
                Effect::ThoughtProvoking,
                Effect::TropicThunder,
                Effect::AntiGravity,
                Effect::Cyclopean,
            ])
        );
        assert_eq!(mix.sell_price(), 241.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_viagra() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::TropicThunder])
        );
        assert_eq!(mix.sell_price(), 118.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_cuke_viagra_banana() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::TropicThunder
            ])
        );
        assert_eq!(mix.sell_price(), 147.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_cuke() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]));
        assert_eq!(mix.sell_price(), 183.);
        assert_eq!(mix.addictiveness(), 74.);
    }
    #[test]
    fn test_og_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::CalorieDense]),
        );
        assert_eq!(mix.sell_price(), 48.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_og_donut_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::CalorieDense, Effect::Explosive]),
        );
        assert_eq!(mix.sell_price(), 48.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_og_donut_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::CalorieDense, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 64.);
        assert_eq!(mix.addictiveness(), 95.);
    }
    #[test]
    fn test_og_donut_viagra_addy() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::CalorieDense,
                Effect::ThoughtProvoking,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 80.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_donut_viagra_addy_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::Addy)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::CalorieDense,
                Effect::Sneaky,
                Effect::Athletic,
                Effect::ThoughtProvoking
            ]),
        );
        assert_eq!(mix.sell_price(), 83.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_sour_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::CalorieDense]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_green_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::CalorieDense]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 49.);
    }
    #[test]
    fn test_purple_donut() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::CalorieDense]),
        );
        assert_eq!(mix.sell_price(), 54.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_purple_donut_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Sedating,
                Effect::CalorieDense,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 70.);
        assert_eq!(mix.addictiveness(), 95.);
    }
    #[test]
    fn test_purple_donut_viagra_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Donut)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Sedating,
                Effect::Sneaky,
                Effect::Balding,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 79.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_donut() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Donut);
        assert_eq!(mix.effects, HashSet::from([Effect::CalorieDense]));
        assert_eq!(mix.sell_price(), 90.);
        assert_eq!(mix.addictiveness(), 70.);
    }
    #[test]
    fn test_cocaine_donut() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Donut);
        assert_eq!(mix.effects, HashSet::from([Effect::CalorieDense]));
        assert_eq!(mix.sell_price(), 192.);
        assert_eq!(mix.addictiveness(), 50.);
    }
    #[test]
    fn test_og_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Athletic]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 65.);
    }
    #[test]
    fn test_sour_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Athletic]),
        );
        assert_eq!(mix.sell_price(), 51.);
        assert_eq!(mix.addictiveness(), 76.);
    }
    #[test]
    fn test_green_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Athletic]),
        );
        assert_eq!(mix.sell_price(), 54.);
        assert_eq!(mix.addictiveness(), 99.);
    }
    #[test]
    fn test_green_energydrink_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Athletic,
                Effect::ThoughtProvoking
            ]),
        );
        assert_eq!(mix.sell_price(), 69.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_energydrink() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Munchies, Effect::Athletic]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 75.);
    }
    #[test]
    fn test_meth_energydrink() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(mix.effects, HashSet::from([Effect::Athletic]));
        assert_eq!(mix.sell_price(), 92.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_energydrink() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(mix.effects, HashSet::from([Effect::Athletic]));
        assert_eq!(mix.sell_price(), 198.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_flumedicine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed]),
        );
        assert_eq!(mix.sell_price(), 58.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_sour_flumedicine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Sedating]),
        );
        assert_eq!(mix.sell_price(), 49.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_green_flumedicine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Sedating]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_green_flumedicine_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Sedating, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 68.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_green_flumedicine_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Balding, Effect::Sedating]),
        );
        assert_eq!(mix.sell_price(), 62.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_meth_flumedicine() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(mix.effects, HashSet::from([Effect::Sedating]));
        assert_eq!(mix.sell_price(), 88.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_meth_flumedicine_gasoline() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Toxic])
        );
        assert_eq!(mix.sell_price(), 88.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_meth_flumedicine_gasoline_viagra() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Toxic, Effect::TropicThunder])
        );
        assert_eq!(mix.sell_price(), 120.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_gasoline_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Toxic, Effect::Foggy])
        );
        assert_eq!(mix.sell_price(), 113.);
        assert_eq!(mix.addictiveness(), 70.);
    }
    #[test]
    fn test_meth_flumedicine_gasoline_megabean_battery() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Sedating,
                Effect::Toxic,
                Effect::Foggy,
                Effect::BrightEyed
            ])
        );
        assert_eq!(mix.sell_price(), 141.);
        assert_eq!(mix.addictiveness(), 90.);
    }
    #[test]
    fn test_meth_flumedicine_battery() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed])
        );
        assert_eq!(mix.sell_price(), 116.);
        assert_eq!(mix.addictiveness(), 80.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed, Effect::Foggy])
        );
        assert_eq!(mix.sell_price(), 141.);
        assert_eq!(mix.addictiveness(), 90.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Sedating,
                Effect::Foggy,
                Effect::BrightEyed,
                Effect::TropicThunder
            ])
        );
        assert_eq!(mix.sell_price(), 174.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Munchies,
                Effect::Sneaky,
                Effect::BrightEyed,
                Effect::Laxative,
                Effect::Athletic
            ])
        );
        assert_eq!(mix.sell_price(), 146.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Munchies,
                Effect::Laxative,
                Effect::Athletic,
                Effect::Foggy,
                Effect::BrightEyed,
            ])
        );
        assert_eq!(mix.sell_price(), 161.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink_megabean_battery() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::CalorieDense,
                Effect::Athletic,
                Effect::Foggy,
                Effect::BrightEyed,
                Effect::TropicThunder,
            ])
        );
        assert_eq!(mix.sell_price(), 204.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink_megabean_battery_viagra() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Foggy,
                Effect::BrightEyed,
                Effect::TropicThunder,
            ])
        );
        assert_eq!(mix.sell_price(), 199.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink_megabean_battery_viagra_energydrink(
    ) {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Laxative,
                Effect::Athletic,
                Effect::BrightEyed,
                Effect::TropicThunder,
            ])
        );
        assert_eq!(mix.sell_price(), 196.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_flumedicine_battery_megabean_viagra_energydrink_megabean_battery_viagra_motoroil()
    {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::FluMedicine)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery)
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Toxic,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Slippery,
                Effect::BrightEyed,
                Effect::TropicThunder,
            ])
        );
        assert_eq!(mix.sell_price(), 197.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_flumedicine() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(mix.effects, HashSet::from([Effect::Sedating]));
        assert_eq!(mix.sell_price(), 189.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_og_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Calming, Effect::Toxic]));
        assert_eq!(mix.sell_price(), 38.);
        assert_eq!(mix.addictiveness(), 5.);
    }
    #[test]
    fn test_sour_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Toxic]),
        );
        assert_eq!(mix.sell_price(), 40.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_green_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Euphoric, Effect::Toxic]),
        );
        assert_eq!(mix.sell_price(), 41.);
        assert_eq!(mix.addictiveness(), 28.);
    }
    #[test]
    fn test_purple_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Toxic]),
        );
        assert_eq!(mix.sell_price(), 44.);
        assert_eq!(mix.addictiveness(), 5.);
    }
    #[test]
    fn test_meth_gasoline() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Toxic]));
        assert_eq!(mix.sell_price(), 70.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_meth_gasoline_flumedicine() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Toxic, Effect::Sedating])
        );
        assert_eq!(mix.sell_price(), 88.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_meth_gasoline_energydrink() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Toxic, Effect::Athletic])
        );
        assert_eq!(mix.sell_price(), 92.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_gasoline_energydrink_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Toxic, Effect::Laxative, Effect::Foggy])
        );
        assert_eq!(mix.sell_price(), 95.);
        assert_eq!(mix.addictiveness(), 80.);
    }
    #[test]
    fn test_cocaine_gasoline() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Toxic]));
        assert_eq!(mix.sell_price(), 150.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_cocaine_gasoline_cuke() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Euphoric, Effect::Energizing])
        );
        assert_eq!(mix.sell_price(), 210.);
        assert_eq!(mix.addictiveness(), 97.);
    }
    #[test]
    fn test_cocaine_gasoline_cuke_battery() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::BrightEyed, Effect::Zombifying])
        );
        assert_eq!(mix.sell_price(), 330.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_gasoline_energydrink() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Toxic, Effect::Athletic])
        );
        assert_eq!(mix.sell_price(), 198.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_gasoline_energydrink_megabean() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Toxic, Effect::Laxative, Effect::Foggy])
        );
        assert_eq!(mix.sell_price(), 204.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_og_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::LongFaced]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 65.);
    }
    #[test]
    fn test_sour_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::LongFaced]),
        );
        assert_eq!(mix.sell_price(), 58.);
        assert_eq!(mix.addictiveness(), 76.);
    }
    #[test]
    fn test_green_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::LongFaced]),
        );
        assert_eq!(mix.sell_price(), 61.);
        assert_eq!(mix.addictiveness(), 99.);
    }
    #[test]
    fn test_purple_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::LongFaced]),
        );
        assert_eq!(mix.sell_price(), 62.);
        assert_eq!(mix.addictiveness(), 65.);
    }
    #[test]
    fn test_meth_horsesemen() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(mix.effects, HashSet::from([Effect::LongFaced]));
        assert_eq!(mix.sell_price(), 106.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_horsesemen() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(mix.effects, HashSet::from([Effect::LongFaced]));
        assert_eq!(mix.sell_price(), 228.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::Jennerising]),
        );
        assert_eq!(mix.sell_price(), 60.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_sour_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::ThoughtProvoking, Effect::Jennerising]),
        );
        assert_eq!(mix.sell_price(), 65.);
        assert_eq!(mix.addictiveness(), 76.);
    }
    #[test]
    fn test_green_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Jennerising]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 73.);
    }
    #[test]
    fn test_purple_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Jennerising]),
        );
        assert_eq!(mix.sell_price(), 59.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_meth_iodine() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Iodine);
        assert_eq!(mix.effects, HashSet::from([Effect::Jennerising]));
        assert_eq!(mix.sell_price(), 99.);
        assert_eq!(mix.addictiveness(), 94.);
    }
    #[test]
    fn test_cocaine_iodine() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Iodine);
        assert_eq!(mix.effects, HashSet::from([Effect::Jennerising]));
        assert_eq!(mix.sell_price(), 213.);
        assert_eq!(mix.addictiveness(), 74.);
    }
    #[test]
    fn test_cocaine_iodine_motoroil() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Iodine)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Jennerising, Effect::Slippery])
        );
        assert_eq!(mix.sell_price(), 264.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_iodine_motoroil_cuke() {
        let mix = Sellable::from_product(Product::Cocaine)
            .add_ingredient(Ingredient::Iodine)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Jennerising, Effect::Energizing, Effect::Munchies])
        );
        assert_eq!(mix.sell_price(), 264.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy, Effect::Glowing]));
        assert_eq!(mix.sell_price(), 64.);
        assert_eq!(mix.addictiveness(), 62.);
    }
    #[test]
    fn test_sour_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Foggy]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_green_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Foggy, Effect::Cyclopean]),
        );
        assert_eq!(mix.sell_price(), 67.);
        assert_eq!(mix.addictiveness(), 25.);
    }
    #[test]
    fn test_green_megabean_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Foggy, Effect::TropicThunder, Effect::Cyclopean]),
        );
        assert_eq!(mix.sell_price(), 83.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Foggy]),
        );
        assert_eq!(mix.sell_price(), 57.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_meth_megabean() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy]));
        assert_eq!(mix.sell_price(), 95.);
        assert_eq!(mix.addictiveness(), 70.);
    }
    #[test]
    fn test_cocaine_megabean() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy]));
        assert_eq!(mix.sell_price(), 204.);
        assert_eq!(mix.addictiveness(), 50.);
    }
    #[test]
    fn test_og_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 35.);
    }
    #[test]
    fn test_og_motoroil_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::Sneaky, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 62.);
        assert_eq!(mix.addictiveness(), 68.);
    }
    #[test]
    fn test_og_motoroil_banana_chili() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Slippery,
                Effect::Spicy,
                Effect::BrightEyed
            ]),
        );
        assert_eq!(mix.sell_price(), 81.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_motoroil_banana_chili_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Chili)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Slippery,
                Effect::Balding,
                Effect::Spicy,
                Effect::BrightEyed
            ]),
        );
        assert_eq!(mix.sell_price(), 92.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_motoroil_banana_chili_mouthwash_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Chili)
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Balding,
                Effect::Slippery,
                Effect::Spicy,
                Effect::BrightEyed,
                Effect::Jennerising,
            ]),
        );
        assert_eq!(mix.sell_price(), 106.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_motoroil_banana_chili_mouthwash_iodine_flumedicine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::Chili)
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Iodine)
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::Balding,
                Effect::Slippery,
                Effect::Spicy,
                Effect::Jennerising,
                Effect::BrightEyed,
                Effect::Sedating,
            ]),
        );
        assert_eq!(mix.sell_price(), 116.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_sour_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 46.);
    }
    #[test]
    fn test_green_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Munchies, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 51.);
        assert_eq!(mix.addictiveness(), 45.);
    }
    #[test]
    fn test_green_motoroil_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Schizophrenic, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 47.);
        assert_eq!(mix.addictiveness(), 35.);
    }
    #[test]
    fn test_green_motoroil_motoroil_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Munchies, Effect::Energizing, Effect::Schizophrenic]),
        );
        assert_eq!(mix.sell_price(), 47.);
        assert_eq!(mix.addictiveness(), 48.);
    }
    #[test]
    fn test_green_motoroil_motoroil_cuke_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::MotorOil)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Sneaky,
                Effect::Schizophrenic,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 62.);
        assert_eq!(mix.addictiveness(), 98.);
    }
    #[test]
    fn test_purple_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 56.);
        assert_eq!(mix.addictiveness(), 35.);
    }
    #[test]
    fn test_meth_motoroil() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::MotorOil);
        assert_eq!(mix.effects, HashSet::from([Effect::Slippery]));
        assert_eq!(mix.sell_price(), 94.);
        assert_eq!(mix.addictiveness(), 90.);
    }
    #[test]
    fn test_cocaine_motoroil() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MotorOil);
        assert_eq!(mix.effects, HashSet::from([Effect::Slippery]));
        assert_eq!(mix.sell_price(), 201.);
        assert_eq!(mix.addictiveness(), 70.);
    }
    #[test]
    fn test_og_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::AntiGravity]),
        );
        assert_eq!(mix.sell_price(), 64.);
        assert_eq!(mix.addictiveness(), 66.);
    }
    #[test]
    fn test_og_mouthwash_banana() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::AntiGravity, Effect::Gingeritis]),
        );
        assert_eq!(mix.sell_price(), 71.);
        assert_eq!(mix.addictiveness(), 66.);
    }
    #[test]
    fn test_og_mouthwash_banana_horsesemen() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Refreshing,
                Effect::Balding,
                Effect::LongFaced
            ]),
        );
        assert_eq!(mix.sell_price(), 72.);
        assert_eq!(mix.addictiveness(), 76.);
    }
    #[test]
    fn test_og_mouthwash_banana_horsesemen_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::HorseSemen)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Refreshing,
                Effect::Balding,
                Effect::LongFaced,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 88.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_mouthwash_banana_horsesemen_mouthwash_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::Banana)
            .add_ingredient(Ingredient::HorseSemen)
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Refreshing,
                Effect::Balding,
                Effect::Slippery,
                Effect::LongFaced,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 99.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_sour_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::Refreshing]),
        );
        assert_eq!(mix.sell_price(), 50.);
        assert_eq!(mix.addictiveness(), 15.);
    }
    #[test]
    fn test_green_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Balding]),
        );
        assert_eq!(mix.sell_price(), 53.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_green_mouthwash_flumedicine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Sedating, Effect::Balding]),
        );
        assert_eq!(mix.sell_price(), 62.);
        assert_eq!(mix.addictiveness(), 39.);
    }
    #[test]
    fn test_purple_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Balding]),
        );
        assert_eq!(mix.sell_price(), 55.);
        assert_eq!(mix.addictiveness(), 5.);
    }
    #[test]
    fn test_meth_mouthwash() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::MouthWash);
        assert_eq!(mix.effects, HashSet::from([Effect::Balding]));
        assert_eq!(mix.sell_price(), 91.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_meth_mouthwash_flumedicine() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::MouthWash)
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::Sedating])
        );
        assert_eq!(mix.sell_price(), 109.);
        assert_eq!(mix.addictiveness(), 60.);
    }
    #[test]
    fn test_cocaine_mouthwash() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MouthWash);
        assert_eq!(mix.effects, HashSet::from([Effect::Balding]));
        assert_eq!(mix.sell_price(), 195.);
        assert_eq!(mix.addictiveness(), 40.);
    }
    #[test]
    fn test_og_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sneaky, Effect::Slippery]),
        );
        assert_eq!(mix.sell_price(), 55.);
        assert_eq!(mix.addictiveness(), 68.);
    }
    #[test]
    fn test_og_paracetamol_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Paranoia, Effect::Munchies, Effect::Energizing]),
        );
        assert_eq!(mix.sell_price(), 47.);
        assert_eq!(mix.addictiveness(), 48.);
    }
    #[test]
    fn test_og_paracetamol_cuke_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Euphoric,
                Effect::Toxic,
                Effect::Sedating
            ]),
        );
        assert_eq!(mix.sell_price(), 54.);
        assert_eq!(mix.addictiveness(), 28.);
    }
    #[test]
    fn test_og_paracetamol_cuke_gasoline_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Euphoric,
                Effect::Sneaky,
                Effect::Sedating,
                Effect::Slippery,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 87.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_gasoline_paracetamol_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Munchies,
                Effect::Energizing,
                Effect::Sedating,
                Effect::Laxative,
                Effect::TropicThunder,
            ]),
        );
        assert_eq!(mix.sell_price(), 72.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_gasoline_paracetamol_cuke_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Munchies,
                Effect::Sedating,
                Effect::Laxative,
                Effect::Foggy,
                Effect::TropicThunder,
                Effect::Cyclopean,
            ]),
        );
        assert_eq!(mix.sell_price(), 97.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_gasoline_paracetamol_cuke_megabean_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Munchies,
                Effect::Sedating,
                Effect::CalorieDense,
                Effect::Foggy,
                Effect::Glowing,
                Effect::BrightEyed,
                Effect::TropicThunder
            ]),
        );
        assert_eq!(mix.sell_price(), 118.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Paranoia,
                Effect::Sneaky,
                Effect::Balding,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 73.);
        assert_eq!(mix.addictiveness(), 98.);
    }
    #[test]
    fn test_og_paracetamol_cuke_paracetamol_gasoline() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Toxic,
                Effect::Balding,
                Effect::TropicThunder,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 84.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_paracetamol_gasoline_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Calming,
                Effect::Euphoric,
                Effect::Energizing,
                Effect::Balding,
                Effect::TropicThunder,
                Effect::AntiGravity,
            ]),
        );
        assert_eq!(mix.sell_price(), 98.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_paracetamol_gasoline_cuke_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Euphoric,
                Effect::Balding,
                Effect::Foggy,
                Effect::TropicThunder,
                Effect::Glowing,
                Effect::AntiGravity,
                Effect::Cyclopean,
            ]),
        );
        assert_eq!(mix.sell_price(), 136.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_og_paracetamol_cuke_paracetamol_gasoline_cuke_megabean_battery() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Gasoline)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MegaBean)
            .add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Balding,
                Effect::Foggy,
                Effect::BrightEyed,
                Effect::TropicThunder,
                Effect::Glowing,
                Effect::AntiGravity,
                Effect::Cyclopean,
                Effect::Zombifying,
            ]),
        );
        assert_eq!(mix.sell_price(), 164.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_sour_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Sneaky]),
        );
        assert_eq!(mix.sell_price(), 48.);
        assert_eq!(mix.addictiveness(), 48.);
    }
    #[test]
    fn test_green_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Paranoia, Effect::Sneaky]),
        );
        assert_eq!(mix.sell_price(), 43.);
        assert_eq!(mix.addictiveness(), 37.);
    }
    #[test]
    fn test_green_paracetamol_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Slippery, Effect::Sneaky, Effect::AntiGravity]),
        );
        assert_eq!(mix.sell_price(), 74.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_green_paracetamol_cuke() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Paranoia, Effect::Energizing, Effect::Sneaky]),
        );
        assert_eq!(mix.sell_price(), 51.);
        assert_eq!(mix.addictiveness(), 71.);
    }
    #[test]
    fn test_green_paracetamol_cuke_motoroil() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::Cuke)
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Munchies,
                Effect::Sneaky,
                Effect::Slippery,
                Effect::AntiGravity
            ]),
        );
        assert_eq!(mix.sell_price(), 78.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_paracetamol() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Sneaky]),
        );
        assert_eq!(mix.sell_price(), 52.);
        assert_eq!(mix.addictiveness(), 37.);
    }
    #[test]
    fn test_purple_paracetamol_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Sneaky, Effect::Balding]),
        );
        assert_eq!(mix.sell_price(), 63.);
        assert_eq!(mix.addictiveness(), 37.);
    }
    #[test]
    fn test_meth_paracetamol() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(mix.effects, HashSet::from([Effect::Sneaky]));
        assert_eq!(mix.sell_price(), 87.);
        assert_eq!(mix.addictiveness(), 92.);
    }
    #[test]
    fn test_meth_paracetamol_energydrink() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sneaky, Effect::Athletic])
        );
        assert_eq!(mix.sell_price(), 109.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_meth_paracetamol_energydrink_megabean() {
        let mix = Sellable::from_product(Product::Meth)
            .add_ingredient(Ingredient::Paracetamol)
            .add_ingredient(Ingredient::EnergyDrink)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Laxative, Effect::Foggy])
        );
        assert_eq!(mix.sell_price(), 102.);
        assert_eq!(mix.addictiveness(), 80.);
    }
    #[test]
    fn test_cocaine_paracetamol() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(mix.effects, HashSet::from([Effect::Sneaky]));
        assert_eq!(mix.sell_price(), 186.);
        assert_eq!(mix.addictiveness(), 72.);
    }
    #[test]
    fn test_og_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 55.);
        assert_eq!(mix.addictiveness(), 85.);
    }
    #[test]
    fn test_og_viagra_mouthwash() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Calming))
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::TropicThunder, Effect::AntiGravity]),
        );
        assert_eq!(mix.sell_price(), 80.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_sour_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Refreshing))
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 56.);
        assert_eq!(mix.addictiveness(), 95.);
    }
    #[test]
    fn test_green_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Energizing))
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 59.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_viagra() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 60.);
        assert_eq!(mix.addictiveness(), 85.);
    }
    #[test]
    fn test_purple_viagra_iodine() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Jennerising, Effect::TropicThunder]),
        );
        assert_eq!(mix.sell_price(), 75.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_purple_viagra_iodine_megabean() {
        let mix = Sellable::from_product(Product::Marijuana(Effect::Sedating))
            .add_ingredient(Ingredient::Viagra)
            .add_ingredient(Ingredient::Iodine)
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Sedating,
                Effect::TropicThunder,
                Effect::Paranoia,
                Effect::Foggy
            ]),
        );
        assert_eq!(mix.sell_price(), 73.);
        assert_eq!(mix.addictiveness(), 95.);
    }
    #[test]
    fn test_meth_viagra() {
        let mix = Sellable::from_product(Product::Meth).add_ingredient(Ingredient::Viagra);
        assert_eq!(mix.effects, HashSet::from([Effect::TropicThunder]));
        assert_eq!(mix.sell_price(), 102.);
        assert_eq!(mix.addictiveness(), 100.);
    }
    #[test]
    fn test_cocaine_viagra() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Viagra);
        assert_eq!(mix.effects, HashSet::from([Effect::TropicThunder]));
        assert_eq!(mix.sell_price(), 219.);
        assert_eq!(mix.addictiveness(), 100.);
    }
}
