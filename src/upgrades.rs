
#[derive(Default)]
pub struct Upgrades {
    pub mining: MiningUpgrade,
    pub ladder: LadderUpgrade,
    pub bag: BagUpgrade,
    pub climb_momentum: ClimbMomentumUpgrade,
    
    pub jetpack: JetpackUpgrade,
    pub jetpack_boost: JetpackBoostUpgrade,
    pub jetpack_fuel: JetpackFuelUpgrade,
    pub jetpack_storage: JetpackStorageUpgrade,
}

pub struct SeqUpgrade<'a> {
    pub name: &'static str,
    pub tier: u8,
    pub cost: i32,
    pub upgrade: Box<dyn FnMut() + 'a>,
    pub count: u8,
    pub unlocked: bool,
    pub reached_count: bool,
}

macro_rules! create_seq {
    {struct $container:ident; enum $kind:ident {
        $first_variant:ident => ($first_name:literal, $first_cost:literal),
        $($variant:ident => ($name:literal, $cost: literal),)*
    }} => {
        #[derive(Copy, Clone, Default)]
        pub struct $container {
            pub kind: $kind,
            pub derived_unlocked: bool,
        }
        
        #[repr(u8)]
        #[derive(Copy, Clone, Default)]
        pub enum $kind {
            #[default]
            $first_variant,
            $($variant,)*
        }

        impl $container {
            pub fn to_seq<'a>(&'a mut self) -> SeqUpgrade<'a> {
                let tier = self.kind as u8;
                let next_tier = $kind::from_u8_if_available(tier);
                let count = std::mem::variant_count::<$kind>() as u8;
                let name = next_tier.name();
                let cost = next_tier.cost();
        
                SeqUpgrade {
                    name,
                    tier,
                    cost,
                    upgrade: Box::new(|| self.kind.upgrade()),
                    count,
                    unlocked: self.derived_unlocked,
                    reached_count: tier == count-1,
                }
            }
            pub fn reached(&self, tier: $kind) -> bool {
                tier as u8 <= self.kind as u8
            }
        }

        impl $kind {
            fn from_u8_if_available(cur: u8) -> Self {
                unsafe {
                    // WARN: This line is inside unsafe because it affects the result. This ensures next
                    // never passes max possible value for variant
                    let next = u8::min(cur+1, std::mem::variant_count::<Self>() as u8-1);
                    std::mem::transmute(next)
                }
            }
            pub fn upgrade(&mut self) {
                *self = Self::from_u8_if_available(*self as u8);
            }
            
            pub fn name(&self) -> &'static str {
                match self {
                    Self::$first_variant => $first_name,
                    $(Self::$variant => $name,)*
                }
            }
            pub fn cost(&self) -> i32 {
                match self {
                    Self::$first_variant => $first_cost,
                    $(Self::$variant => $cost,)*
                }
            }
        }
    };
}

create_seq! {struct MiningUpgrade; enum MiningUpgradeKind {
    DefaultPickaxe => ("Default Pickaxe", 0),
    IronPickaxe => ("Iron Pickaxe", 100),
    HardenedPickaxe => ("Hardened Pickaxe", 500),
    AlloyPickaxe => ("Alloy Pickaxe", 2500),
}}
create_seq! {struct LadderUpgrade; enum LadderUpgradeKind {
    DefaultClimb => ("Default Climb", 0),
    FastClimb => ("Fast Climb", 500),
}}
create_seq! {struct BagUpgrade; enum BagUpgradeKind {
    DefaultBag => ("Default Bag", 0),
    SmallPouch => ("Small Pouch", 150),
    BiggerPouch => ("Bigger Pouch", 500),
    Backpack => ("Backpack", 1500),
    Sack => ("Sack", 3500),
}}
create_seq! {struct ClimbMomentumUpgrade; enum ClimbMomentumUpgradeKind {
    NoClimbMomentum => ("No Climb Momentum", 0),
    ClimbMomentum => ("Climb Momentum", 1500),
}}
create_seq! {struct JetpackUpgrade; enum JetpackUpgradeKind {
    NoJetpack => ("No Jetpack", 0),
    Jetpack => ("Jetpack", 5000),
}}
create_seq! {struct JetpackBoostUpgrade; enum JetpackBoostUpgradeKind {
    NoDwarfcopterBoost => ("No Boost", 0),
    SmallBoost => ("Small Boost", 6000),
}}
create_seq! {struct JetpackFuelUpgrade; enum JetpackFuelUpgradeKind {
    DefaultFuel => ("Default Fuel", 0),
    QuickTanks => ("Quick Tanks", 3500),
    DoubleTanks => ("Long Double Tanks", 6000),
    LongHaulTanks => ("Long Haul Tanks", 12000),
}}
create_seq! {struct JetpackStorageUpgrade; enum JetpackStorageUpgradeKind {
    DefaultStorage => ("Default Storage", 0),
    XLStorage => ("XL Storage", 4000),
    XXLStorage => ("XXL Storage", 7000),
    XXXLStorage => ("XXXL Storage", 13000),
}}
