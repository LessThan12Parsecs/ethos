// TODO: Relationships
// TODO: Work dynamics
// TODO: Think of better overall architecture
// TODO: Define how location is used.
// TODO: values for social skills.


struct Motivations {
    arts : u8, // 0-100
    logic: u8, // 0-100
    commerce: u8, // 0-100
    philosophy: u8, // 0-100
}

struct Needs {
    sustenance: u8, // 0-100
    energy: u8, // 0-100, 0 -> passing out
    companionship: u8, // 0-100
    health: u8, //0-100, 0 -> dead. || Related to Age.

}


// TODO: Should these be the same as motivations?
struct Skills {
    arts : u8, // 0-100
    logic: u8, // 0-100
    commerce: u8, // 0-100
    philosophy: u8, // 0-100
}

struct Creds {
    fame: u8,
    formal_education: u8, // 0 - 100, 100 -> Top of your field

}

struct Mindset {
    anxiety: u8,  // 0-100
    negativity: u8, // 0-100

}

struct Externals {
    funds: u32, // 0-u32
    living_arrangement: u8, // 0-100, 0 -> Homeless and 100 completely satisfied.
}



struct Person {
    id: u32,
    age: u16, // 0-100
    motivations: Motivations,
    needs: Needs,
    mindset: Mindset,
    skills: Skills,
    externals: Externals,
    creds: Creds,
    pos_x: u16,
    pos_y: u16,
    // pos_z?

}


fn main() {
    println!("Hello, ETHOS!");
}
