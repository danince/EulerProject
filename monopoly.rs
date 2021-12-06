


use rand::Rng;

fn roll6()-> u32
{
    let mut rng = rand::thread_rng();
    let roll_1=rng.gen_range(1, 7);
    let roll_2=rng.gen_range(1, 7);
    roll_1+roll_2
}

const MONOPOLY_JAIL:u32 = 10;
const MONOPOLY_CC1:u32 = 2;
const MONOPOLY_CC2:u32 = 17;
const MONOPOLY_CC3:u32 = 33;
const MONOPOLY_CH1:u32 = 7;
const MONOPOLY_CH2:u32 = 22;
const MONOPOLY_CH3:u32 = 36;
const MONOPOLY_G2J:u32 = 30;
const MONOPOLY_GO:u32 = 0;
const MONOPOLY_C1:u32 = 11;
const MONOPOLY_E3:u32 = 24;
const MONOPOLY_H2:u32 = 39;
const MONOPOLY_R1:u32 = 5;
const MONOPOLY_R2:u32 = 15;
const MONOPOLY_R3:u32 = 25;
const MONOPOLY_R4:u32 = 35;
const MONOPOLY_U1:u32 = 12;
const MONOPOLY_U2:u32 = 28;

const MONOPOLY_CH_CARD_G2G:u32 = 0;
const MONOPOLY_CH_CARD_G2J:u32 = 1;
const MONOPOLY_CH_CARD_C1:u32  =2;
const MONOPOLY_CH_CARD_E3:u32  =3;
const MONOPOLY_CH_CARD_H2:u32  =4;
const MONOPOLY_CH_CARD_R1:u32  =5;
const MONOPOLY_CH_CARD_NR1:u32  =6;
const MONOPOLY_CH_CARD_NR2:u32  =7;
const MONOPOLY_CH_CARD_NU1:u32  =8;
const MONOPOLY_CH_CARD_B3:u32  =9;

const MONOPOLY_CC_CARD_G2G:u32 = 0;
const MONOPOLY_CC_CARD_G2J:u32 = 1;

pub struct Deck
{
    pub current_card:u32,
    pub size:u32
}

impl Deck
{
    pub fn new(newsize:u32) ->  Deck
    {
        Deck
            {
                size:newsize,
                current_card:0,
            }
    }
    pub fn draw_ch_card(&mut self, old_position:u32) -> u32
    {
        let mut position = old_position;
        match self.current_card
            {
                MONOPOLY_CH_CARD_G2G => position=MONOPOLY_GO,
                MONOPOLY_CH_CARD_G2J => position=MONOPOLY_JAIL,
                MONOPOLY_CH_CARD_C1  => position=MONOPOLY_C1,
                MONOPOLY_CH_CARD_E3 => position=MONOPOLY_E3,
                MONOPOLY_CH_CARD_H2 => position=MONOPOLY_H2,
                MONOPOLY_CH_CARD_R1 => position=MONOPOLY_R1,
                MONOPOLY_CH_CARD_NR1 | MONOPOLY_CH_CARD_NR2  =>
                    {
                        while (position % 5 !=0) ||  (position % 10  ==0) {position+=1; position %=40}
                    }, //Go to next R
                MONOPOLY_CH_CARD_NU1  =>
                    {
                        if position < 12 {position=MONOPOLY_U1;}
                        else if position < 28 {position=MONOPOLY_U2;}
                        else {position=MONOPOLY_U1;}
                    },
                MONOPOLY_CH_CARD_B3   =>
                    {
                        if   position>=3 { position-=3; } else { position=position+40-3; }
                    },              //Go back 3 squares.
                _ => (),
            }
        self.current_card+=1;
        self.current_card %= 16;
        position
    }

    pub fn draw_cc_card(&mut self, old_position:u32) -> u32
    {
        let mut position = old_position;
        match self.current_card
            {
                MONOPOLY_CC_CARD_G2G => position=MONOPOLY_GO,
                MONOPOLY_CC_CARD_G2J => position=MONOPOLY_JAIL,

                _ => (),
            }
        self.current_card+=1;
        self.current_card %= 16;
        position
    }

}

pub fn  play()
{
    let mut rng = rand::thread_rng();
    let mut count:u64=0;
    let mut position_vector:Vec<u64> = vec![0;40];
    let mut position:u32 = 0;let mut doubles =0;
    let mut ch_deck:Deck = Deck::new(16);
    let mut cc_deck:Deck = Deck::new(16);
    loop
    {
        count+=1;
        let roll_1=rng.gen_range(1, 5);
        let roll_2=rng.gen_range(1, 5);
        //println!("roll={} {} {}",roll_1,roll_2,roll_1+roll_2);
        if roll_1 == roll_2
        {
            doubles+=1;
            if doubles >=3
            {
                position =MONOPOLY_JAIL;
                position_vector[position as usize]+=1;
                doubles=0;
               // println!("position={}",position);
                continue;
            }
        }
        else{doubles=0 }
        position+=roll_1+roll_2;
        position %= 40;
        match position
            {
                MONOPOLY_CC1 | MONOPOLY_CC2 | MONOPOLY_CC3 => {/*println!("Community Chest {}");*/ position=cc_deck.draw_cc_card(position)},
                MONOPOLY_CH1 | MONOPOLY_CH2 | MONOPOLY_CH3 => {/*println!("Chance {}");*/ position=ch_deck.draw_ch_card(position)},
                MONOPOLY_G2J => position = MONOPOLY_JAIL,
                _ => (),
            }

       // println!("position={}",position);
        position_vector[position as usize]+=1;
        if count % 1_000_000  == 0
        {
            for (index,f) in position_vector.iter().enumerate()
                {
                    println!("{} {} {} ",index,(*f as f64*100.0)/count as f64,count);
                }
        }
    }
}