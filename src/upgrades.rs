
#[derive(Default)]
pub struct Upgrades {
    pub mining: MiningUpgrade,
    pub ladder: LadderUpgrade,
    pub bag: BagUpgrade,
}

pub struct SeqUpgrade<'a> {
    pub name: &'static str,
    pub tier: u8,
    pub cost: i32,
    pub upgrade: Box<dyn FnMut() + 'a>,
    pub count: u8,
    pub reached_count: bool,
}

macro_rules! create_seq {
    {enum $type:ident {
        $first_variant:ident => ($first_name:literal, $first_cost:literal),
        $($variant:ident => ($name:literal, $cost: literal),)*
    }} => {
        #[repr(u8)]
        #[derive(Copy, Clone, Default)]
        pub enum $type {
            #[default]
            $first_variant,
            $($variant,)*
        }

        impl $type {
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
            pub fn to_seq<'a>(&'a mut self) -> SeqUpgrade<'a> {
                let tier = *self as u8;
                let next_tier = Self::from_u8_if_available(tier);
                let count = std::mem::variant_count::<Self>() as u8;
                let name = next_tier.name();
                let cost = next_tier.cost();
        
                SeqUpgrade {
                    name,
                    tier,
                    cost,
                    upgrade: Box::new(|| self.upgrade()),
                    count,
                    reached_count: tier == count-1,
                }
            }
        }
    };
}

create_seq! {enum MiningUpgrade {
    DefaultPickaxe => ("Default Pickaxe", 0),
    IronPickaxe => ("Iron Pickaxe", 100),
    HardenedPickaxe => ("Hardened Pickaxe", 500),
    AlloyPickaxe => ("Alloy Pickaxe", 1000),
}}
create_seq! {enum LadderUpgrade {
    DefaultClimb => ("Default Climb", 0),
    FastClimb => ("Fast Climb", 250),
}}
create_seq! {enum BagUpgrade {
    DefaultBag => ("Default Bag", 0),
    SmallPouch => ("Small Pouch", 150),
    BiggerPouch => ("Bigger Pouch", 250),
    Backpack => ("Backpack", 750),
    Sack => ("Sack", 1500),
}}
