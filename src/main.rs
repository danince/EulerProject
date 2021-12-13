#![allow(dead_code)]
#![recursion_limit="256"]

extern crate num_bigint;
extern crate num_traits;
extern crate num_rational;
extern crate primal;
extern crate factorial;
extern crate rusttype;
extern crate rusty_machine;
extern crate rulinalg;
extern crate rand;
extern crate permutohedron;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate fraction;


mod helpers;
mod monopoly;

//use prime_check;
use std::collections::VecDeque;
use num_bigint::BigUint;
use num_bigint::{ToBigUint};
use num_traits::{Zero, One};
use num_bigint::ToBigInt;
use permutohedron::heap_recursive;
//use num_biguint::biguint::IntDigits;
//use std::mem::replace;
//use num_bigint;
use num_traits::pow;
//use std::num;
//use std::env;
use num_bigint::BigInt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;
use std::cmp::Ordering;
use primal::Sieve;
use factorial::Factorial;
use std::time::{Duration, Instant};
use rusttype::Point;
use rusttype::Line;
use helpers::prime_check;
use helpers::prime_check_u64;
use helpers::truncate_biguint_to_u32;
use helpers::is_pandigital;
use helpers::digits_in_number;
use helpers::digit_count;
use helpers::triangle_number;
use helpers::char_value;
use helpers::is_pandigital_zero;
use helpers::is_pentagon;
use helpers::is_triangle;
use helpers::prime_factors;
use helpers::prime_factors_64;
use helpers::number_to_vec;
use helpers::find_permutations;
use helpers::find_permutations_u64;
use helpers::reverse_digits;
use helpers::big_uint_to_vec;
use helpers::print_vec;
use helpers::is_octagon;
use helpers::is_heptagon;
use helpers::is_hexagon;
use helpers::is_square;
use helpers::is_square_64;
use helpers::is_big_square;
use helpers::is_permutation;
use helpers::print_vec_u64;
use helpers::factor_multiply;
use helpers::factor_divide;
use helpers::gcd;
use helpers::number_to_vec_u64;
use helpers::subsets;
use helpers::subsets_64;
use helpers::vec_to_number;
use helpers::vec_to_number_u64;
use helpers::find_pattern;
use helpers::can_be_square;
use helpers::can_be_square_u128;
use helpers::cross_product;
use monopoly::play;
use std::iter::Sum;
use std::ops::Add;
use std::sync::Mutex;
use num::integer::Roots;
use num_rational::Ratio;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use rand::Rng;
use fraction::Fraction;

//use ndarray::arr2;
//use num::BigUint;
//use num::pow::pow;


fn main()
{
    euler_158();
}

//fn euler1()
//{
//    let mut total =0;
//    for f in 0..1000
//        {
//            if f % 3 == 0
//            {
//                total += f
//            }
//            if f % 5 == 0
//            {
//                total += f
//            }
//            if f % 15 == 0
//            {
//                total -= f
//            }
//        }
//    println!("Total = {}",total);
//}

//fn euler2()
//{
//    let mut total =0;
//    let constraint = 4000000;
//    let mut a=0;
//    let mut b=1;
//    let mut temp = 0;
//    loop
//        {
//            temp=b;
//            b =b + a;
//            a = temp;
//            println!( "{}",b);
//            if b>constraint
//            {
//                break;
//            }
//            if b % 2 == 0
//            {
//                total += b;
//            }
//
//
//        }
//    println!("Total = {}",total);
//}

//fn euler3()
//{
//    let mut answer: i64 = 0;
//    let number: i64  = 600851475143;
//    let mut working_number = number;
//    let f_number : f64 = number as f64;
//    let target = f_number.sqrt();
//    let int_target = target as i64  +1;
//    for f in 2 .. int_target
//        {
//            while working_number % f == 0
//            {
//                working_number = working_number / f;
//                answer = f;
//            }
//        }
//    if working_number > answer
//    {
//        answer = working_number;
//    }
//    println!("Answer = {}",answer);
//}

//fn euler4()
//{
//    let mut answer :i32 =0;
//    let mut length =0;
//    for f in 1 .. 999
//        {
//            for g in 1..999
//                {
//                  let total : i32 = f *g;
//
//                    let total_string = total.to_string();
//                    if total_string.len() % 2 ==0
//                    {
//                        length = total_string.len()/2;
//                    }
//                    else
//                    {
//                        length = total_string.len()/2+1;
//                    }
//                    let mut good_string = true;
//                    for h in 0 .. length
//                        {
//                            if total_string.as_bytes()[h as usize] != total_string.as_bytes()[(total_string.len()-(h+1) as usize) as usize]
//                            {
//                                good_string = false;
//                            }
//                        }
//                        if good_string == true
//                        {
//                            if total > answer
//                            {
//                                answer = total;
//                                println!("answer = {}",answer)
//                            }
//                        }
//                }
//        }
//}

const TARGET: usize =20;

//fn euler5()
//{
//    let target_number =TARGET;
//    let mut factor_array: [i64; TARGET +1] = [0; TARGET+1];
//    let mut test_number: i64;
//    let mut unique;
//    let mut total = 1;
//    for f in 1 .. target_number+1
//        {
//            test_number =f as i64;
//            unique =true;
//            for g in 1 .. f-1
//                {
//                    if test_number % factor_array[g] as i64  == 0
//                    {
//                        test_number = test_number / factor_array[g];
//                        //println!("test_number = {}",test_number);
//                    }
//                }
//                println!("factor = {}",test_number);
//                factor_array[f]= test_number;
//                total =total * test_number;
//        }
//    println!("total = {}",total);
//}


//fn euler6()
//{
//    let target_number:i64 = 100;
//    let mut total_1:i64 =0;
//    let mut total_2:i64 =0;
//    for f in 1 .. target_number + 1
//        {
//            total_1 = total_1 + f *f;
//        }
//    println!("total 1 = {}",total_1);
//
//    for f in 1 .. target_number + 1
//        {
//            total_2 = total_2 + f;
//        }
//    total_2 = total_2 * total_2;
//    println!("total 2 = {}",total_2);
//    println!("total = {}",total_2 - total_1);
//}

//fn euler7()  //10001st prime
//{
//    let target = 10001;
//    let mut prime_number_count = 1;
//    let mut largest_prime =2;
//    let mut current_number=3;
//    let mut prime = false;
//    loop
//        {
//            prime = true;
//            for f in 2 .. current_number
//                {
//
//                    if current_number % f  == 0
//                        {
//                            prime = false;
//                            break;
//                        }
//                }
//            if prime
//                {
//                    prime_number_count+=1;
//                    largest_prime=current_number;
//                    println!("prime = {}",current_number);
//                }
//            if prime_number_count >= target
//                {
//                    break;
//                }
//            current_number+=1;
//        }
//
//}

const SIZE: usize =13;

const TEST_STRING: &'static str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

const ZERO:&'static str ="0";
//fn euler8()  //Largest product in a series
//{
//    let mut answer:i64 =0 ;
//    let mut x:i64 =0;
//   // let mut digit_array:[i64;SIZE] = [0,SIZE];
//    for f in 0 .. TEST_STRING.len() -(SIZE -1)
//    {
//        x=1;
//        for g in 0 ..  SIZE
//            {
//                x *= TEST_STRING.as_bytes()[f+g] as i64 -ZERO.as_bytes()[0] as i64;
//            }
////        let x1=TEST_STRING.as_bytes()[f+0]-ZERO.as_bytes()[0];
////        let x2=TEST_STRING.as_bytes()[f+1]-ZERO.as_bytes()[0];
////        let x3=TEST_STRING.as_bytes()[f+2]-ZERO.as_bytes()[0];
////        let x4=TEST_STRING.as_bytes()[f+3]-ZERO.as_bytes()[0];
//        //let scratch:i64 = x1 as i64 * x2 as i64 * x3 as i64 * x4 as i64;
//
//        if x > answer
//        {
//            answer = x;
//        }
//        println!("Answer={}",answer);
//
//    }
//}



//fn euler9() //Special Pythagorean triplet
//{
//    for f in 1 .. 1000
//        {
//            for g in 1 .. 1000-f
//                {
//                            let hsquared = f*f + g*g;
//                            let f_hsquared:f64 = hsquared as f64;
//                            let h:f64= f_hsquared.sqrt();
//                            if h.fract() ==0.00
//                            {
//                                if f + g + h as i32== 1000
//                                {
//                                    println!("Answer={}",f * g * h as i32);
//                                    println!("A,B,C = {} {} {}",f,g,h);
//                                    break;
//                                }
//                            }
////                            if f + g + h == 1000
////                            {
////                                if f*f +g*g == h*h
////                                {
////                                    println!("Answer={}",f * g * h);
////                                    println!("A,B,C = {} {} {}",f,g,h);
////                                    break;
////                                }
////                            }
//
//                }
//        }
//}

//fn euler10()  //	Summation of primes
//{
//    let target = 2000000;
//    let mut total:i64 =2;
//    let mut prime_number_count = 1;
//    let mut largest_prime =2;
//    let mut current_number=3;
//    let mut prime = false;
//    loop
//        {
//            prime = true;
//            let f_current_number:f64 =current_number as f64;
//            let f_Loop_limit:f64= f_current_number.sqrt();
//            let loop_limit:i64 = f_Loop_limit as i64;
//            for f in 2 .. loop_limit +1
//                {
//
//                    if current_number % f  == 0
//                    {
//                        prime = false;
//                        break;
//                    }
//                }
//            if prime
//            {
//                total+=current_number;
//                println!("prime = {}",current_number);
//            }
//            if current_number >= target
//            {
//                break;
//            }
//            current_number+=1;
//        }
//        println!("total = {}",total);
//
//}

const EULER_11_ARRAY_DIM:usize = 20;
const EULER_11_PRODUCT_SIZE:usize = 4;
const EULER_11_TEST_STRING: &'static str = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

//fn euler11()
//{
//    let mut test_array:[[i32; EULER_11_ARRAY_DIM]; EULER_11_ARRAY_DIM] = [[0; EULER_11_ARRAY_DIM]; EULER_11_ARRAY_DIM];
//    let mut count =0;
//    let mut highest:i32 = 0;
//    let mut iter = EULER_11_TEST_STRING.split_whitespace();
//    for f in iter
//        {
//            test_array[count / EULER_11_ARRAY_DIM][count % EULER_11_ARRAY_DIM] = f.parse::<i32>().unwrap();
//            println!("number = {}", f);
//            count+=1;
//        }
//    for f in 0 .. EULER_11_ARRAY_DIM
//    {
//        for g in 0 .. EULER_11_ARRAY_DIM
//            {
//                println!("array value = {}", test_array[f][g]);
//            }
//
//    }
//    for f in 0 .. EULER_11_ARRAY_DIM
//    {
//        for g in 0 .. EULER_11_ARRAY_DIM-(EULER_11_PRODUCT_SIZE)
//            {
//                let mut scratch1 = 1;
//                let mut scratch2 = 1;
//                for h in 0 .. EULER_11_PRODUCT_SIZE
//                    {
//                        scratch1=scratch1*test_array[f][g+h];
//                        scratch2=scratch2*test_array[g+h][f];
//                    }
//                if highest < scratch1
//                {
//                    highest = scratch1;
//                    println!("highest value = {}", highest);
//                }
//                if highest < scratch2
//                {
//                    highest = scratch2;
//                    println!("highest value = {}", highest);
//                }
//            }
//    }
//
//    for f in 0 .. EULER_11_ARRAY_DIM-(EULER_11_PRODUCT_SIZE)
//        {
//            for g in 0 .. EULER_11_ARRAY_DIM-(EULER_11_PRODUCT_SIZE)
//                {
//                    let mut scratch = 1;
//                    for h in 0 .. EULER_11_PRODUCT_SIZE
//                        {
//                            scratch=scratch*test_array[f+h][g+h]
//                        }
//                    if highest < scratch
//                    {
//                        highest = scratch;
//                        println!("highest value = {}", highest);
//                    }
//                }
//        }
//    for f in EULER_11_PRODUCT_SIZE .. EULER_11_ARRAY_DIM
//        {
//            for g in 0 .. EULER_11_ARRAY_DIM-(EULER_11_PRODUCT_SIZE)
//                {
//                    let mut scratch = 1;
//                    for h in 0 .. EULER_11_PRODUCT_SIZE
//                        {
//                            scratch=scratch*test_array[f-h][g+h]
//                        }
//                    if highest < scratch
//                    {
//                        highest = scratch;
//                        println!("highest value = {}", highest);
//                    }
//                }
//        }
//
//}

const EULER_12_DIVISOR_TARGET:usize = 500;
fn euler12()  //Highly divisible triangular number
{
    let mut count:i64 =0;
    let mut total:i64 =0;
    let mut highest_divisor_count=0;
    loop
        {
            let mut divisor_count=0;
            count=count+1;
            total = total+count;
            let f_total:f64 =total as f64;
            let f_loop_limit:f64= f_total.sqrt();
            let loop_limit:i64 = f_loop_limit as i64;
            for f in 1 .. loop_limit+1
                {
                    if total % f == 0
                    {
                        divisor_count+=2;
                    }
                }
            if highest_divisor_count < divisor_count
            {
                highest_divisor_count = divisor_count
            }
            println!("count total divisor_count = {} {} {} {}", count,total,divisor_count,highest_divisor_count);
            if divisor_count> EULER_12_DIVISOR_TARGET
            {
                break;
            }
        }
}


const EULER_13_DIGITS:usize = 50;
const EULER_13_NUMBER_COUNT:usize = 100;
const EULER_13_BUFFER:usize =10;
const EULER_13_TEST_STRING: &'static str ="37107287533902102798797998220837590246510135740250
46376937677490009712648124896970078050417018260538
74324986199524741059474233309513058123726617309629
91942213363574161572522430563301811072406154908250
23067588207539346171171980310421047513778063246676
89261670696623633820136378418383684178734361726757
28112879812849979408065481931592621691275889832738
44274228917432520321923589422876796487670272189318
47451445736001306439091167216856844588711603153276
70386486105843025439939619828917593665686757934951
62176457141856560629502157223196586755079324193331
64906352462741904929101432445813822663347944758178
92575867718337217661963751590579239728245598838407
58203565325359399008402633568948830189458628227828
80181199384826282014278194139940567587151170094390
35398664372827112653829987240784473053190104293586
86515506006295864861532075273371959191420517255829
71693888707715466499115593487603532921714970056938
54370070576826684624621495650076471787294438377604
53282654108756828443191190634694037855217779295145
36123272525000296071075082563815656710885258350721
45876576172410976447339110607218265236877223636045
17423706905851860660448207621209813287860733969412
81142660418086830619328460811191061556940512689692
51934325451728388641918047049293215058642563049483
62467221648435076201727918039944693004732956340691
15732444386908125794514089057706229429197107928209
55037687525678773091862540744969844508330393682126
18336384825330154686196124348767681297534375946515
80386287592878490201521685554828717201219257766954
78182833757993103614740356856449095527097864797581
16726320100436897842553539920931837441497806860984
48403098129077791799088218795327364475675590848030
87086987551392711854517078544161852424320693150332
59959406895756536782107074926966537676326235447210
69793950679652694742597709739166693763042633987085
41052684708299085211399427365734116182760315001271
65378607361501080857009149939512557028198746004375
35829035317434717326932123578154982629742552737307
94953759765105305946966067683156574377167401875275
88902802571733229619176668713819931811048770190271
25267680276078003013678680992525463401061632866526
36270218540497705585629946580636237993140746255962
24074486908231174977792365466257246923322810917141
91430288197103288597806669760892938638285025333403
34413065578016127815921815005561868836468420090470
23053081172816430487623791969842487255036638784583
11487696932154902810424020138335124462181441773470
63783299490636259666498587618221225225512486764533
67720186971698544312419572409913959008952310058822
95548255300263520781532296796249481641953868218774
76085327132285723110424803456124867697064507995236
37774242535411291684276865538926205024910326572967
23701913275725675285653248258265463092207058596522
29798860272258331913126375147341994889534765745501
18495701454879288984856827726077713721403798879715
38298203783031473527721580348144513491373226651381
34829543829199918180278916522431027392251122869539
40957953066405232632538044100059654939159879593635
29746152185502371307642255121183693803580388584903
41698116222072977186158236678424689157993532961922
62467957194401269043877107275048102390895523597457
23189706772547915061505504953922979530901129967519
86188088225875314529584099251203829009407770775672
11306739708304724483816533873502340845647058077308
82959174767140363198008187129011875491310547126581
97623331044818386269515456334926366572897563400500
42846280183517070527831839425882145521227251250327
55121603546981200581762165212827652751691296897789
32238195734329339946437501907836945765883352399886
75506164965184775180738168837861091527357929701337
62177842752192623401942399639168044983993173312731
32924185707147349566916674687634660915035914677504
99518671430235219628894890102423325116913619626622
73267460800591547471830798392868535206946944540724
76841822524674417161514036427982273348055556214818
97142617910342598647204516893989422179826088076852
87783646182799346313767754307809363333018982642090
10848802521674670883215120185883543223812876952786
71329612474782464538636993009049310363619763878039
62184073572399794223406235393808339651327408011116
66627891981488087797941876876144230030984490851411
60661826293682836764744779239180335110989069790714
85786944089552990653640447425576083659976645795096
66024396409905389607120198219976047599490197230297
64913982680032973156037120041377903785566085089252
16730939319872750275468906903707539413042652315011
94809377245048795150954100921645863754710598436791
78639167021187492431995700641917969777599028300699
15368713711936614952811305876380278410754449733078
40789923115535562561142322423255033685442488917353
44889911501440648020369068063960672322193204149535
41503128880339536053299340368006977710650566631954
81234880673210146739058568557934581403627822703280
82616570773948327592232845941706525094512325230608
22918802058777319719839450180888072429661980811197
77158542502016545090413245809786882778948721859617
72107838435069186155435662884062257473692284509516
20849603980134001723930671666823555245252804609722
53503534226472524250874054075591789781264330331690";


fn euler13()
{
    let mut test_array:[[i64; EULER_13_DIGITS+EULER_13_BUFFER]; EULER_13_NUMBER_COUNT] = [[0; EULER_13_DIGITS+EULER_13_BUFFER]; EULER_13_NUMBER_COUNT];
    let mut final_array:[i64; EULER_13_DIGITS+EULER_13_BUFFER] = [0;EULER_13_DIGITS+EULER_13_BUFFER];
    println!("length = {}",EULER_13_TEST_STRING.len());
    let test_string_line= EULER_13_TEST_STRING.split_whitespace();
    let mut line_count=0;
    for iter in test_string_line
        {
            for g in 0 .. EULER_13_DIGITS
                {
                    test_array[line_count][g+EULER_13_BUFFER] = iter.as_bytes()[g] as i64 -ZERO.as_bytes()[0] as i64;
                }
            line_count+=1;
        }
    let mut scratch:i64 = 0;
    for f in (0 .. EULER_13_DIGITS+  EULER_13_BUFFER).rev()
        {

            for g in 0 .. EULER_13_NUMBER_COUNT
                {
                    //println!("g {} ",g);
                    scratch+= test_array[g][f];
                }
            final_array[f]= scratch %10;
            println!("digit {} = {}",f,scratch %10);
            scratch = scratch / 10;


        }


}

const EULER_13_TARGET:usize = 1000000;

fn euler14()  //Longest Collatz sequence
{
    //let mut cach_array:[i32;EULER_13_TARGET]=[0;EULER_13_TARGET];
    //let mut cach_array:Box<[i32; EULER_13_TARGET]> = Box::new([0; EULER_13_TARGET]);
    //let vec = vec![0;EULER_13_TARGET];
    let mut longest_chain=0;
    let mut longest_chain_index=0;
    for f in 2 .. EULER_13_TARGET
        {
            let mut number = f;
            let mut chain_length =0;
            while number != 1
                {
                    chain_length +=1;
                    if number % 2  == 0
                    {
                        number = number /2;
                    }
                    else
                    {
                        number =(number*3)+1;
                    }
                }
            if longest_chain < chain_length
            {
                longest_chain = chain_length;
                longest_chain_index = f;
            }
            println!("chain_length,longest_chain {} {} {}",longest_chain_index,chain_length,longest_chain);

        }
}

const LATTICE_SIZE:usize = 20;

fn euler15() //Lattice paths
{
    let mut count =0;
    let mut cache:[i64;21]=[0;21];
    for f in 2 .. 21
        {
            //let answer = euler15_recursive(0 as i64, 0 as i64, &mut count, &mut cache, f);
            println!("size count {} {}", f,count);
            cache[f as usize]=count;
            count=0;
        }
}

fn euler15_recursive(x: i64, y:i64, count: &mut i64, cache: &mut [i64;21], size: i64 )
{


    if (x < size as i64) & (y < size as i64)
      {
          *count+=1;
          if(x==y) & (cache[size as usize  - x as usize]!=0)
              {
                //  println!("cache hit {}",cache[size as usize + 1 - x as usize]);

                  *count+=cache[size as usize  - x as usize];
                  *count-=1;

              }
          else
          {
              //println!("cache miss");
              if x == size as i64
                  {
                      *count-=1;
                  }
              else if y == size as i64
                  {
                      *count-=1;
                  }

              euler15_recursive(x+1,y, count,cache,size);
              euler15_recursive(x,y+1, count, cache,size);
          }

      }
//    if y < LATTICE_SIZE as i32
//    {
//        euler15_recursive(x,y+1, count);
//    }
    //println!("count {}",count);



}

const EULER_16_POWER_LIMIT:usize =1000;

fn euler16()  //Power digit sum
{
    let mut number_string:[i8;1000] = [0;1000];
    let mut carry =0;
    number_string[0]=1;
    for f in 0 .. EULER_16_POWER_LIMIT
        {
            println!("digit = {}",f);
            for g in 0 .. 1000
            {
                number_string[g] =  (number_string[g]*2) + carry;
                if number_string[g] >= 10
                    {
                        carry = 1;
                        number_string[g] -= 10;
                    }
                else
                {
                    carry = 0;
                }

            }
            let mut total:i64 = 0;
            for f in (0..1000).rev()
            {
                total +=number_string[f] as i64;
                //print!("{}",number_string[f]);
            }
            println!("total={}",total);
        }



}

const EULER_18_NUMBER_OF_LINES:usize =15;
const EULER_18_PYRAMID: &'static str =
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";



const ONE:&'static str ="one";
const TWO:&'static str ="two";
const THREE:&'static str ="three";
const FOUR:&'static str ="four";
const FIVE:&'static str ="five";
const SIX:&'static str ="six";
const SEVEN:&'static str ="seven";
const EIGHT:&'static str ="eight";
const NINE:&'static str ="nine";
const TEN:&'static str ="ten";
const ELEVEN:&'static str ="eleven";
const TWELVE:&'static str ="twelve";
const THIRTEEN:&'static str ="thirteen";
const FORTEEN:&'static str ="fourteen";
const FIFTEEN:&'static str ="fifteen";
const SIXTEEN:&'static str ="sixteen";
const SEVENTEEN:&'static str ="seventeen";
const EIGHTEEN:&'static str ="eighteen";
const NINETEEN:&'static str ="nineteen";
const TWENTY:&'static str ="twenty";
const THIRTY:&'static str ="thirty";
const FORTY:&'static str ="forty";
const FIFTY:&'static str ="fifty";
const SIXTY:&'static str ="sixty";
const SEVENTY:&'static str ="seventy";
const EIGHTY:&'static str ="eighty";
const NINETY:&'static str ="ninety";
const HUNDRED:&'static str ="hundred";
const THOUSAND:&'static str ="thousand";


fn euler17() //Number letter counts
{
    let one_to_ten = ONE.len() + TWO.len() + THREE.len() + FOUR.len() + FIVE.len() + SIX.len() + SEVEN.len() + EIGHT.len() + NINE.len() + TEN.len();
    let one_to_nine = one_to_ten - TEN.len();
    let eleven_to_19 = ELEVEN.len() + TWELVE.len() + THIRTEEN.len() + FORTEEN.len() + FIFTEEN.len() + SIXTEEN.len() + SEVENTEEN.len() + EIGHTEEN.len() + NINETEEN.len();
    let units: [&'static str;10] = ["zero",ONE,TWO,THREE,FOUR,FIVE,SIX,SEVEN,EIGHT,NINE];
    let tens: [&'static str;10] = ["  ", TEN, TWENTY, THIRTY, FORTY, FIFTY, SIXTY, SEVENTY, EIGHTY, NINETY];
    println!("First ten = {}", one_to_ten);
    println!("First nineteen = {}", eleven_to_19);
    let mut total = one_to_ten + eleven_to_19;
    for f in 2..10
        {
            total = total + tens[f].len() * 10;
            total += one_to_nine;
        }
    let first_ninety_nine =total;
    println!("first 99 {}", total);

    for f in 1 .. 10
        {
            println!("{}",units[f]);
            total += units[f].len() *100;
            total += HUNDRED.len() * 100;
            total += "and".len() * 99;
            total += first_ninety_nine;
        }
    total += ONE.len();
    total += THOUSAND.len();
      println!("Total= {}", total) ;


}
fn euler18()
{
    //let mut number_of_cells_on_this_line =1;
    let mut test_array:[[i32; EULER_18_NUMBER_OF_LINES]; EULER_18_NUMBER_OF_LINES] = [[0; EULER_18_NUMBER_OF_LINES]; EULER_18_NUMBER_OF_LINES];
    let mut current_position = 0;
    let mut current_line_number = 0;
    let  iter = EULER_18_PYRAMID.split_whitespace();

    for f in iter
        {

         //   println!("number = {}", f);
            test_array[current_position][current_line_number]=f.parse::<i32>().unwrap();
            current_position+=1;
            if current_position>current_line_number
                {
                    current_position = 0;
                    current_line_number +=1;
                }
        }
        for f in (0 .. EULER_18_NUMBER_OF_LINES-1).rev()
            {
                for g in 0 .. f+ 1
                    {
                        if test_array[g][f]+test_array[g][f+1] > test_array[g][f]+test_array[g+1][f+1]
                            {
                                test_array[g][f]= test_array[g][f]+test_array[g][f+1];
                            }
                        else
                        {
                            test_array[g][f]= test_array[g][f]+test_array[g+1][f+1];
                        }
                    }
                for g in 0 .. f + 1
                    {
                        println!("number = {}", test_array[g][f]);
                    }
                println!("break")
            }

}

fn euler19() //Counting Sundays
{
    let month_days:[i32;12]=[31,28,31,30,31,30,31,31,30,31,30,31];
    let mut day_index =2;
    let mut total =0;
    for f in 1901 .. 2001
      {
          let mut feb_plus =0;

          if f % 4 == 0 //& (f % 100 != 0 ) %
              {
                  feb_plus=1;
              }
          for g in 0 .. 12
              {
                  if day_index ==0
                      {
                          total+=1;
                      }
                  let mut day_count = month_days[g];
                  if g == 1
                      {
                          day_count+= feb_plus;
                      }
                  day_index+=day_count;
                  day_index = day_index    % 7;
                  println!("day_index={}",day_index);

              }
          println!("total= {}",total);

      }
}



fn euler20() //Factorial digit sum
{
    //let mut total:i64  =1;
    let mut total: BigUint = One::one();
    //let mut twos:i64 =0;
    //let mut threes:i64 = 0;
    let mut digit_total = 0;
    for f in 1 .. 100
    {
        //let big_f:BigUint = f.to_biguint().unwrap();
        total =total *f.to_biguint().unwrap();
        println!("total= {} {}",f,total);

    }
    for _f in 1 .. 2000
        {
            let nt =total.clone();
            let big_remainder= nt % 10.to_biguint().unwrap();
            let remainder = truncate_biguint_to_u32(&big_remainder);
            digit_total =digit_total+remainder;
            total =total / 10.to_biguint().unwrap();

        }
    println!("digit_total {}",digit_total);
}

fn vector_sum(vec:Vec<i64>)->i64
{
    let mut sum:i64=0;
    for iter in vec
        {
           sum+=iter;
        }
    sum
}

fn vector_sum_u32(vec:Vec<u32>)->u32
{
    let mut sum:u32=0;
    for iter in vec
        {
            sum+=iter;
        }
    sum
}

fn find_divisors(number: u64) ->Vec<u64>
{
    let mut vec = Vec::new();
    let f_number : f64 = number as f64;
    let temp = f_number.sqrt();
    let number_sqrt = temp as u64;
    //println!("number = {}",number);
    if number==1
    {
        vec.push(number);
    }
    else
    {
        for f in 1..number_sqrt + 1
            {
                if number % f as u64 == 0
                {
                    vec.push(f);
                    if (f !=number/f) & (f!=1)
                    {
                        vec.push(number/f);
                    }
                    //println!("divisor = {}", f);
                }
            }
//        if number_sqrt * number_sqrt == number
//        {
//            vec.push(number_sqrt);
//            println!("sqrt");
//        }
    }
    vec

}

//fn euler21()//Amicable numbers
//{
//    let mut total = 0;
//    for f in 1 .. 10000
//        {
//            let divisors =find_divisors(f) ;
//            let sum = vector_sum(divisors);
//            //println!("sum ={}", sum);
//            let complimentary_divisors = find_divisors(sum) ;
//            let complimentary_sum = vector_sum(complimentary_divisors);
//            if (complimentary_sum == f) & (sum!=f)
//            {
//                total+=f;
//                println!(" {} {} ",f,sum);
//            }
//
//        }
//    println!("total ={}",total);
//}
//

//fn euler22() //Names scores
//{
//    let contents = fs::read_to_string("p022_names.txt")
//        .expect("Something went wrong reading the file");
//    let mut count:i32=0;
//    let mut total:i32 = 0;
//    let mut vec: Vec<&str> = contents.split(',').collect();
//    vec.sort();
//    for f in vec
//        {
//            count+=1;
//            let mut name_score =0;
//            let temp=f.trim_matches('"');
//            let trimmed_name=temp.trim();
//            println!(" name = {}",trimmed_name);
//            for g in 0 ..  trimmed_name.len()
//            {
//                print!("{} ",trimmed_name.as_bytes()[g]-64);
//                name_score+=trimmed_name.as_bytes()[g]-64;
//            }
//            total +=count * name_score as i32;
//            println!("Name score {} {} {}",count,name_score,total);
//
//
//        }
//}


const  EULER_23_CONSTRIANT:i64 = 28123;
//fn euler23() //Non-abundant sums
//{
//    let mut total =0;
//    let mut vec = Vec::new();
//    for f in 1 .. EULER_23_CONSTRIANT
//        {
//            let divisors =find_divisors(f) ;
//            let sum = vector_sum(divisors);
//            if sum > f
//            {
//                println!("Abundant = {}",f);
//                vec.push(f);
//            }
//        }
//    for f in 1 .. EULER_23_CONSTRIANT
//        {
//            let mut abundant_sum =false;
//            //let mut vec2 =vec.clone();
//            for g in &vec
//                {
//                    if *g > f
//                    {
//                        break;
//                    }
//                    let other_value=f-g;
//                    if vec.contains(&other_value)
//                    {
//                        abundant_sum = true;
//
//                        break;
//                    }
//                }
//            if abundant_sum==false
//            {
//                total+=f;
//                println!("Target value {} {}",f,total);
//            }
//        }
//}
const PERM_SIZE:usize = 10;
const EULER_24_ARRAY:[i32;PERM_SIZE]=[0;PERM_SIZE];

const EULER_24_CONSTRAINT:i32 = 1000000;
fn euler24() //Lexicographic permutations
{
    let mut count =0;
    let test:[i32;PERM_SIZE]=[0;PERM_SIZE];
    euler_24_recursive(0,test, &mut count);

}

fn euler_24_recursive(index:i32,mut test:[i32;PERM_SIZE],count: &mut i32)
{
    for f in 0 .. PERM_SIZE
        {
            let mut used =false;
            for g in 0 .. index
                {
                    if test[g as usize]   == f as i32
                          {
                            used =true
                          }
                }
            if used ==false
                {
                    test[index as usize] = f as i32;
                    if index == PERM_SIZE  as i32 -1
                        {
                            *count+=1;
                            print!("{}:  ",*count );
                            for h in 0 ..PERM_SIZE
                                {
                                    print!("{} ", test[h]);
                                }
                            println!(" ");
                            if *count >= EULER_24_CONSTRAINT
                            {
                                break;
                            }
                            //println!("{}: {} {} {}", *count, test[0], test[1], test[2]);
                        }
                    if index != PERM_SIZE as i32 - 1
                        {
                            euler_24_recursive(index + 1, test,count);
                        }
                }




        }
}
fn euler25()
{
    let mut total = 0;
    let mut highest_digit=0;
    let constraint = 1000;
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut temp: BigUint;
    loop
        {
            total+=1;   //iteration count
            temp = b.clone();
            b = b + a;
            a = temp;
            println!("iteration, value {} {}", total, a);
            let mut working_b =a.clone();
            for f in 1 .. 2000
                {
                    let nt =working_b.clone();
                    let big_remainder= nt % 10.to_biguint().unwrap();
                    let remainder = truncate_biguint_to_u32(&big_remainder);
                    if remainder != 0
                    {
                        highest_digit= f;
                    }
                    working_b =working_b / 10.to_biguint().unwrap();

                }
            if highest_digit >= constraint
            {
                println!("Highest digit = {}",highest_digit);
                break;
            }
        }
}

fn euler26()//Reciprocal cycles
{
    let mut highest=0;
    for f in 3 .. 1000
        {
            let mut count =1;
            let mut number  = f;
            let mut magic_mumber=9;
            while number %2 ==0
                {
                    number=number/2;
                }
            while number % 5 == 0
                {
                    number=number/5;
                }
            if number!=1
            {
                println!("test = {}",magic_mumber % number  );
                while magic_mumber % number != 0
                    {
                        //println!("test = {}",9 % number  );
                        magic_mumber = magic_mumber % number;

                        magic_mumber = magic_mumber * 10;
                        magic_mumber = magic_mumber + 9;
                        count += 1;
                    }
                if highest<count
                {
                    highest=count;
                }
                println!("Number ={} {} {}", f, count,highest);

            }
            else
            {
                println!("Number ={} is trivial", f);
            }

        }
}



const EULER_27_A:i32 = 1000;
const EULER_27_B:i32 = 1000;
fn euler27()  //Quadratic primes
{
    let mut highest = 0;
    let mut highest_a = 0;
    let mut highest_b = 0;
    for f in 0 .. 100
    {
        let x =prime_check(f);
        println!("{} {}",f,x);
    }
    for f in -EULER_27_A+1 .. EULER_27_A
        {
            for g in -EULER_27_B .. EULER_27_B+1
                {
                    let mut n =0;
                    //let mut loop_count=0;
                    loop
                        {
                            //loop_count+=1;
                            let total = n*n + f*n +g;
                            if !prime_check(total)
                            {
                                break;
                            }
                            n=n+1;
                        }
                    if highest<n
                    {
                        highest_a =f;
                        highest_b =g;
                        highest=n;
                    }
                        println!("{} {} {} {} {} {}",f,g,n,highest,highest_a,highest_b);

                }
            println!("{}",highest_a*highest_b);
        }
}
const EULER_28_GRID_SIZE:usize = 1001;
fn euler28()//Number spiral diagonals
{
    let mut scratch_count=1;
    let mut scratch =1;
    for f in 1 .. EULER_28_GRID_SIZE
        {
            scratch+=2 * f;
            scratch_count+=scratch;
        }

    println!("First Diagonal {}",scratch_count);
    // 4 4 8 8 12 12
    let mut total =scratch_count;
    scratch_count=1;
    scratch =0;

    for f in 1 .. EULER_28_GRID_SIZE
        {
            if f % 2 ==1
            {
                scratch = 4 * (f/2 +1);
            }
            println!("Scratch = {} {}",scratch, scratch_count);
            scratch_count+=scratch;
            total+=scratch_count;
        }
    println!("Second Diagonal {}",total);
    println!("total {}",total);
}
const EULER_29_CONSTRAINT:i32 = 100;


fn euler29() //Distinct powers
{
    let mut vec: Vec<BigUint> = Vec::new();
    for f in 2..EULER_29_CONSTRAINT + 1
        {
            for g in 2..EULER_29_CONSTRAINT + 1
                {
                    let big_f = f.to_biguint().unwrap();
                    let x: BigUint = num_traits::pow(big_f, g as usize);
                    println!("{}", x);
                    if !vec.contains(&x)
                    {
                        vec.push(x)
                    }
                }
        }
    println!("total = {}", vec.len());
}

fn euler30()  //Digit fifth powers
{
    let mut vec: Vec<i32> = Vec::new();
    for f in 2 .. 10000000
        {
            let mut total = 0;
            let mut number = f;

            for _g in 0 .. 10
                {
                    let scratch = number % 10;
                    let scratch_to_5 = scratch * scratch * scratch * scratch * scratch;
                    total+=scratch_to_5;
                    if total > f
                    {
                        break;
                    }
                    number =number/10;
                }
            if total == f
            {
                println!("number = {}",f);
                vec.push(f);
            }

        }
    let mut total=0;
    for iter in vec
        {
            total+=iter;
        }
    println!("total = {}",total);
}

const EULER_31_COINS:[i32;8] = [1,2,5,10,20,50,100,200];
const TARGET_VALUE:i32 =200;
fn euler31() //Coin sums
{
    let mut coin_multiples:[i32;8]=[0;8];
    for f in 0 .. 8
        {
            coin_multiples[f]=200/EULER_31_COINS[f];
        }
    let mut count=1;  //one for the 2 lb coin
    let mut total;
    for f7 in 0 .. coin_multiples[6]+1
        {
            let f7_val=EULER_31_COINS[6]*f7;
            for f6 in 0 .. coin_multiples[5]+1
                {
                    let f6_val=(EULER_31_COINS[5]*f6)+f7_val;
                    if(f6_val) > TARGET_VALUE
                    {
                        break;
                    }
                    for f5 in 0 .. coin_multiples[4]+1
                        {
                            let f5_val=(EULER_31_COINS[4]*f5)+f6_val;
                            if(f5_val) > TARGET_VALUE
                            {
                                break;
                            }
                            for f4 in 0 .. coin_multiples[3]+1
                                {
                                    let f4_val=(EULER_31_COINS[3]*f4)+f5_val;
                                    if(f4_val) > TARGET_VALUE
                                    {
                                        break;
                                    }
                                    for f3 in 0 .. coin_multiples[2]+1
                                        {
                                            let f3_val=(EULER_31_COINS[2]*f3)+f4_val;
                                            if(f3_val) > TARGET_VALUE
                                            {
                                                break;
                                            }
                                            for f2 in 0 .. coin_multiples[1]+1
                                                {
                                                    let f2_val=(EULER_31_COINS[1]*f2)+f3_val;
                                                    if(f2_val) > TARGET_VALUE
                                                    {
                                                        break;
                                                    }
                                                    for f1 in 0 .. coin_multiples[0]+1
                                                        {
                                                            total=EULER_31_COINS[0]*f1 +f2_val;
                                                            if(total) > TARGET_VALUE
                                                            {
                                                                break;
                                                            }
//                                                           total= EULER_31_COINS[0]*f1+
//                                                               EULER_31_COINS[1]*f2+
//                                                               EULER_31_COINS[2]*f3+
//                                                               EULER_31_COINS[3]*f4+
//                                                               EULER_31_COINS[4]*f5+
//                                                               EULER_31_COINS[5]*f6+
//                                                               EULER_31_COINS[6]*f7;
                                                               if total==200
                                                                   {
                                                                       count += 1;
                                                                   }
                                                               println!("total={} {}",total,count);

                                                        }
                                                }
                                        }
                                }
                        }
                }
        }
}


fn euler32() //Pandigital products
{
    let mut digit_used:[bool;10]=[true;10];
    let mut vec:Vec<i64> = Vec::new();
    for f in 1 .. 100000
        {
            let mut bad_digit =false;
            for g in 1 .. 10
                {
                    digit_used[g]=false;
                }
            let digit_count_1 = f.to_string().len();
            let mut number =f;
            for _g in 0 .. digit_count_1
                {
                    let digit = number % 10;
                    if digit_used[digit]
                    {
                            bad_digit=true;
                            break;
                    }
                    digit_used[digit] =true;
                    number=number/10;
                }
            if !bad_digit
            {
               // println!("number = {}", f);
            }
            if !bad_digit
                {
                for g in 0 ..100000
                    {
                        let mut bad_digit = false;
                        let mut digit_used_for_g = digit_used.clone();
                        let digit_count_2 = g.to_string().len();
                        let mut number = g;
                        for _h in 0..digit_count_2
                            {
                                let digit = number % 10;
                                if digit_used_for_g[digit]
                                    {
                                        bad_digit = true;
                                        break;
                                    }
                                digit_used_for_g[digit] = true;
                                number = number / 10;
                            }
                        if !bad_digit
                            {
                               // println!("number = {} {}", f, g);
                                let product: i64 = f as i64 * g as i64;
                                let digit_count = product.to_string().len();
                                if digit_count + digit_count_1 + digit_count_2 > 9
                                    {
                                      //  println!("Number too big");
                                        break;
                                    }
                                if digit_count + digit_count_1 + digit_count_2  ==9
                                {
                                    let mut number = product;
                                    let mut digit_used_for_product = digit_used_for_g.clone();
                                    for _h in 0..digit_count
                                        {
                                            let digit = number % 10;
                                            if digit_used_for_product[digit as usize]
                                            {
                                                bad_digit = true;
                                                break;
                                            }
                                            digit_used_for_product[digit as usize] = true;
                                            number = number / 10;
                                        }
                                    if !bad_digit
                                    {
                                        println!("number = {} {} {}", f, g, product);
                                        if !vec.contains(&product)
                                        {
                                            vec.push(product)
                                        }
                                    }
                                }
                            }
                    }
            }
        }
        println!("Sum={}",vector_sum(vec));
}

fn euler33()  //Digit cancelling fractions
{
    let mut num_vec:Vec<i32> = Vec::new();
    let mut dem_vec:Vec<i32> = Vec::new();
    for f in 10 .. 100
        {
            for g in 10 .. 100
                {
                    if f <g
                    {
                        let high_digit_f =f / 10;
                        let high_digit_g =g / 10;
                        let low_digit_f =f %10;
                        let low_digit_g =g% 10;
                        if(high_digit_f == high_digit_g) & (high_digit_f != 0) & (low_digit_g != 0)
                        {
                            if (f as f64/g as f64) as f64 == (low_digit_f as f64/low_digit_g as f64) as f64
                            {
                                println!("{}/{} {}/{}",f,g,low_digit_f,low_digit_g);
                                num_vec.push(low_digit_f);
                                dem_vec.push(low_digit_g);
                            }
                        }
                        if(low_digit_f == low_digit_g) & (low_digit_f != 0) & (high_digit_g != 0)
                        {
                            if (f as f64/g as f64) as f64 == (high_digit_f as f64/high_digit_g as f64) as f64
                            {
                                println!("{}/{} {}/{}",f,g,high_digit_f,high_digit_g);
                                num_vec.push(high_digit_f);
                                dem_vec.push(high_digit_g);
                            }
                        }
                        if(low_digit_f == high_digit_g) & (low_digit_f != 0) & (low_digit_g != 0)
                        {
                            if (f as f64 /g as f64) as f64 == (high_digit_f as f64/low_digit_g as f64) as f64
                            {
                                println!("{}/{} {}/{}",f,g,high_digit_f,low_digit_g);
                                num_vec.push(high_digit_f);
                                dem_vec.push(low_digit_g);
                            }
                        }
                        if(high_digit_f == low_digit_g) & (high_digit_f != 0) & (high_digit_g != 0)
                        {
                            if (f as f64/g as f64) as f64 == (low_digit_f as f64/high_digit_g as f64) as f64
                            {
                                println!("{}/{} {}/{}",f,g,low_digit_f,high_digit_g);
                                num_vec.push(low_digit_f);
                                dem_vec.push(high_digit_g);
                            }
                        }



                    }
                }
        }
    let mut numerator=1;
    let mut denoninator = 1;
    for f in 0 .. num_vec.len()
        {
            numerator *= num_vec[f];
            denoninator *= dem_vec[f];
        }
    println!("{}/{}",numerator,denoninator);
    for f in (2 .. numerator).rev()
        {
            if(numerator %f ==0) & (denoninator %f ==0)
            {
                numerator/=f;
                denoninator/=f;
            }
        }
    println!("{}/{}",numerator,denoninator);


}
fn euler34()  //Digit factorials
{
    //let mut count=0;
    for f in 10 .. 10000000
        {
            //println!("f ={}",f);
            let mut total=0;
            let mut number =f;
            for _g in 0  .. 10
                {
                    let digit:u32= number %10;
                    let digit_factorial = digit.factorial();
                    //println!("Digit Factorial {}",digit_factorial);
                    total+=digit_factorial;
                    number = number /10;
                    if total>f
                    {
                        break;
                    }
                    if number==0
                    {
                        break;
                    }
                }
            if f == total
            {
                //count+=1;
                println!("number = {}",f);
            }


        }

}

const EULER_35_CONSTRAINT:usize =1000000;
fn euler35() //Circular primes
{
    let mut count =0;
    for f in 1 .. EULER_35_CONSTRAINT+1
    {
        let digit_count = f.to_string().len();
        //println!("digit count = {}",digit_count);
        let mut number:i32 = f as i32;
        let mut prime =true;
        for _g in 0 .. digit_count
            {
                prime= prime & prime_check(number);
                if !prime
                {
                    break;
                }
                let last_digit =  number %10;
                number =number /10;
                number = number + last_digit * num_traits::pow(10,digit_count-1);
            }
        if prime
        {
            count+=1;
            println!("number={}",f);
        }
    }
    println!("count= {}",count);
}
const EULER_36_CONSTRAINT:usize =1000000;
fn euler36() //Double-base palindromes
{
    let mut total =0;
    for f in 1 .. EULER_36_CONSTRAINT+1
        {
            let mut palindrome = true;
            let f_string = f.to_string();
            if f_string.len() !=1
            {
                for g in 0 .. f_string.len()/2
                {
                    if f_string.as_bytes()[g] != f_string.as_bytes()[f_string.len()-(g+1)]
                    {
                        palindrome=false;
                    }
                }
            }
            if palindrome
            {
                let binary = format!("{:b}", f);
                for g in 0 .. binary.len()/2
                    {
                        if binary.as_bytes()[g] != binary.as_bytes()[binary.len()-(g+1)]
                        {
                            palindrome=false;
                        }
                    }
                if palindrome
                {
                    total +=f;
                    println!("number={} {} {}", f, binary,total);
                }
            }
        }
}

const EULER_37_CONSTRAINT:usize =1000000;
fn euler37() // Truncatable primes
{
    let mut total=0;
    for f in 10 .. EULER_37_CONSTRAINT+1
        {
            let digit_count = f.to_string().len();
            //println!("digit count = {}",digit_count);
            let mut number:i32 = f as i32;
            let mut prime =true;
            for _g in 0 .. digit_count
                {
                    //println!("number={}",number);
                    prime =prime & prime_check(number);
                    if !prime
                    {
                        break;
                    }
                    number = number /10;
                }
            //prime =true;
            if prime
            {
                number = f as i32;
                for g in 0 .. digit_count
                    {
                        //println!("number={}",number);
                        prime =prime & prime_check(number);
                        if !prime
                        {
                            break;
                        }
                        let highest_digit = number / num_traits::pow(10,digit_count-(g+1));
                        //println!("highest digit {}",highest_digit);
                        number = number - highest_digit * num_traits::pow(10,digit_count-(g+1));
                    }
            }
            if prime
            {
                println!("prime = {}",f);
                total+=f;
            }
        }
    println!("total={}",total);
}




fn euler38()  //Pandigital multiples
{
    let multiplier_array: [u32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
    //let mut bigest = 0;


    for f in 1..100000 // / multiplier_array[ h]
        {
            let mut vec:Vec<u32> = Vec::new();
            let mut total: u32 ;
            let mut digits=0;
            //let mut bad_number;
            for g in 1..10
                {
                    let product: u32 = f * g;
                    //println!("f,g,product = {} {} {}",f,g,product);
                    if !is_pandigital(product)
                            {
                                //bad_number = true;
                                break;
                            }
                    vec.push(product);
                    digits += digits_in_number(product as u32);
                    //println!("product,digits={},{}",product,digits);
                    if digits > 9
                    {
                                //bad_number = true;
                                break;
                    }
                    if digits == 9
                    {
                        total = 0;
                        for h in 0..vec.len()
                            {
                                //println!("not total={}", total);
                                total = total * multiplier_array[digit_count(vec[h]as i32) as usize];

                                total += vec[h];
                                //println!("tmp total={}  {}",total,vec[h]);
                            }
                        if is_pandigital(total)
                        {
                            println!("total={}", total);
                        }
                        break;
                    }


//                    if bigest < total    //BAD THINGS HAPPENIF THIS IS NOT COMMENTED OUT
//                    {
//                        bigest = total;
//                    }
//                    println!("total {} {}", total, bigest);
                }
//            if(bad_number)
//            {
//                break;
//            }
        }
}

const EULER_39_CONSTRAINT:usize =1000;
fn euler39() // Integer right triangles
{
    let mut highest =0;
    for f in 1 .. EULER_39_CONSTRAINT
        {
            let mut count=0;
            for g in 1 .. f
                {
                    for h in g .. f
                        {
                            let hypot_squared=g * g + h *h;
                            let f_hypot_squared:f64 = hypot_squared as f64;
                            let hypot= f_hypot_squared.sqrt();
                            if hypot.fract()==0.0
                            {
                                if h+g+hypot as usize == f
                                {
                                    count+=1;
                                    if highest < count
                                    {
                                        highest =count;
                                        println!("{} {}",f,highest);
                                    }
                                    //println!("{},{},{},{}",f,g,h,hypot);
                                }
                            }
                        }
                }
        }
}

const EULER_40_CONSTRAINT:usize =1000000;
fn euler40() // Integer right triangles
{
    let mut target_string:String ="".to_string();
    for f in 1 .. EULER_40_CONSTRAINT
        {
            target_string.push_str(&f.to_string());
            //println!("{}",target_string);
            if target_string.len() > 1100000
            {
                println!("{}",target_string);
                let target = (target_string.as_bytes()[1-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[10-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[100-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[1000-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[10000-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[100000-1]-ZERO.as_bytes()[0])*
                             (target_string.as_bytes()[1000000-1]-ZERO.as_bytes()[0]);
                println!("{} {} ",target_string.as_bytes()[10-1]-ZERO.as_bytes()[0],target);
                break;
            }
        }
}

fn euler41() //Pandigital prime
{
    let multiplier_array:[i32;10]=[1,10,100,1000,10000,100000,1000000,10000000,100000000,1000000000];
    let mut digit_used:[bool;10]=[true;10];
    //let mut vec:Vec<i64> = Vec::new();
    let mut f = 0; //999999999;
    while f < 1000000000
    //for f in 1 .. 1000//000000
        {
            let mut bad_digit = false;
            for g in 0..10
                {
                    digit_used[g] = true;
                }
            for g in 1.. f.to_string().len()+1
                {
                    digit_used[g] = false;
                }
            let f_string=f.to_string();
            let digit_count_1 = f_string.len();
            //let mut number = 0;
            for g in 0..digit_count_1
                {
                    let digit = f_string.as_bytes()[g]-ZERO.as_bytes()[0];
                    if digit_used[digit as usize]
                    {
                        bad_digit = true;
                        break;
                    }
                    digit_used[digit as usize] = true;
                    //number = number / 10;
                }
            if !bad_digit
            {

                let prime =prime_check(f);
                if prime
                {
                    println!("number = {}", f);
                }
                f=f+1;
            }
            else
            {
                let mut  multiplier=1;
                for g in (1..digit_count_1).rev()
                    {
                        if digit_used[g]
                        {
                            //multiplier=num_traits::pow(10,digit_count_1-(g+1));
                            multiplier=multiplier_array[digit_count_1-(g+1)];
                            break;
                        }
                    }
                f=f+multiplier;
            }
        }
}

fn euler42() //Coded triangle numbers
{
    let contents = fs::read_to_string("p042_words.txt")
        .expect("Something went wrong reading the file");
    //let mut count:i32=0;
    //let mut total:i32 = 0;
    let mut total_count=0;
    let mut vec: Vec<&str> = contents.split(',').collect();
    vec.sort();
    let  triangle_number_vec = triangle_number(200);
    for f in vec
        {
            //count += 1;
            //let mut name_score = 0;
            let temp = f.trim_matches('"');
            let trimmed_name = temp.trim();

            let mut total:u32 =0;
            for g in 0 .. trimmed_name.len()
                {
                   // print!(" ={} ",total);
                    total += char_value(trimmed_name.chars().nth(g).unwrap()) as u32;
                }
            print!(" ={} ",total);
            println!(" name = {}", trimmed_name);
            if triangle_number_vec.contains(&total)
            {
                total_count+=1;
                println!("triangle");
            }
        }
    println!("total = {}",total_count);
}

fn euler43() //Sub-string divisibility
{
    let mut sum:u64 =0;
    let mut f:u64 = 1000000000;
    while f<= 9999999999
      {
          f+=1;
          let  d2d3d4 = (f /1000000) %1000;
          if  d2d3d4 %2 !=0
          {
              continue;
          }
          let  d3d4d5 = (f /100000) %1000;
          if  d3d4d5 %3 !=0
          {
              continue
          }
          let  d4d5d6 = (f /10000) %1000;
          if  d4d5d6 %5 !=0
          {
              continue;
          }
          let  d5d6d7 = (f /1000) %1000;
          if  d5d6d7 %7 !=0
          {
              continue;
          }
          let  d6d7d8 = (f /100) %1000;
          if  d6d7d8 %11 !=0
          {
              continue;
          }
          let  d7d8d9 = (f /10) %1000;
          if  d7d8d9 %13 !=0
          {
              continue;
          }
          let  d8d9d10 = (f /1) %1000;
          if  d8d9d10 %17 !=0
          {
              continue;
          }
          //println!("maybe={}",f);
          if !is_pandigital_zero(f)
          {
              continue;
          }
          sum+=f;
          println!("total={} {}",f,sum);

      }
}



fn euler44()  //Pentagon numbers
{
    for f in 2 .. 100
        {
            println!("{}",((3 * f) -1) *f  /2);
        }
    for f  in 1 .. 10000
        {
            let pent_f = ((3 * f) -1) *f  /2;
            for g in (1 .. f).rev()
                {
                    let pent_g = ((3 * g) -1) * g /2;
                    let sum =pent_f + pent_g;
                    let difference = pent_f -pent_g;
                    if (is_pentagon(difference)) & (is_pentagon(sum))
                        {
                            println!("target= {} {} {}",pent_f,pent_g, pent_f -pent_g);
                            break;
                        }


                }
        }
}


fn euler45() //Triangular, pentagonal, and hexagonal
{
    for f in 1 ..100000000
        {
            let f_hex=f * ((2 * f) - 1 );
                  if is_pentagon(f_hex)
                      {
                          if is_triangle(f_hex)
                              {
                                  println!("target={} {}", f,f_hex);
                              }
                      }

        }

}
////const digits:Vec<&'static str> ="one";
//fn get_next_pandigit(number:u32) -> u32
//{
//    let mut digits:Vec<&'static str> = Vec::new();
//    for f in 1 .. 9
//        {
//            digits.push(&f.to_string());
//        }
//    let mut newnumber =0;
//    let number_as_string = number.to_string();
//    let digit_count= number_as_string.len();
//    let number_plus_1 = number+1;
//    if digit_count != number_plus_1.to_string().len()
//    {
//        newnumber=number_plus_1;
//    }
//    else
//    {
//        let  mut working_digit_place=digit_count-1;
//        let mut working_number=number;
//        let working_digit = working_number %10;
//
//
////            {
////              for g in 1 .. working_digit_place
////        for f in working_number .. digit_count
////                {
////                    if number_as_string.to_bytes()[g] == f.to_bytes()[0]
////                    {
////
////                    }
////                }
////            }
//    }
//    newnumber
//}


fn euler46() //Goldbach's other conjecture
{
    for f in 4 .. 100000000
        {
            //let mut done = false;
            let mut found_one =false;
            let number = (2 * f) +1;
            if !prime_check(number)
            {
                println!("number={}",number);
                for g in 2 .. number
                {
                    if prime_check(g)
                    {
                        let double_square = number - g;
                        if double_square %2 == 0  //if even
                        {
                            let square =double_square /2;
                            let f_square = square as f64;
                            let target = f_square.sqrt();
                            if target.fract() == 0.0
                            {
                                found_one = true;
                                break;
                            }

                        }
                    }
                }
                if found_one == false
                {
                    break;
                }
            }
        }
}


fn euler47() //Distinct primes factors
{
    let mut count=0;
    for f in 2 .. 1000000
        {
            //let mut bad_set:bool=false;
           // let vec=prime_factors(f);
            let mut vec=prime_factors(f);
            if vec.len() < 4
            {
                count = 0;
                continue
            }
            //let vec_no_dup =vec.dedup();
            if vec.len() ==4
            {
                   count+=1;
                   println!("count={} {}",f,count);
            }
            else
            {
                count=0;
            }

            if count == 4
            {
                println!("numbers = {} {} {} {}",f,f+1,f+2,f+3);
                break;

            }


        }
}




fn euler48() //Self powers
{
    let mut total:BigUint= BigUint::zero();
    for f in 1 .. 1001
        {
            let big_f:BigUint= f.to_biguint().unwrap();
            let x: BigUint = num_traits::pow(big_f, f as usize);
            total+=x;
            println!("total={}",total);

        }
}


//fn find_permutations(num:u32) ->Vec<u32>
//{
//    let digits =num.len();
//    let num_as_string= num.to_string();
//    for f in 0  .. digits
//    {
//        let mut working_string = num_as_string;
//        working_string.swap()
//    }
//}



//fn find_permutations(num:u32) ->Vec<u32>
//{
//
//   // let num_as_string= num.to_string();
//    let mut number_vec:Vec<u32> =Vec::new();
//    let mut working_number=num;
//    while working_number>0
//        {
//            number_vec.push(working_number %10);
//            working_number/=10;
//        }
//    let digits =number_vec.len();
//    let mut vec:Vec<u32> = Vec::new();
//    print_vec(&number_vec);
//    for f in 0  .. digits
//        {
//
//        }
//    vec
//}








fn euler49() //Prime permutations
{
    for f in 1000 .. 9999
        {
            let mut number_vec=number_to_vec(f);
            let mut length = number_vec.len();
            let mut return_vec:Vec<u32> =Vec::new();
            find_permutations(&mut number_vec,length as u32, &mut return_vec);
            //let mut count =0;
            return_vec.sort();
            //println!("number={}",f);
            for g in (0 .. 24).rev()
                {
                    //println!("other={}",return_vec[g]);
                    if !prime_check(return_vec[g] as i32)
                        {

                            return_vec.remove(g);
                        }
                }
            if return_vec.len() >=3
                {
                    //let difference_vec:Vec<u32>= Vec::new();
                    for g in 0..return_vec.len() -  1
                        {

                            for h in g + 1..return_vec.len()
                                {
                                    let diff = return_vec[h] - return_vec[g];
                                    if diff > 0
                                        {
                                            if return_vec.contains(&(return_vec[h] + diff))
                                                {
                                                    println!("tsrgets={} {} {} ", return_vec[h] - diff, return_vec[h], return_vec[h] + diff)
                                                }
                                        }
                                    }
                        }
                }
        }

}

const EULER_50_CONSTRAINT:usize =1000000;
fn euler50() //Consecutive prime sum
{
     let mut vec:Vec<i32>=Vec::new();
     for f in 2 .. EULER_50_CONSTRAINT
         {
             if prime_check(f as i32)
             {
                 println!("prime = {}",f);
                 vec.push(f as i32);
             }
        }
    let mut longest=0;
    let mut landing_vec=0;
    for f in 0 ..vec.len()
    {
        let mut total =0;
        for g in f .. vec.len()
            {
                total+=vec[g];
                if total > EULER_50_CONSTRAINT as i32
                {
                    total-=vec[g];
                    landing_vec=g;
                    break;
                }
            }
        //println!("total={}",total);
        for g in (f .. landing_vec).rev()
            {
                if g-f <= longest
                {
                    break;
                }
                if prime_check(total)
                {
                    longest=g-f;
                    print!("chain=");
                    for h in f ..g+1
                        {
                            print!("{},",vec[h]);
                        }
                    println!("{}",total);
                    println!("longest={}",longest+1);
                    break;
                }
                total -=vec[g];

            }
    }
}



fn euler51()
{
    let mut done=false;
    let target =8;
    let digits=6;
    let start:u32=10_u32.pow(digits-1);
    let end:u32 =10_u32.pow(digits);
    for f in start .. end
        {
            for h in 0..digits - 2
                {
                    for i in h+1..digits-1
                        {
                            for k in i+1 .. digits
                                {
                                    let mut count = 0;
                                    let mut bad_count = 0;
                                    for g in 0..10
                                        {
                                            let mut f_vec = number_to_vec(f);
                                            f_vec[h as usize] = g;
                                            f_vec[i as usize] = g;
                                            f_vec[k as usize] = g;
                                            //println!("h i k {} {} {}",h,i,k);
                                            if (g == 0) & (k == 5) { continue; }
                                            let mut newnum = 0;
                                            let mut display_string: String = String::new();
                                            for j in (0..digits).rev()
                                                {
                                                    if (j == h) | (j == i) | (j == k)
                                                        {
                                                            display_string += "*";
                                                        } else {
                                                        let newstr = f_vec[j as usize].to_string();
                                                        display_string += &newstr;
                                                    }
                                                    newnum *= 10;
                                                    newnum += f_vec[j as usize]
                                                }
                                            let prime = prime_check(newnum as i32);
                                            if prime
                                                {
                                                    count += 1;
                                                    println!("{} {} {} {} {}", g, f, display_string, newnum, count);
                                                } else {
                                                bad_count += 1;
                                            }
                                            if count >= target {
                                                done = true;
                                                break;
                                            }
                                            if bad_count > (10 - target) { break; }
                                        }
                                    if done {break;}
                                }
                            if done {break;}
                        }
                    if done {break;}
                    }
                if done {break;}
        }

}


fn euler52() //Permuted multiples
{
    for f in 1 .. 1000000
        {
            let mut number_vec=number_to_vec(f);
            let mut length = number_vec.len();
            let mut return_vec:Vec<u32> =Vec::new();
            find_permutations(&mut number_vec,length as u32, &mut return_vec);
            if !return_vec.contains(&(f *2))
            {
                continue
            }
            if !return_vec.contains(&(f *3))
            {
                continue
            }
            if !return_vec.contains(&(f *4))
            {
                continue
            }
            if !return_vec.contains(&(f *5))
            {
                continue
            }
            if !return_vec.contains(&(f *6))
            {
                continue
            }
            println!("answer = {}",f);
            break;


        }
}

fn euler53()
{
    let mut count=0;
    let big_million= 1000000.to_biguint().unwrap();
    for f in 1 .. 100+1
        {
            for g in 1 ..f +1
                {
                    let big_number_f:BigUint=f.to_biguint().unwrap();
                    let big_number_g:BigUint=g.to_biguint().unwrap();
                    let big_number_f_minus_g:BigUint=(f-g).to_biguint().unwrap();
                    let f_factorial:BigUint=big_number_f.factorial();
                    let g_factorial:BigUint=big_number_g.factorial();
                    let f_minus_g_factorial:BigUint=big_number_f_minus_g.factorial();
                    let total:BigUint = f_factorial/(g_factorial * f_minus_g_factorial);
                    if total > big_million
                    {
                        count+=1;
                        println!("count={}",count);
                    }
                }
        }
}



fn flush_check(this_hand:[Card;5]) -> bool
{
    let mut flush:bool=true;
    let suit = this_hand[0].suit;
    for f in 1 .. 5
        {
            if this_hand[f].suit != suit
            {
                flush =false;
            }
        }
    flush
}

fn straight_check(this_hand:[Card;5]) -> bool
{
    let mut straight:bool=true;
    //let mut working_hand = this_hand.clone();
    //working_hand.sort();
    let mut vec:Vec<u32> = Vec::new();
    for f in 0..5
        {
            vec.push(this_hand[f].value)
        }
    vec.sort();
    let start = vec[0];
    for f in 1 .. 5
        {
            if vec[f] != start+f as u32
            {
                straight =false;
            }
        }
    straight
}


fn pair_check(this_hand:[Card;5],exclusion:u32) -> PairValue
    {
        let mut return_value:PairValue = PairValue::new();
        //let mut working_hand = this_hand.clone();
        //let mut count = 0;
        //let mut return_val=0;
        for f in 2..15
            {
                if f != exclusion
                {
                    let mut temp_count=0;
                    for g in 0 .. 5
                        {
                            if this_hand[g].value== f
                                {
                                    temp_count+=1;
                                }
                        }
                    if temp_count>1
                        {
                            return_value.count=temp_count;
                            return_value.value=f;
                            //println!("count {} value {}",return_value.count,return_value.value);
                            break;
                        }
                }
            }
        return_value
    }
fn highest_card(hand:[Card;5]) -> u32
{
    let mut highest =0;
    for f in 0 .. 5
        {
           if hand[f].value> highest
           {
               highest=hand[f].value;
           }
        }
    highest
}
fn compare_hands(hand:Hand) -> u32
{
    let mut winner = 0;
    let hand_value1=hand.player_1_hand_value;
    let hand_value2=hand.player_2_hand_value;
    if (hand_value1.strait & hand_value1.flush) &
        !(hand_value2.strait & hand_value2.flush)
    {
        winner = 1;
    }
    else    if !(hand_value1.strait & hand_value1.flush) &
        (hand_value2.strait & hand_value2.flush)
    {
        winner = 2;
    }
    else if(hand_value1.primary_set_count==4) &
        (hand_value2.primary_set_count==4)
    {
        if hand_value1.primary_set_value > hand_value2.primary_set_value { winner = 1; }
        else if hand_value1.primary_set_value < hand_value2.primary_set_value { winner = 2; }
    }
    else if  hand_value1.primary_set_count==4
        {
            winner = 1;
        }

    else if hand_value2.primary_set_count==4
    {
        winner = 2;
    }
    else if (hand_value1.primary_set_count==3) & (hand_value1.secondary_set_count==2) &
            (hand_value2.primary_set_count==3) & (hand_value2.secondary_set_count==2)
    {
        if hand_value1.primary_set_value > hand_value2.primary_set_value { winner = 1; }
        else  {winner =2}
    }
    else if (hand_value1.primary_set_count==3) & (hand_value1.secondary_set_count==2) &
        !(hand_value2.primary_set_count==3) & (hand_value2.secondary_set_count==2)
    {
        winner = 1;
    }
    else if !(hand_value1.primary_set_count==3) & (hand_value1.secondary_set_count==2) &
        (hand_value2.primary_set_count==3) & (hand_value2.secondary_set_count==2)
    {
        winner = 2;
    }
    else if (hand_value1.flush) & ! (hand_value2.flush) { winner = 1; }
    else if !(hand_value1.flush) &  (hand_value2.flush) { winner = 2; }
    else if (hand_value1.strait) & ! (hand_value2.strait) { winner = 1; }
    else if !(hand_value1.strait) &  (hand_value2.strait) { winner = 2; }
    else if (hand_value1.primary_set_count==3) & (hand_value2.primary_set_count==3)
    {
        if hand_value1.primary_set_value > hand_value2.primary_set_value { winner = 1; }
        else { winner = 2; }
    }
    else if hand_value1.primary_set_count==3 { winner = 1; }
    else if hand_value2.primary_set_count==3 { winner = 2; }
    else if (hand_value1.primary_set_count==2) & (hand_value1.secondary_set_count==2) &
        (hand_value2.primary_set_count==2) & (hand_value2.secondary_set_count==2)
    {
        println!("2 pair on 2 pair");
        if (hand_value1.primary_set_value > hand_value2.primary_set_value) &
            (hand_value1.primary_set_value > hand_value2.secondary_set_value) { winner = 1; }
        else if (hand_value1.secondary_set_value > hand_value2.primary_set_value) &
            (hand_value1.secondary_set_value > hand_value2.secondary_set_value) { winner = 1; }
        else if (hand_value2.primary_set_value > hand_value1.primary_set_value) &
            (hand_value2.primary_set_value > hand_value1.secondary_set_value) { winner = 2; }
        else if (hand_value2.secondary_set_value > hand_value1.primary_set_value) &
            (hand_value2.secondary_set_value > hand_value1.secondary_set_value) { winner = 2; }
    }
    else if (hand_value1.primary_set_count==2) & (hand_value1.secondary_set_count==2) { winner = 1; }
    else if (hand_value2.primary_set_count==2) & (hand_value2.secondary_set_count==2) { winner = 2; }
    else if (hand_value1.primary_set_count==2) & (hand_value2.primary_set_count==2)
    {
      //  println!("Pair match");
        if hand_value1.primary_set_value > hand_value2.primary_set_value  { winner = 1; }
        if hand_value2.primary_set_value > hand_value1.primary_set_value  { winner = 2; }
    }
    else if hand_value1.primary_set_count==2 { winner = 1; }
    else if hand_value2.primary_set_count==2 { winner = 2; }
    if winner ==0
    {
        println!("High card fight");
        if highest_card(hand.player_1) > highest_card(hand.player_2) { winner = 1; } else if highest_card(hand.player_2) > highest_card(hand.player_1) { winner = 2; } else { winner = 0; }
    }

    winner
}

fn display_hand(hand_value:HandValue)
{
    if (hand_value.flush) & (hand_value.strait)
    {
        println!("Straight Flush")
    }
    else if hand_value.primary_set_count ==4
    {
        println!("Four of a Kind")
    }
    else if ((hand_value.primary_set_count ==3) &
        (hand_value.secondary_set_count ==2)) |
        ((hand_value.primary_set_count ==2) &
            (hand_value.secondary_set_count ==3))
    {
        println!("Full House")
    }
    else if hand_value.flush
    {
        println!("Flush")
    }
    else if hand_value.strait
    {
        println!("Straight")
    }
    else if(hand_value.primary_set_count ==3) |
        (hand_value.secondary_set_count ==3)
    {
        println!("three of a kind")
    }
    else if(hand_value.primary_set_count ==2) &
        (hand_value.secondary_set_count ==2)
    {
        println!("two pair")
    }
    else if hand_value.primary_set_count ==2
       // |
        //(hand_value.secondary_set_count ==2)
    {
        println!("pair")
    }
}

fn euler54()
{
        let file = File::open("p054_poker.txt").expect("Bad File");
    let mut win_count =0;
    let mut win_count2 =0;
    let mut hand_count=0;
        for line in BufReader::new(file).lines()
            {
                let line_data=line.unwrap();
                //println!("{}", line.unwrap());
                let mut iter = line_data.split_whitespace();
                let mut count=0;

                let mut hands:Vec<Hand> = Vec::new();
                let mut this_hand:Hand =Hand {player_1:[Card { value:0,suit:SuitEnum::Error};5],player_2:[Card { value:0,suit:SuitEnum::Error};5],
                    player_1_hand_value:HandValue {set:false, flush:false, strait:false, primary_set_count:0, primary_set_value:0,secondary_set_count:0,secondary_set_value:0},
                    player_2_hand_value:HandValue {set:false, flush:false, strait:false, primary_set_count:0, primary_set_value:0,secondary_set_count:0,secondary_set_value:0}
                    };
                for f in iter
                    {
                        //let mut new_card = decode_card(f);
                        if count<5
                        {
                            this_hand.player_1[count]= decode_card(f);
                        }
                        else
                        {
                            this_hand.player_2[count-5]= decode_card(f);
                        }
                        count+=1;
                        if count>=10
                        {
                            hands.push(this_hand);
                            count = 0
                        }
                       // println!("{}",f);
                    }
                for mut iter in hands
                    {
                        hand_count+=1;
                        println!("Hand Number={}",hand_count);
                        iter.player_1_hand_value.flush=flush_check(iter.player_1);
                        iter.player_1_hand_value.strait=straight_check(iter.player_1);
                        let mut pairs=pair_check(iter.player_1,0);
                        iter.player_1_hand_value.primary_set_count=pairs.count;
                        if iter.player_1_hand_value.primary_set_count > 0
                        {
                            iter.player_1_hand_value.primary_set_value = pairs.value;
                            let mut pairs = pair_check(iter.player_1, iter.player_1_hand_value.primary_set_value);
                            iter.player_1_hand_value.secondary_set_count = pairs.count;
                            if iter.player_1_hand_value.secondary_set_count > 0
                            {
                                iter.player_1_hand_value.secondary_set_value = pairs.value;
                            }
                        }
                        if iter.player_1_hand_value.primary_set_count < iter.player_1_hand_value.secondary_set_count
                        {
                            let temp_count=iter.player_1_hand_value.primary_set_count;
                            let temp_value=iter.player_1_hand_value.primary_set_value;
                            iter.player_1_hand_value.primary_set_count = iter.player_1_hand_value.secondary_set_count;
                            iter.player_1_hand_value.primary_set_value = iter.player_1_hand_value.secondary_set_value;
                            iter.player_1_hand_value.secondary_set_count=temp_count;
                            iter.player_1_hand_value.secondary_set_value=temp_value;
                        }
                        println!("Player 1");
                        for f in 0..5
                            {
                               println!("{}",iter.player_1[f]) ;
                            }
                        display_hand(iter.player_1_hand_value);

                        iter.player_2_hand_value.flush=flush_check(iter.player_2);
                        iter.player_2_hand_value.strait=straight_check(iter.player_2);
                        let mut pairs=pair_check(iter.player_2,0);
                        iter.player_2_hand_value.primary_set_count=pairs.count;
                        if iter.player_2_hand_value.primary_set_count > 0
                        {
                            iter.player_2_hand_value.primary_set_value = pairs.value;
                            let mut pairs = pair_check(iter.player_2, iter.player_2_hand_value.primary_set_value);
                            iter.player_2_hand_value.secondary_set_count = pairs.count;
                            if iter.player_2_hand_value.secondary_set_count > 0
                            {
                                iter.player_2_hand_value.secondary_set_value = pairs.value;
                            }
                        }
                        if iter.player_2_hand_value.primary_set_count < iter.player_2_hand_value.secondary_set_count
                        {
                            let temp_count=iter.player_2_hand_value.primary_set_count;
                            let temp_value=iter.player_2_hand_value.primary_set_value;
                            iter.player_2_hand_value.primary_set_count = iter.player_2_hand_value.secondary_set_count;
                            iter.player_2_hand_value.primary_set_value = iter.player_2_hand_value.secondary_set_value;
                            iter.player_2_hand_value.secondary_set_count=temp_count;
                            iter.player_2_hand_value.secondary_set_value=temp_value;
                        }
                       println!("Player 2");
                        for f in 0..5
                            {
                                println!("{}",iter.player_2[f]) ;
                            }
                        display_hand(iter.player_2_hand_value);
                        let winner=compare_hands(iter);
                        if winner == 1
                        {
                            win_count+=1;
                            println!("Player One Wins {}",win_count);
                        }
                        else if winner == 2
                        {
                            win_count2+=1;
                            println!("Player Two Wins {}",win_count2)
                        }
                        else  {println!("ERROR");}
                        println!();
                        println!();
                    }


            }
}
fn euler55() //Lychrel numbers
{
    let mut count = 0;
    for f in 1 .. 10000
        {
            let mut lychrel =true;
            let reverse_num = reverse_digits(&f.to_biguint().unwrap());
            let mut new_number:BigUint =f.to_biguint().unwrap();
            println!("{} {}", f,reverse_num);
            for _g in 1 .. 50
                {
                    //println!("new number {}",new_number.clone());
                    let reverse_num = reverse_digits(&new_number);
                    new_number=new_number+reverse_num;
                    if new_number == reverse_digits(&new_number)
                    {
                        lychrel =  false;
                        break;
                    }
                }
            if lychrel
            {
                count+=1;
                println!("Count {}",count);
            }
        }

}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
enum SuitEnum
{
    Hearts,
    Diamonds,
    Spades,
    Clubs,
    Error,
}

#[derive(Copy, Clone)]
#[derive(Eq)]
struct Card
{
    pub suit: SuitEnum,
    pub value: u32,
}


impl Ord for Card
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Card {
    fn new() -> Self
    {
        Self
        {
            suit: SuitEnum::Error,
            value:0
        }
    }

//    fn sort(&self) -> u32
//    {
//        self.value
//    }
}

impl fmt::Display for Card
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let  number_string:String;
        let  suit_string:String;
        match  self.value
            {
                2...9  => number_string =format!("{} of ",self.value),
                14       => number_string ="Ace of ".to_string(),
                11       => number_string ="Jack of ".to_string(),
                10       => number_string ="Ten of ".to_string(),
                12      => number_string ="Queen of ".to_string(),
                13       => number_string ="King of ".to_string(),
                _         => number_string ="Error ".to_string(),
            }
        match  self.suit
            {
                SuitEnum::Hearts=>     suit_string ="Hearts".to_string(),
                SuitEnum::Diamonds=>        suit_string ="Diamonds".to_string(),
                SuitEnum::Spades=>        suit_string ="Spades".to_string(),
                SuitEnum::Clubs=>        suit_string ="Clubs".to_string(),
                SuitEnum::Error=>        suit_string ="Error".to_string(),
            }
        write!(f, "{}",number_string+&suit_string)
    }
}
#[derive(Copy, Clone)]

struct PairValue
{
    count:u32,
    value:u32,
}

impl PairValue
{
    fn new() -> Self
    {
        Self
            {
                count: 0,
                value: 0,
            }
    }
}
#[derive(Copy, Clone)]
struct HandValue
{
    pub set:bool,
    pub flush:bool,
    pub strait:bool,
    pub primary_set_count:u32,
    pub primary_set_value:u32,
    pub secondary_set_count:u32,
    pub secondary_set_value:u32,
}

#[derive(Copy, Clone)]
struct Hand
{
    pub player_1:[Card;5],
    pub player_1_hand_value: HandValue,
    pub player_2:[Card;5],
    pub player_2_hand_value: HandValue,

}



fn decode_card(card_string:&str) -> Card
{
    let mut this_card:Card = Card{ value:0,suit:SuitEnum::Error};
    let  value:u32;
    let  suit:SuitEnum;
    let  ch_vec:Vec<char>=card_string.chars().collect();
    this_card.suit= SuitEnum::Error;
    this_card.value = 0;
    match ch_vec[0]
        {
            '2'...'9'  => value = ch_vec[0] as u32 -'0' as u32,
            'A'       => value = 14,
            'J'       => value =11,
            'T'       => value = 10,
            'Q'       => value = 12,
            'K'       => value = 13,
            _         => value = 0,
        }
    this_card.value =value;

    match ch_vec[1]
        {
            'H'       => suit = SuitEnum::Hearts,
            'D'       => suit =SuitEnum::Diamonds,
            'S'       => suit = SuitEnum::Spades,
            'C'       => suit = SuitEnum::Clubs,
            _         => suit = SuitEnum::Error,
        }
    this_card.value =value;
    this_card.suit = suit;
    this_card
}
fn euler56()  //Powerful digit sum
{
    let mut highest =0;
  for f in 1 .. 100
      {
          let big_f:BigUint = f.to_biguint().unwrap();
          for g in 1 .. 100
              {
                  //let big_g:BigUint = g.to_biguint().unwrap();
                  let x: BigUint = num_traits::pow(big_f.clone(), g as usize);
                  let vec = big_uint_to_vec(x);
                  let sum =vector_sum_u32(vec);
                  if sum > highest
                  {
                      highest =sum;
                      println!("Highest Sum {} {}",highest,sum);
                  }
              }
      }
}


fn euler57()  //Square root convergents
{
    let mut last_numerator:BigUint =One::one();
    let mut numerator:BigUint =3.to_biguint().unwrap();
    let mut denominator:BigUint =2.to_biguint().unwrap();


    let mut count =0;

    for _f in 1 .. 1000
        {
            let temp=numerator.clone();
            denominator=denominator+numerator.clone();
            numerator=(numerator * 2.to_biguint().unwrap())+last_numerator;
            last_numerator=temp;
            println!("{}/{}   {}",numerator,denominator,count);
            if numerator.to_string().len() != denominator.to_string().len()
                {
                    count+=1;

                }
        }

}


fn euler58()
{
    let mut side_length =1;
    let mut count=1;
    let mut corners = 1;
    let mut prime_corners=0;
    let mut not_done = true;
    while not_done                                  //2 4 6 8
        {
            for _f in 0..4
                {
                    count += side_length * 2;
                    corners += 1;
                    if prime_check(count)
                        {
                            prime_corners += 1;
                        }
                }
            let percent: f64 = prime_corners as f64 / corners as f64;
            if percent < 0.1000
                {
                    not_done =false;
                }

            println!("side length ={} percent = {}",(side_length * 2)+1,percent);
            side_length+=1;
        }

}

fn euler59() //XOR decryption
{
        let contents = fs::read_to_string("p059_cipher.txt")
        .expect("Something went wrong reading the file");
      let mut count:usize=0;
      //let mut total:u32 = 0;
      let the: &'static str ="the";
    let and: &'static str ="and";
    let is: &'static str ="is";
    let in1: &'static str ="in";
    let  vec: Vec<&str> = contents.split(',').collect();
      for f in 97 .. 123
          {
              for g in 97 .. 123
                  {
                      for h in 97 .. 123
                          {
                              let mut code:Vec<u8> = Vec::new();
                              code.push(f);
                              code.push(g);
                              code.push(h);
                              let mut newvec:Vec<u8>= Vec::new();
                              for iter in vec.clone()
                                  {
                                      newvec.push(&iter.parse::<u8>().unwrap() ^ code[count]);
                                      count+=1;
                                      if count >=3 {count=0}
                                  }
                              if newvec.is_ascii()
                                  {
                                      //println!("ascii");
                                      let mut found_and =false;
                                      let mut found_the =false;
                                      let mut found_is =false;
                                      let mut found_in =false;
                                      for i in 0  .. newvec.len() -2
                                          {
                                              if (newvec[i] == the.as_bytes()[0]) & (newvec[i+1] == the.as_bytes()[1]) & (newvec[i+2] == the.as_bytes()[2])
                                              {
                                                  found_the =true;
                                                 // break;
                                              }
                                              if (newvec[i] == and.as_bytes()[0]) & (newvec[i+1] == and.as_bytes()[1]) & (newvec[i+2] == and.as_bytes()[2])
                                              {
                                                  found_and = true;
                                              }
                                              if (newvec[i] == is.as_bytes()[0]) & (newvec[i+1] == is.as_bytes()[1])
                                                  {
                                                      found_is = true;
                                                  }
                                              if (newvec[i] == in1.as_bytes()[0]) & (newvec[i+1] == in1.as_bytes()[1])
                                                  {
                                                      found_in = true;
                                                  }
                                              if found_the & found_and & found_is & found_in
                                                  {
                                                      let mut sum:u32 = 0;
                                                      for i2 in newvec
                                                          {
                                                              sum += i2 as u32;
                                                              print!("{}",i2 as char);
                                                          }
                                                      println!("found and and the {} {} {} {}",f,g,h,sum);
                                                      break;
                                                  }
                                          }
                                  }
                          }
                  }
          }


}

// 7,  1237,  2341,  12409,  18433, round 3 count  1 sum 34427
fn euler60() //Prime pair sets
{
        let mut prime_vec_of_vec:Vec<Vec<u32>> ;
        let mut new_vec_of_vec:Vec<Vec<u32>> =Vec::new();
        let mut prime_vec:Vec<u32> =Vec::new();
        for f in 0..10000                            //Get prime number set
            {
                if prime_check(f)
                {
                    let mut temp_vec: Vec<u32> = Vec::new();
                    temp_vec.push(f as u32);
                    //prime_vec_of_vec.push(temp_vec);
                    new_vec_of_vec.push(temp_vec);
                    prime_vec.push(f as u32);
                }
            }
        //Build pair sets
        prime_vec_of_vec=new_vec_of_vec.clone();
        let mut count = 0; //prime_vec.len();
        for f in 0..prime_vec_of_vec.len()-1
            {
                let mut good_number = true;

                for g in 0..prime_vec.len()
                    {
                        let mut new_vec: Vec<u32> = prime_vec_of_vec[f].clone();
                        let mut good_number = true;
                        for h in 0..prime_vec_of_vec[f].len()
                            {
                                //print!(".");
                                let f_string = prime_vec_of_vec[f][h].to_string();
                                let g_string = prime_vec[g].to_string();
                                let perm1_string = f_string.clone() + &g_string;
                                let perm2_string = g_string + &f_string;
                                let perm1 = perm1_string.parse::<u32>().unwrap();
                                let perm2 = perm2_string.parse::<u32>().unwrap();
                                //println!("{} {}",perm1,perm2);
                                if !prime_check(perm1 as i32)
                                    {
                                        good_number = false;
                                        //break;
                                    } else {
                                    if !prime_check(perm2 as i32)
                                        {
                                            good_number = false;
                                            // break;
                                        }
                                }
                                //println!();
                            }
                        if good_number
                            {
                                count += 1;
                                new_vec.push(prime_vec[g]);
                                    for h in new_vec.clone()
                                        {
                                            print!(" {}, ",h)
                                        }
                                //println!();
                                println!("count  {} sum {}", count, vector_sum_u32(new_vec.clone()));
                                new_vec_of_vec.push(new_vec);
                            }
                    }
                if !good_number
                    {
                        // count=count-1;
                        //  println!("count {}", count);
                    }
            }
            // let mut new_vec_of_vec:Vec<Vec<u32>> =Vec::new();
            let mut vec_of_pairs:Vec<Vec<u32>> = Vec::new(); //new_vec_of_vec.clone();  //prime_vec_of_vec;
            for f in 0.. new_vec_of_vec.len()
                {
                   if new_vec_of_vec[f].len()==2
                   {
                       let mut temp_vec:Vec<u32> = new_vec_of_vec[f].clone();
                       temp_vec.sort();
                       vec_of_pairs.push(temp_vec);
                   }
                }
            vec_of_pairs.sort();
            vec_of_pairs.dedup();
            for f in vec_of_pairs.clone()
                {
                    let test_vec=f;
                    println!("vec of pairs {} {}",test_vec[0],test_vec[1]);

                }
            for i in 2 .. 7
            {
                prime_vec_of_vec=new_vec_of_vec.clone();
                prime_vec_of_vec.sort();
                prime_vec_of_vec.dedup();
                new_vec_of_vec =Vec::new();

                let mut count = 0; //prime_vec.len();
                for f in 0..prime_vec_of_vec.len()-1   //loop over our test vector
                    {
                        let mut good_number = false;

                        for g in 0..prime_vec.len()    // for each of our prime numbers
                            {
                                //if prime_vec_of_vec[f].contains(&prime_vec[g])  //if the prime is in the text vector
                                if prime_vec_of_vec[f].binary_search(&prime_vec[g]).is_ok()  //if the prime is in the text vector

                                    {
                                    continue;
                                }
                                //let mut new_vec:Vec<u32>=prime_vec_of_vec[f].clone();
                                let mut good_number = true;
                                for h in 0..prime_vec_of_vec[f].len()  // for each element in the test vector
                                    {
                                        //print!(".");
                                        let mut temp_vec:Vec<u32> = Vec::new();  // create a new pairing
                                        temp_vec.push(prime_vec[g]);
                                        temp_vec.push(prime_vec_of_vec[f][h]);
                                        temp_vec.sort();                         // sort the new pairing
                                        let mut found_match=false;
                                        //if vec_of_pairs.contains(&temp_vec)
                                        if vec_of_pairs.binary_search(&temp_vec).is_ok()
                                        {
                                            found_match=true;
                                        }
//                                        for j in 0 .. vec_of_pairs.len()
//                                        {
//                                            let test_vec= &vec_of_pairs[j];
//                                            if (temp_vec[0] == test_vec[0]) & (temp_vec[1] == test_vec[1])
//                                                {
//                                                    found_match=true;
//                                                    break;
//                                                }
//                                        }
                                        if !found_match
                                        {
                                            //println!("eleminated {} {}",temp_vec[0],temp_vec[1]);
                                            good_number = false;
                                            break;
                                        }
                                        //println!();
                                    }
                                if good_number
                                {
                                    let mut new_vec:Vec<u32>=prime_vec_of_vec[f].clone();
                                    count += 1;
                                    new_vec.push(prime_vec[g]);
                                   new_vec.sort();
                                    for h in new_vec.clone()
                                        {
                                            print!(" {}, ",h)
                                        }
                                    println!();
                                    println!("round {} count  {} sum {}", i,count,vector_sum_u32(new_vec.clone()));
                                    new_vec_of_vec.push(new_vec);




                                }
                            }
                        if !good_number
                        {
                            // count=count-1;
                            //  println!("count {}", count);
                        }
                    }
            }
}



//fn euler60() //Prime pair sets
//{
//    let mut prime_vec:Vec<u32> =Vec::new();
//    for f in 0..10000
//        {
//            if prime_check(f)
//                {
//                    prime_vec.push(f);
//                }
//        }
//    let mut Test_vec:Vec<u32> =Vec::new();
//    for f in prime_vec
//        {
//            let f_string =f.to_string();
//            if Test_vec.len() ==0
//                {
//                    Test_vec.push(f)
//
//                }
//            else
//            {
//                good_number=true;
//                for g in Test_vec
//                    {
//                        let g_string = g.to_string();
//                        let perm1_string = f_string + g_string;
//                        let perm2_string = g_string + f_string;
//                        let perm1 = perm1_string.parse::<u32>().unwrap();
//                        let perm2 = perm2_string.parse::<u32>().unwrap();
//                        if !prime_check(perm1) | !prime_check(perm2)
//                            {
//                                good_number = false;
//                            }
//                    }
//                if(good_number)
//                    {
//                        Test_vec.push(f);
//                    }
//            }
//
//        }
//
//}
struct Possibles
{
    next_numbers:Vec<u32>,
    target_number:u32,
}

fn euler61()
{
    let mut working_queue:VecDeque<Vec<u32>> = VecDeque::new();
    let mut possible_vec:Vec<u32> = Vec::new();
    for f in 1  .. 10000
        {
             //(n+1)/2
             let x=f * (f+1) /2;
            if (x > 999) & (x < 10000)
                 {
                     if !possible_vec.contains(&x) { possible_vec.push(x) }
                 }
            //(x*x
            let x=f * f;
            if (x > 999) & (x < 10000)
                {
                    if !possible_vec.contains(&x) { possible_vec.push(x) }
                }
            //(n(3n−1)/2
            let x=f * (3 * f - 1) /2;
            if (x > 999) & (x < 10000)
                {
                    if !possible_vec.contains(&x) { possible_vec.push(x) }
                }
            //(n(2n−1)
            let x=f * (2 * f - 1);
            if (x > 999) & (x < 10000)
                {
                    if !possible_vec.contains(&x) { possible_vec.push(x) }
                }
            //n(5n−3)/2
            let x=f * (5 * f - 3) /2;
            if (x > 999) & (x < 10000)
                {
                    if !possible_vec.contains(&x) { possible_vec.push(x) }
                }
            //n(3n−2)
            let x=f * (3 * f - 2);
            if (x > 999) & (x < 10000)
                {
                    if !possible_vec.contains(&x) { possible_vec.push(x) }
                }

        }
    println!("possible vec size={}",possible_vec.len());
    for f in possible_vec.clone()
        {
            let mut temp_vec:Vec<u32> = Vec::new();
            temp_vec.push(f);
            working_queue.push_back(temp_vec);
        }
    while !working_queue.is_empty()
        {
            //println!("queue length={}",working_queue.len());
            let mut test_vec:Vec<u32> = working_queue.pop_front().unwrap();
            let last_element = test_vec[test_vec.len()-1];
            //println!("last element {}",last_element);
            print_vec(&test_vec);
            if test_vec.len() <6
                {
                    for g in 10 .. 100
                        {
                           // let last_element = test_vec[test_vec.len()-1];
                           // println!("last element {}",last_element);
                            let mut working_vec = test_vec.clone();
                            let test_number = ((last_element %100) *100) +g;
                            if (possible_vec.contains(&test_number)) & (!test_vec.contains(&test_number))
                                {
                                    working_vec.push(test_number);
                                    working_queue.push_back(working_vec);
                                }

                        }
                }
            else
            {
                if test_vec.first().unwrap()/100 == test_vec.last().unwrap()%100
                    {
                        println!("Possible Match............................................................");
                        let save_vec =test_vec.clone();
                        for h in 0 .. test_vec.len()
                            {
                                if is_octagon(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Octagon");
                                        break;
                                    }
                            }
                        for h in 0 .. test_vec.len()
                            {
                                if is_heptagon(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Heptagon");
                                        break;
                                    }
                            }
                        for h in 0 .. test_vec.len()
                            {
                                if is_hexagon(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Hexagon");
                                        break;
                                    }
                            }
                        for h in 0 .. test_vec.len()
                            {
                                if is_pentagon(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Pentagon");
                                        break;
                                    }
                            }
                        for h in 0 .. test_vec.len()
                            {
                                if is_square(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Square");
                                        break;
                                    }
                            }
                        for h in 0 .. test_vec.len()
                            {
                                if is_triangle(test_vec[h])
                                    {
                                        test_vec.remove(h) ;
                                        println!("Found Triangle");
                                        break;
                                    }
                            }
                        if test_vec.len() ==0
                            {
                                println!("sum = {}",vector_sum_u32(save_vec));
                                break;
                            }


                    }
            }

        }

}

//fn euler62()  // Cubic permutations
//{
//    let mut done=false;
//    for f in 3 .. 18
//        {
//            let mut vec_of_cubes:Vec<u64> = Vec::new();
//            for g in 1 .. 10000
//                {
//                    let the_cube: u64 = g * g * g;
//                    if the_cube.digits() == f
//                        {
//                            vec_of_cubes.push(the_cube);
//                        }
//                        else if the_cube.digits() > f
//                        {
//                            break;
//                        }
//                }
//            let target_size =5;
//            for h in 0 .. vec_of_cubes.len()
//                {
//                    //println!("{}",vec_of_cubes[h]);
//                    let mut target_vec:Vec<u64> = Vec::new();
//                    for i in 0 .. vec_of_cubes.len()
//                        {
//                           if is_permutation(vec_of_cubes[h] as u64,vec_of_cubes[i] as u64)
//                           {
//                               print!("{} ",vec_of_cubes[i]);
//                               target_vec.push(vec_of_cubes[i]) ;
//                           }
//                        }
//                    println!();
//                    if target_vec.len() == target_size
//                    {
//                       println!("found it  ");
//                        print_vec_u64(&target_vec);
//                        done=true;
//                        break;
//                    }
//                }
//            if done
//            {
//                break;
//            }
//        }
//}
//fn euler63() //Powerful digit counts
//{
//    for f in 0 .. 100
//        {
//            for g in 0.. 100
//                {
//                    let testnum:BigUint =g .to_biguint().unwrap();
//                    //let big_f = f.to_biguint().unwrap();
//                    let testnum2= num_traits::pow(testnum, f as usize);
//                    //let x: BigUint = num_traits::pow(big_f.clone(), g as usize);
//                    let digits = testnum2.digits();
//
//                    if digits == f as usize
//                    {
//                        println!("f,g,number,digits {} {} {} {}",f,g,testnum2,digits);
//                        //break;
//                    }
//                }
//        }
//}

fn continuing_fraction_for_square_root(number:u32) -> Vec<u32>
{
    //let mut count =0;
    let mut return_vec:Vec<u32> = Vec::new();
            let float_f = number as f64;
            let square_root_f = float_f.sqrt();
            println!("number {} square root {}",number,square_root_f);
            if square_root_f.fract() == 0.0 { return return_vec}
            //let fraction = square_root_f.fract();  // get rid of integer portion
            let mut a:i32 = square_root_f.trunc() as i32;  // get rid of integer portion
            let original_a =a;
            let mut m = 0;
            let mut d =1;
            //let mut target_vector: Vec<u64> = Vec::new();
            //let fractional_vector: Vec<u64> = Vec::new();
            //let mut period = 0;
            return_vec.push(original_a as u32);
            loop
                {

                    m = d * a - m;
                    d = (number as i32  - m * m) / d;
                    let a_float:f64 = (original_a as f64 + m as f64 ) / d as f64;
                    a = a_float.trunc() as i32;
                    //println!("a,d,m {} {} {}",a,d,m);
                   // println!("a {}",a);
                    return_vec.push(a as u32);
                    //period+=1;
                    if a  == original_a *2 {break}

                }
        return_vec
}


fn euler64()  //Odd period square roots
{
    //let  count =0;
    let mut odd_periods =0;
    for f in  2 ..= 4
        {
            let float_f = f as f64;
            let square_root_f = float_f.sqrt();
            println!("number {} square root {}",f,square_root_f);
            if square_root_f.fract() == 0.0 {continue}
            //let mut fraction = square_root_f.fract();  // get rid of integer portion
            let mut a:i32 = square_root_f.trunc() as i32;  // get rid of integer portion
            let original_a =a;
            let mut m = 0;
            let mut d =1;
            //let mut target_vector: Vec<u64> = Vec::new();
            //let mut fractional_vector: Vec<u64> = Vec::new();
            let mut period = 0;
            loop
                {

                    m = d * a - m;
                    d = (f - m * m) / d;
                    let a_float:f64 = (original_a as f64 + m as f64 ) / d as f64;
                    a = a_float.trunc() as i32;
                  //  println!("a,d,m {} {} {}",a,d,m);
                   // println!("a {}",a);
                    period+=1;
                    if a  == original_a *2 {break}

                }
            println!("Period ={}",period);
            if period %2 ==1 { odd_periods+=1;}
            println!("odd_periods ={}",odd_periods);
//                print_vec_u64(&target_vector);
//                if target_vector.len() % 2 ==1
//                {
//                    count+=1;
//
//                }
//                println!("count={}",count);

        }
}

fn euler65()
{
    let mut count = 1;
    let mut continuing_vector:Vec<BigUint> = Vec::new();
    let mut n:BigUint=BigUint::one();
    while count<=200
        {
            continuing_vector.push(BigUint::one());
            count+=1;
            continuing_vector.push(BigUint::one());
            count+=1;
            continuing_vector.push(n.clone() *  2.to_biguint().unwrap());
            count+=1;
            n+=1.to_biguint().unwrap();
        }
    //print_vec_u64(&continuing_vector);
    //let mut numinator_vector:Vec<u32> = Vec::new();
    let mut num_2:BigUint =2.to_biguint().unwrap();
    let mut num_1:BigUint =3.to_biguint().unwrap();
    count =3;
    while count <= 100
        {
            let num = num_2 + num_1.clone() * continuing_vector[count-1].to_biguint().unwrap();
            num_2 =num_1.clone();
            num_1 =num.clone();
            let mut temp = num.clone();
            let mut total:BigUint = 0.to_biguint().unwrap();
            while temp!= 0.to_biguint().unwrap()
                {
                    total+=temp.clone() % 10.to_biguint().unwrap();
                    temp/=10.to_biguint().unwrap();
                }
            println!("Count, Numerator {} {} {}",count,num,total);
            count+=1;
        }

}

fn euler66()
{

    let mut open_vectors=999;
    let sieve = primal::Sieve::new(1_000_000);
    let mut cache_queue:VecDeque<Vec<(usize,usize)>> = VecDeque::with_capacity(20);
    let mut final_vec = vec![0; 1001];
    let mut y_squared:Vec<BigUint> = Vec::with_capacity(20);
    let mut prime_factor_cache:Vec<Vec<(usize,usize)>>= Vec::with_capacity(20);
    //let sprimes = prime::prime_sieve(MAX_SMALL_NUM);
    for f in 1 .. 1000000
        {
            y_squared.push(f.to_biguint().unwrap() *f.to_biguint().unwrap() );
        }
    let test_vector:Vec<(usize,usize)> =Vec::with_capacity(20);
    prime_factor_cache.push(test_vector);
    let test_vector:Vec<(usize,usize)> =Vec::with_capacity(20);
    prime_factor_cache.push(test_vector);
    let test_vector:Vec<(usize,usize)> =Vec::with_capacity(20);
    cache_queue.push_back(test_vector);
    let test_vector:Vec<(usize,usize)> =Vec::with_capacity(20);
    cache_queue.push_back(test_vector);
    for f in 2 .. 1001
        {
            println!("{}",f);

            match sieve.factor(f as usize)
                {
                    Ok(number) =>
                        {
                            let test_vec=number;
                            prime_factor_cache.push(test_vec);
                        },
                    Err(_x) => println!("Cannot factor {}.", f),

                }
        }
    //let mut highest_x=0; //:BigUint =BigUint::zero();
    //let mut highest_D=0; //:BigUint =BigUint::zero();
    //let mut good_number = true;
    //println!("D = {}",f);
    //let new_x:u64=0; //BigUint =BigUint::zero();
    //let new_D:u64=0; //:BigUint =BigUint::zero();
    let mut divisor;
    for f in 2 .. 1001
        {
            if y_squared.binary_search(&f.to_biguint().unwrap()).is_ok()
            {
                final_vec[f] = 1;
                println!("square={}",f);
                open_vectors -= 1;
            }
        }
    let mut add_value =1;
    let mut offset = 0;
    let mut g:u64 =1;
    //let  D_factors:Vec<(usize,usize)> =Vec::with_capacity(20);
    let now = Instant::now();
    while  final_vec.len() >1
        {
            g= 1 +offset;
            print!("Remaining values of D ");
            for f in 0 .. final_vec.len()
                {

                    if final_vec[f]==0
                    {
                        print!("{} ",f);
                    }
                }
            println!();

            for h in 2..1001
                {
                    add_value =1;
                    //println!("Testing D={}",h);
                    if final_vec[h] != 0 { continue}
                    println!("Testing D={}",h);
                    g= 1 +offset;
                    let mut prime_number=false;
                    if prime_factor_cache[h].len()==1
                    {
                        prime_number=true;
                    }
                    while g<100_000_000 + offset
                        {
                            g += add_value;
                            if g % 1_000_000 == 0
                            {
                                println!("g={} {} {}", g, open_vectors, now.elapsed().as_secs());
                            }
                            //if (g%2 == 0) & (h%2 == 0) {continue}
                            //if (g%3 == 0) & (h%3 == 0) {continue}


                            //let mut divisor: u64 = f as u64;
                            let mut good_number ;
                            let mut x_minus_1 = g - 1;
                            let mut x_plus_1 = g + 1;
                            divisor = h;
                            add_value =1;
                            if prime_number
                            {
                                if (g - 1) % h as u64 == 0
                                {
                                    x_minus_1 /= h as u64;
                                    divisor = 1;
                                    add_value=h as u64 -2;;
                                } else if (g + 1) % h as u64 == 0
                                {
                                    x_plus_1 /= h as u64;
                                    divisor = 1;
                                    add_value=2
                                }
                            }
                            else
                            {
                                let my_vec = prime_factor_cache[h].clone();
                                let (factor,_exponent) = my_vec.last().unwrap();
                                if (g - 1) % *factor as u64 == 0
                                {
                                    add_value=*factor as u64 -2;
                                }
                                else if (g + 1) % *factor as u64 == 0
                                {
                                    add_value=2
                                }
                            }

                            //println!("Before {}",x_over_D);
                            let mut x_minus_1_factors: Vec<(usize, usize)> = Vec::with_capacity(20);
                            //let my_vector:Vec<(usize,usize)> =cache_queue.pop_front().unwrap();
                            //if !my_vector.is_empty()
//            {
//                x_minus_1_factors=my_vector;
//                //println!("cache hit");
//            }
//            else
//            {
                            match sieve.factor(x_minus_1 as usize)
                                {
                                    Ok(number) => x_minus_1_factors = number,
                                    Err(_x) => println!("Cannot factor {}.", x_minus_1),
                                }

                            let mut x_plus_1_factors: Vec<(usize, usize)> = Vec::with_capacity(20);
                            match sieve.factor(x_plus_1 as usize)
                                {
                                    Ok(number) => x_plus_1_factors = number,
                                    Err(_x) => println!("Cannot factor {}.", x_plus_1),
                                }


                            //let mut number_factors =
                            factor_multiply(&mut x_minus_1_factors, &x_plus_1_factors);
                            //cache_queue.push_back(x_plus_1_factors);
//                    print!("x-1 * x+1 {} ",(g-1)*(g+1));
//                    for h in 0 .. number_factors.len()
//                        {
//                            let (factor,exponent) = number_factors[h];
//                            print!("({} {})",factor,exponent);
//                        }
//                    println!();
                            let mut final_factors: Vec<(usize, usize)> = Vec::with_capacity(20);
                            good_number = true;

                            // D_factors = prime_factor_cache[h]
                            if divisor == 1
                            {
                                final_factors=x_minus_1_factors;
                            }
                            else
                            {
                                x_minus_1_factors.sort();
                                match factor_divide(&x_minus_1_factors, &prime_factor_cache[h]/*D_factors*/)
                                    {
                                        Some(number) => final_factors = number,
                                        None => good_number = false,
                                    }
                            }
                        //                    print!("division result ");
                        //                    for h in 0 .. final_factors.len()
                        //                        {
                        //                            let (factor,exponent) = final_factors[h];
                        //                            print!("({} {})",factor,exponent);
                        //                        }
                        //                    println!();
                        if good_number
                        {
                            for h in final_factors.iter()
                                {
                                    let (_factor, exponent) = h;
                                    if exponent % 2 != 0
                                    {
                                        good_number = false;
                                        break;

                                    }
                                }
                        }
                        if good_number
                        {
                            open_vectors -= 1;
                            println!("D={} X={}", h, g);
                            println!("Open Vectors={}", open_vectors);
                            final_vec[h] = g;
                            //new_x = g; //.to_biguint().unwrap();
                            //new_D = f as u64; //.to_biguint().unwrap();
                            //println!("D {} X {}  Highest D {} Highest X {}",new_D,new_x,highest_D,highest_x);
                            //
                            break;
                        }
                    }

                }
            offset+=100_000_000;
        }

//    if new_x > highest_x
//    {
//        highest_x= new_x;
//        highest_D = new_D;
//    }
//    println!("D {} X {}  Highest D {} Highest X {}",new_D,new_x,highest_D,highest_x);


}



fn euler66_2()  //Odd period square roots
{
    let mut y_squared: Vec<BigUint> = Vec::with_capacity(20);
    for f in 1..1000000
        {
            y_squared.push(f.to_biguint().unwrap() * f.to_biguint().unwrap());
        }
    let mut highest_dem =BigUint::zero();
    let mut highest_dem_index  =BigUint::zero();
    for f in 2..=662
        {
            if y_squared.binary_search(&f.to_biguint().unwrap()).is_ok()
            {
                println!("square={}", f);
                continue;
            }
            let mut period_vector = continuing_fraction_for_square_root(f);
            print_vec(&period_vector);
            let mut num_2 = BigUint::one() ; //period_vector[0];
            let mut num_1 = period_vector[0].to_biguint().unwrap();;
            let mut dem_2 = BigUint::zero();
            let mut dem_1 = BigUint::one();
            period_vector.remove(0);
            println!("Vector Length={}",period_vector.len());
            let mut  period_length = period_vector.len();
            if period_length % 2 ==1
            {
                let mut x =period_vector.clone();
                period_vector.extend(x);
                period_length*=2;

            }
            for g in 1..period_length
                {
                    let index = g ; //% period_vector.len();

                    let num = num_2.clone() + num_1.clone() * period_vector[index - 1];
                    let dem = dem_2.clone() + dem_1.clone() * period_vector[index - 1];
                    num_2 = num_1.clone();
                    num_1 = num.clone();
                    dem_2 = dem_1.clone();
                    dem_1 = dem.clone();
                    println!("{}/{}", num, dem);
                    if dem > highest_dem
                    {
                        highest_dem =dem;
                        highest_dem_index =f.to_biguint().unwrap();
                    }

                }
            println!("highest x = {} {}",highest_dem,highest_dem_index);

        }
}
//                print_vec_u64(&target_vector);
//                if target_vector.len() % 2 ==1
//                {
//                    count+=1;
//
//                }
//                println!("count={}",count);





const EULER_67_NUMBER_OF_LINES:usize =100;
fn euler67()
{
    //let mut number_of_cells_on_this_line =1;
    let mut test_array:[[i32; EULER_67_NUMBER_OF_LINES]; EULER_67_NUMBER_OF_LINES] = [[0; EULER_67_NUMBER_OF_LINES]; EULER_67_NUMBER_OF_LINES];
    let mut current_position = 0;
    let mut current_line_number = 0;
    let contents = fs::read_to_string("p067_triangle.txt")
        .expect("Something went wrong reading the file");
//    let mut count:i32=0;
//    let mut total:i32 = 0;
//    let mut vec: Vec<&str> = contents.split(',').collect();


    let  iter = contents.split_whitespace();

    for f in iter
        {

            //   println!("number = {}", f);
            test_array[current_position][current_line_number]=f.parse::<i32>().unwrap();
            current_position+=1;
            if current_position>current_line_number
            {
                current_position = 0;
                current_line_number +=1;
            }
        }
    for f in (0 .. EULER_67_NUMBER_OF_LINES-1).rev()
        {
            for g in 0 .. f+ 1
                {
                    if test_array[g][f]+test_array[g][f+1] > test_array[g][f]+test_array[g+1][f+1]
                    {
                        test_array[g][f]= test_array[g][f]+test_array[g][f+1];
                    }
                    else
                    {
                        test_array[g][f]= test_array[g][f]+test_array[g+1][f+1];
                    }
                }
            for g in 0 .. f + 1
                {
                    println!("number = {}", test_array[g][f]);
                }
            println!("break")
        }

}





fn euler68() //Magic 5-gon ring
{
  // let InnerCicleVector = find_permutations()
  let mut inner_circle_vector:Vec<Vec<u32>> = Vec::new();
    let mut greatest_final_number=0;
  for f1 in 1 .. 10
      {
          let mut test_vector:Vec<u32> = Vec::new();
          test_vector.push(f1);
          for f2 in 1.. 10
              {
                  if test_vector.contains(&f2)
                  {
                      continue;
                  }
                  let mut tv2 = test_vector.clone();
                  tv2.push(f2);
                  for f3 in 1.. 10
                      {
                              if tv2.contains(&f3)
                              {
                                  continue;
                              }
                              let mut tv3 = tv2.clone();
                              tv3.push(f3);
                              for f4 in 1 .. 10
                          {
                              if tv3.contains(&f4)
                                  {
                                      continue;
                                  }
                              let mut tv4 = tv3.clone();
                              tv4.push(f4);
                              for f5 in 1 .. 10
                                  {
                                      if tv4.contains(&f5)
                                          {
                                              continue;
                                          }
                                      let mut tv5 = tv4.clone();
                                      tv5.push(f5);
                                      inner_circle_vector.push(tv5);
                                  }
                          }
                      }


              }
      }
      println!("Inner Circle");
      for f in inner_circle_vector.iter()
          {
              print_vec(&f);
          }
      for f in inner_circle_vector.iter()
          {
              for f1 in 1 ..= 10
                  {
                      if f.contains(&f1) {continue}
                      for f2 in 1 ..= 10
                          {
                              if f.contains(&f2) {continue}
                              if f2 == f1 {continue;}
                              for f3 in 1 ..= 10
                                  {
                                      if f.contains(&f3) {continue}
                                      if f3 == f1 {continue;}
                                      if f3 == f2 {continue;}
                                      for f4 in 1 ..= 10
                                          {
                                              if f.contains(&f4) {continue}
                                              if f4 == f1 {continue;}
                                              if f4 == f2 {continue;}
                                              if f4 == f3 {continue;}
                                              for f5 in 1 ..= 10
                                                  {
                                                      if f.contains(&f5) {continue}
                                                      if f5 == f1 {continue;}
                                                      if f5 == f2 {continue;}
                                                      if f5 == f3 {continue;}
                                                      if f5 == f4 {continue;}
                                                      let mut test_vec:VecDeque<u32> = VecDeque::new();
                                                      test_vec.push_back(f1);
                                                      test_vec.push_back(f2);
                                                      test_vec.push_back(f3);
                                                      test_vec.push_back(f4);
                                                      test_vec.push_back(f5);
                                                      for _g in 0 ..5
                                                          {
                                                              let t1 =f[0]+f[1]+test_vec[0];
                                                              if t1 != f[1]+f[2]+test_vec[1] {continue}
                                                              if t1 != f[2]+f[3]+test_vec[2] {continue}
                                                              if t1 != f[3]+f[4]+test_vec[3] {continue}
                                                              if t1 != f[4]+f[0]+test_vec[4] {continue}
                                                              //println!("Possible");
                                                              print_vec(&f);
                                                              for h in 0 .. 5
                                                                  {
                                                                      print!("{}",test_vec[h]);
                                                                  }
                                                              print!("    ");
                                                              let mut test_number_vector:Vec<u32> = Vec::new();
                                                              test_number_vector.push(test_vec[0]);
                                                              test_number_vector.push(f[0]);
                                                              test_number_vector.push(f[1]);
                                                              test_number_vector.push(test_vec[1]);
                                                              test_number_vector.push(f[1]);
                                                              test_number_vector.push(f[2]);
                                                              test_number_vector.push(test_vec[2]);
                                                              test_number_vector.push(f[2]);
                                                              test_number_vector.push(f[3]);
                                                              test_number_vector.push(test_vec[3]);
                                                              test_number_vector.push(f[3]);
                                                              test_number_vector.push(f[4]);
                                                              test_number_vector.push(test_vec[4]);
                                                              test_number_vector.push(f[4]);
                                                              test_number_vector.push(f[0]);
                                                              let mut rotate_value =0;
                                                              let mut smallest_test_number:u32=99;
                                                              for h in 0 .. 5
                                                                  {
                                                                      if test_vec[h] < smallest_test_number
                                                                      {
                                                                          rotate_value=h;
                                                                          smallest_test_number = test_vec[h];
                                                                      }
                                                                  }
                                                              test_number_vector.rotate_left(rotate_value*3);
                                                              print_vec(&test_number_vector);
                                                              let mut final_number:u64=0;
                                                              for h in test_number_vector.iter()
                                                                  {
                                                                      if *h<10
                                                                      {
                                                                          final_number*=10;
                                                                          final_number+=(*h) as u64;
                                                                      }
                                                                      else
                                                                      {
                                                                          final_number*=100;
                                                                          final_number+=10;
                                                                      }
                                                                  }
                                                              if final_number > greatest_final_number
                                                              {
                                                                  greatest_final_number=final_number;
                                                                 // println!("final  {} {}",final_number,greatest_final_number);
                                                              }
                                                              println!("final  {} {}",final_number,greatest_final_number);




                                                          }

                                                  }



                                          }

                                  }
                          }
                  }
          }
}
const EULER_69_TARGET: usize =1_000_000;
fn euler69_2()
{

    let sieve = primal::Sieve::new(1_000_000);
    let mut total_vec:Vec<u32>=Vec::new();
    let  _phi_vec:Vec<f64>=Vec::new();
    let mut highest:f64=0.00;
    let mut highest_index =0;
    total_vec.push(0);
    for f in 1 ..=EULER_69_TARGET
        {
            total_vec.push(f as u32 -1);
            // phi_vec.push(0.0);
        }
    let mut prime_vec:Vec<u32>= Vec::new();
    prime_vec.push(0);
    for f in 1 .. 2000
        {
            prime_vec.push(sieve.nth_prime(f) as u32) ;
        }
    for f in 2 ..= EULER_69_TARGET
        {
            if f % 1000 ==0
            {
                println!("f={}", f);
            }

            let f_float = f as f64;
            let fsqrt = f_float.sqrt();
            let fsqrt_int = fsqrt+1.0;

            let mut used_array:Vec<u32> = vec![0; f];
            for g in 1 .. f {used_array[g]=1;}
            let mut total:u32 =f as u32 -1;
            for g in 1 ..= 1000
                {
                    //let prime:u32 = sieve.nth_prime(g) as u32;
                    let prime = prime_vec[g];
                    if prime > f as u32 /2 {break}
                    if f as u32 % prime ==0
                    {
                        let val=f as u32;
                        let mut temp:u32 = prime;
                        while temp < f as u32
                            {
                                if used_array[temp as usize]==1
                                {
                                    used_array[temp as usize] = 0;
                                    total -= 1;
                                }
                                //   print!("{} ",temp);
                                temp+=prime;
                            }

                        //  println!();
                        //println!("{} {}",prime,temp);
                        // total_vec[f]-=temp;
                    }

                }
            // print_vec(&used_array.to_vec());
            let temp:u32 = total ; //used_array.into_iter().sum();
            // assert_eq!(temp,total);
            let phi = f as f64 / temp as f64;

            if (phi > highest) & (phi!=0.0)
            {
                highest= phi;
                highest_index=f;
                println!("{} {} {} {}",temp,phi,highest,highest_index);
            }
            if f% 1000 ==0
            {
                println!("{} {} {} {}", temp, phi, highest, highest_index);
            }

        }
//    //print_vec(&total_vec);
//    for (index,f) in total_vec.iter().enumerate()
//        {
//            if *f != 0
//                {
//                    let temp = index as f64 / *f as f64;
//                    phi_vec.push(temp);
//                } else
//            {
//                phi_vec.push(0.0);
//            }
//        }
    // print!("Vector :");

//    for (index,f) in phi_vec.iter().enumerate()
//        {
//           // print!("{} ",*f);
//            if (*f< lowest) & (*f!=0.0)
//                {
//                    lowest= *f;
//                    lowest_index=index;
//                }
//        }
//    //println!();
//    println!("lowest={} {}",lowest,lowest_index);



}

fn euler69()
{
    let sieve = primal::Sieve::new(1_000_000);
    let mut factor_cache:Vec<Vec<u64>> = Vec::new();
    for f in 0 .. 1_000_000
        {
            //if f<2 {continue}
            print!("{}  ",f);
            //let mut this_factor:Vec<usize> = Vec::new();
            let mut this_factor=find_divisors(f);
            this_factor.push(f);
            this_factor.remove(0);
            print_vec_u64(&this_factor);
            factor_cache.push(this_factor);
        }
    let mut highest:f64 = 0.0;
    let mut highest_n =0;
    for f in 2 .. 1_000_000
        {

            print!("n={}:",f );
            if sieve.is_prime(f) {continue}
            if f/factor_cache[f].len() < 20 as usize {continue}
            let mut count =0;
            for g in 1 .. f
                {
                    let mut factors = factor_cache[f].clone();
                    //println!("g={} ",g);
                    let mut factors2=factor_cache[g].clone();
                    factors.append(&mut factors2);
                    factors.sort();
                    //print_vec_u64(&factors);
                    let before_length= factors.len();
                    factors.dedup();
                    //print_vec_u64(&factors);
                    let after_length = factors.len();
                    if before_length == after_length
                    {
                        count+=1;
                     //   print!("{} ",g);
                    }
                    if (f as f64)/ (count as f64)  < highest {break}


                }
            //println!();

            let current_phi:f64 = f as f64 /  count as f64;
            if current_phi > highest
                {
                    highest = current_phi;
                    highest_n = f;
                }
            println!("current, highest {} {} {} {} ", current_phi, highest, highest_n,count);      }
}

//const EULER_70_TARGET: usize =10_000_000;
//fn euler70()
//{
//
//    let sieve = primal::Sieve::new(1_000_000);
//    let mut total_vec:Vec<u32>=Vec::new();
//    let phi_vec:Vec<f64>=Vec::new();
//    let mut lowest:f64=1000000000000.00;
//    let mut lowest_index =0;
//    total_vec.push(0);
//    for f in 1 ..=EULER_70_TARGET
//        {
//            total_vec.push(f as u32 -1);
//            // phi_vec.push(0.0);
//        }
//    let mut prime_vec:Vec<u32>= Vec::new();
//    prime_vec.push(0);
//    for f in 1 .. 100000
//        {
//            prime_vec.push(sieve.nth_prime(f) as u32) ;
//        }
//
//    for f in 2 ..= EULER_70_TARGET
//        {
//            let mut bad_number = false;
//            if f % 10_000 ==0
//                {
//                    println!("f={}", f);
//                }
//            for g in prime_vec.iter()
//                {
//                   // println!("g={}", g);
//                    if *g == 0 {continue}
//                    if *g > 1000
//                    {
//                        break;
//                    }
//                    if f % *g as usize == 0
//                    {
//                        bad_number = true;
//                        break;
//                    }
//                }
//            if bad_number == true {continue}
//
//            let mut used_array:Vec<u32> = vec![0; f];
//            for g in 1 .. f {used_array[g]=1;}
//            let mut total:u32 =f as u32 -1;
//            let mut floor =1;
//            while floor * 10 < f
//                {
//                    floor *=10;
//                }
//
//            for g in 1 ..= 10000
//                {
//                    //let prime:u32 = sieve.nth_prime(g) as u32;
//                    let prime = prime_vec[g];
//                    if prime > f as u32 /2 {break}
//                    if f as u32 % prime ==0
//                    {
//                        let val=f as u32;
//                        let mut temp:u32 = prime;
//                        while temp < f as u32
//                            {
//                                if used_array[temp as usize]==1
//                                {
//                                    used_array[temp as usize] = 0;
//                                    total -= 1;
//                                }
//                                //   print!("{} ",temp);
//                                temp+=prime;
//                            }
//                        if f as f64 / total as f64 > lowest
//                        {
//                            bad_number = true;
//                            break;
//                        }
//                        if total < floor as u32
//                            {
//                                bad_number = true;
//                                break;
//                            }
//
//                        //  println!();
//                        //println!("{} {}",prime,temp);
//                        // total_vec[f]-=temp;
//                    }
//
//                }
//            if f% 100_000 ==0
//            {
//                println!("New Lowest {} {} {}", f, lowest, lowest_index);
//            }
//            if bad_number {continue}
//            // print_vec(&used_array.to_vec());
//            let temp:u32 = total ; //used_array.into_iter().sum();
//            // assert_eq!(temp,total);
//
//            if is_permutation(f as u64,total as u64)
//                {
//                    println!("perm number {}",f);
//                }
//            else { continue; }
//            let phi = f as f64 / total as f64;
//
//            if (phi < lowest) & (phi!=0.0)
//            {
//                lowest= phi;
//                lowest_index=f;
//                println!("{} {} {} {}",total,phi,lowest,lowest_index);
//            }
//
//
//        }
//
//}
//
//fn euler71()
//{
//    let target = 3.0/7.0;
//    let mut best_match =0.0;
//    let mut best_match_numerator =0;
//    for f in 1 ..= 1_000_000
//        {
//            let mut exclusion = 0;
//            let mut starting_point =1;
//            if f % 7 ==0
//            {
//                exclusion = (f/7) *3;
//                starting_point = (f/7) *2;
//            }
//            for g in starting_point ..= 1_000_000
//                {
//                    if g == exclusion {continue}
//                    let number=g as f64 / f as f64;
//                    if number > target {break}
//                    if number > best_match
//                        {
//                            best_match = number;
//                            best_match_numerator=g;
//                            println!("best match {}/{} {} {} {}", g,f,best_match,target,best_match_numerator);
//                        }
//                }
//        }
//}
//
//
//const EULER_72_TARGET: u64 =1_000_000;
//fn euler72()
//{
//    let mut total:u64 =0;
//    for f in 1 ..= EULER_72_TARGET
//        {
//           // println!("f {} {}",f, total);
//            for g in 1 ..= EULER_72_TARGET
//                {
//                    if g>=f {break}
//                    if (f%2 ==0) & (g%2==0) {continue}
//                    if (f%3 ==0) & (g%3==0) {continue}
//                    if (f%5 ==0) & (g%5==0) {continue}
//                    if gcd(f,g) == 1
//                    {
//                        total+=1;
//                       // println!("{}/{}",g,f);
//                    }
//                }
//            if f % 1 == 0
//            {
//                println!("f {} {}", f, total);
//            }
//        }
//}
//
//fn euler72_2()
//{
//    let mut total:u64 =0;
//    let sieve = primal::Sieve::new(1_000_000);
//    let mut prime_factors:Vec<(usize,usize)> = Vec::new();
//    for f in 1 ..= EULER_72_TARGET
//        {
//
//           // println!("f {}",f);
//            let mut factor_count = f-1;
//            match sieve.factor(f as usize)
//                {
//                    Ok(number) => prime_factors = number,
//                    Err(x) => println!("Cannot factor {}.", f),
//                }
//            let mut used_array:Vec<u32> = vec![0; f as usize];
//            for g in 1 .. f {used_array[g as usize]=1;}
//            for g in prime_factors.iter()
//                {
//
//
//                    let (factor,_expo) =g;
//                   // let multple = (f-1)/ *factor as u64;
//                    //factor_count -= multple;
//                    //println!("prime {}",factor);
//
//                    let mut temp =*factor;
//                    while temp < f as usize
//                        {
//                            if used_array[temp as usize]==1
//                            {
//                                used_array[temp as usize] = 0;
//                                factor_count -= 1;
//                            }
//                            //   print!("{} ",temp);
//                            temp+=*factor;
//                        }
//                }
//            total+=factor_count;
//            // println!("f {} {}",f, total);
//
//            if f % 1 == 0
//            {
//                println!("f2 {} {}", f, total);
//            }
//        }
//}


//fn factorial_digits(number:u32) -> u32
//{
//    let mut test_number =number;
//    let mut total=0;
//    while test_number >0
//        {
//            total+=factorial_cache[test_number%10];
//            test_number/=10;
//        }
//    total
//}

fn euler73()
{
    let mut count =0;
    for f in 5 ..= 12000
        {
            for g in (f/3) +1 ..= f/2
                {
                    println!("{}/{}",g,f);
                    //let fract = g as f64 / f as f64;
                    //if (fract > 1 as f64 / 3 as f64) & (fract > 1 as f64 / 2 as f64)
                   // {
                        if gcd(f,g) == 1
                        {
                            count+=1;
                            println!("count={}",count);
                        }
                   // }
                }
        }
}

fn euler74() //Digit factorial chains
{
    let mut factorial_cache:[u32;10] = [0;10];
    for f in 0 .. 10
        {
            factorial_cache[f]=f.factorial() as u32;
        }
    let mut highest_count =0;
    for f in 1 .. 1_000_000
        {
            let mut chain_vector:Vec<u32> = Vec::new();
            let mut test_number =f;
            let mut count =0;

            print!(" f ={} ", f);
            loop  // !chain_vector.contains(&test_number)
                {
                    let mut total=0;
                    count+=1;
                    while test_number > 0
                        {
                            //if test_number <
                           // total += factorial_cache[(test_number % 10) as usize];
                            let test_number_digit = test_number % 10;
                            total+=test_number_digit.factorial();
                            test_number /= 10;
                        }
                    print!("{} ", total);
                    if chain_vector.contains(&total)
                    {
                        break;
                    }
                    else
                    {
                        chain_vector.push(total);
                        test_number = total;
                    }
                }
            println!();
            if count==60
            {
                highest_count+=1;
            }
            println!("count={} {}",count,highest_count);
        }
}


const EULER_75_TEST_SIZE:u64 = 1_500_000;

fn euler75() //Singular integer right triangles
{
    let mut test_vector = vec![0;(EULER_75_TEST_SIZE+1) as usize];
    let mut square_vector:Vec<u64> = Vec::new();
    for f in 1 .. 1_000_000
        {
            if f*f > EULER_75_TEST_SIZE {break}
            square_vector.push(f as u64 * f as u64);

        }
    for f in 1 .. EULER_75_TEST_SIZE
        {
            let multiple:u64  = 2 *f;
            let constant = f *f ;
            //if constant + multiple > EULER_75_TEST_SIZE {continue}
            for g in 1 ..EULER_75_TEST_SIZE/2 //square_vector.iter()
                {
                    if  constant >= g *g  {continue}
                    let temp:u64 = (g *g) -constant;
                    if temp % multiple  == 0
                    //if temp2 % 2 ==0
                    {
                        let temp2 = temp/multiple;
                        let leg_a = temp2;
                        let leg_b = g;
                        let leg_c = temp2+f;
                        let total=leg_a+leg_b+leg_c;
                        if temp2 +g > EULER_75_TEST_SIZE *2 {break}
                        if total <= EULER_75_TEST_SIZE
                        {
                            println!("({}) {} {} {} {}",f,leg_a,leg_b,leg_c,total);
                            test_vector[total as usize]+=1;
                            println!("test vector ={}",test_vector[total as usize]);
                        }

                    }

                }
        }
    let mut total=0;
    for (i,f) in test_vector.iter().enumerate()
        {
            println!("test vector ={} {} ",i,*f);
            if *f == 2 { total+=1;}
        }
    println!("total = {}",total);

}

const EULER_76_TEST_SIZE:usize = 100;
const EULER_76_TEST_NUMBER:u32 = 100;
fn euler_76() //Counting summations
{
    let mut sum =0;
    let mut test_vec =  Vec::new();
    test_vec.push(EULER_76_TEST_NUMBER);
    while test_vec[0]> 1
        {
            let mut done =false;
            for g in (0 .. test_vec.len()).rev()
                {
                    if test_vec[g] ==  1  {continue}

                    test_vec[g]-=1;
                    if g != test_vec.len() // if not the last one
                    {
                        for h in g+1 .. test_vec.len()
                            {
                                if test_vec[h]+1 <= test_vec[h-1]
                                {
                                    while test_vec.len() > h+1 {test_vec.pop();};
                                    while (vector_sum_u32(test_vec.clone()) < EULER_76_TEST_NUMBER) & (test_vec[h]+1 <= test_vec[h-1])
                                        {
                                            test_vec[h]+=1;
                                        }
                                    while vector_sum_u32(test_vec.clone()) < EULER_76_TEST_NUMBER
                                    {
                                        let  mut push_value = EULER_76_TEST_NUMBER-vector_sum_u32(test_vec.clone());
                                        if push_value > test_vec[test_vec.len()-1]
                                        {
                                            push_value=test_vec[test_vec.len()-1];
                                        }
                                        test_vec.push(push_value);
                                    }
                                    done =true;
                                    break;
                                }
                            }
                        if vector_sum_u32(test_vec.clone()) < EULER_76_TEST_NUMBER {test_vec.push(1); break;}
                        if done {break;}
                    }
                    else
                    {
                        //test_vec[g]-=1;
                         test_vec.push(1);
                        break;
                    }

                }
            print_vec(&test_vec);
            sum+=1;
        }
        println!("sum = {}",sum);



}



fn recurse_77(total:u32, elements:&std::vec::Vec<i32>)  -> u32
{
    let mut count = 0;
    if elements.is_empty() {return 0;}
    let mut new_elements=elements.clone();
    let current_number=new_elements.pop().unwrap() as u32;
    //println!("current number {}",current_number);
    if total % current_number == 0  {count += 1;}
    for f in (0 .. total).step_by(current_number as usize)
        {
           //println!("f={}",f);
            count += recurse_77(total - f, &new_elements.clone());
        }
    count
}

fn euler_77()  //Prime summations
{
    let mut prime_number_vector = Vec::new();
    for f in (2 ..=100).rev()
        {
            if prime_check(f)
            {
                prime_number_vector.push(f);
            }
        }
//    for f in prime_number_vector.iter()
//        {
//           // println!("{}",f);
//        }
    for f in 2 .. 1000
        {
            let permutations=recurse_77(f,&prime_number_vector.clone());
            println!("{} {}",f,permutations);
        }

}

//fn euler_78() //Coin partitions
//{
//    let mut partitions = vec![1,1,2];
//
//    while partitions[partitions.len()-1] % 1_000_000 != 0
//        {
//            let mut val = 0;
//            let mut k:i32 = 1;
//            let mut n:i32 = partitions.len() as i32;
//            let mut temp = (k * (3 * k - 1)) / 2;
//            let neg_one:i32 =-1;
//            while n >= temp
//                {
//                    val = val + (neg_one.pow(k as u32 - 1)) * partitions[(n-temp) as usize ];
//                    k = k + 1;
//                    temp = (k * (3 * k - 1)) / 2;
//                }
//
//            k = -1;
//            let mut temp = (k * (3 * k - 1)) / 2;
//            while n >= temp
//                {
//                    val = val + (neg_one.pow(k as u32 - 1)) * partitions[(n - temp) as usize];
//                    k = k - 1;
//                    temp = (k * (3 * k - 1)) / 2;
//                }
//
//
//            partitions.push(val % 1_000_000);
//
//            println!("{} {}",partitions.len() - 1, partitions[partitions.len() - 1]);
//        }
//}
fn euler79()  //Passcode derivation
{
    let  matrix_vector /*:Vec<Vec<u32>> */ = vec![[0 ; EULER_81_SIZE] ; EULER_81_SIZE ];
    let contents = fs::read_to_string("p079_keylog.txt")
        .expect("Something went wrong reading the file");
    let mut string_lines:Vec<&str>=contents.split_whitespace().collect();

    for  f in string_lines.iter_mut()
    {
        println!("{}",f);
        *f=f.trim_start_matches('7');
        println!("{}",f);
    }
    for  f in string_lines.iter_mut()
        {
            println!("{}",f);
            *f=f.trim_start_matches('3');
            println!("{}",f);
        }
    for  f in string_lines.iter_mut()
        {
            println!("{}",f);
            *f=f.trim_start_matches('1');
            println!("{}",f);
        }
    for  f in string_lines.iter_mut()
        {
            println!("{}",f);
            *f=f.trim_start_matches('6');
            println!("{}",f);
        }
    for  f in string_lines.iter_mut()
        {
            println!("{}",f);
            *f=f.trim_start_matches('2');
            println!("{}",f);
        }
    for  f in string_lines.iter_mut()
        {
            println!("{}",f);
            *f=f.trim_start_matches('8');
            println!("{}",f);
        }
    println!("final");
    for f in string_lines.iter()
        {
           println!("{}",f);
           // f.remove[0];
        }
}

fn euler_80()
{
    let mut sum =0;
    //%These choices depend on the problem being solved
    for g in 1 .. 100
        {
            let mut x: BigInt = (g*5).to_bigint().unwrap();    //BigUint::one(); // %The initial value
            let mut n: BigInt = (g*10).to_bigint().unwrap();    //BigUint::one(); // %The initial value
            if is_square(g) {continue}
            println!("x={}", x.clone()/5);
            for _f in 1..200
                {
                    x *= 10_i32.to_bigint().unwrap();
                    n *= 10_i32.to_bigint().unwrap();
                }
            let max_iterations = 400; //%Don't allow the iterations to continue indefinitely
            //haveWeFoundSolution = false %Have not converged to a solution yet

            for _i in 1..max_iterations
                {
                    let nx = (x.clone() + n.clone() / x.clone()) / 2_i32.to_bigint().unwrap();

                    x = nx.clone(); //%Update x0 to start the process again
                   // println!("{}", x);
                }
            //println!("digits={}",x.digits());
            let mut digit_vec = Vec::new();
            while x > 0_i32.to_bigint().unwrap()
                {
                    let temp2 = x.to_biguint().unwrap();
                    let temp3 = temp2 % 10_u32.to_biguint().unwrap();
                    ;
                    let temp4 = truncate_biguint_to_u32(&temp3);
                    //let temp=truncate_biguint_to_u32(temp2 % (10_u32.to_biguint().unwrap()));
                    digit_vec.push(temp4);
                    x /= 10_i32.to_bigint().unwrap();
                }
            println!("length= {}", digit_vec.len());
            while digit_vec.len() > 100
                {
                    digit_vec.remove(0);
                }
            print_vec(&digit_vec);
            sum +=  vector_sum_u32(digit_vec);
            println!("sum={}", sum);
        }


}

const EULER_81_TEST_STRING: &'static str =

"131,201,630,537,805
 673,96,803,699,732
 234,342,746,497,524
 103,965,422,121,37
 18,150,111,956,331";

const EULER_81_SIZE:usize =80;

fn euler81() //Path sum: two ways
{
   let mut matrix_vector /*:Vec<Vec<u32>> */ = vec![[0 ; EULER_81_SIZE] ; EULER_81_SIZE ];
    let contents = fs::read_to_string("p081_matrix.txt")
        .expect("Something went wrong reading the file");
    let string_lines:Vec<&str>=contents.split_whitespace().collect();
    for f in 0 .. string_lines.len()
        {
            //println!("{}",string_lines[f]);
            let elements:Vec<&str> = string_lines[f].split(',').collect();
            for g in 0 .. elements.len()
                {
                    matrix_vector[g][f]=elements[g].parse::<i32>().unwrap() as u32;
                }
        }

    for f in 0 .. EULER_81_SIZE
        {
            for g in 0 .. EULER_81_SIZE
                {
                    print!("{}, ",matrix_vector[f][g]);
                }
            println!();
        }
    for f in 1 .. (EULER_81_SIZE * 2) -1
        {
            //let mut temp =0;
            for g in 0 ..= f
                {
                    let mut temp =999999;
                    let x:i32 = (f-g) as i32 ;
                    let y:i32 = g as i32;
                    if x  >= EULER_81_SIZE as i32 {continue}
                    if y >= EULER_81_SIZE as i32 {continue}
                    println!("x={} y={}",x,y);
                    if x-1>=0
                    {
                        temp= matrix_vector[(x-1) as usize][y as usize];
                        println!("temp set to {}",temp);
                    }
                    if y-1>=0
                    {
                        if matrix_vector[x as usize ][(y-1) as usize] < temp
                        {
                            temp = matrix_vector[x as usize][(y-1) as usize];
                            println!("temp set to {}",temp);
                        }
                    }
                    assert_ne!(temp,0);
                    matrix_vector[f-g][g]  +=temp;
                }
            println!();
        }
    for f in 0 .. EULER_81_SIZE
        {
            for g in 0 .. EULER_81_SIZE
                {
                    print!("{}, ",matrix_vector[f][g]);
                }
            println!();
        }
        println!("total ={}",matrix_vector[EULER_81_SIZE-1][EULER_81_SIZE-1]);
}

const EULER_82_SIZE:usize =80;

fn euler_82() //Path sum: three ways
{
    let mut matrix_vector /*:Vec<Vec<u32>> */ = vec![[0 ; EULER_82_SIZE] ; EULER_82_SIZE ];
    let contents = fs::read_to_string("p082_matrix.txt")
        .expect("Something went wrong reading the file");
    let string_lines:Vec<&str>=contents.split_whitespace().collect();
    for f in 0 .. string_lines.len()
        {
            //println!("{}",string_lines[f]);
            let elements:Vec<&str> = string_lines[f].split(',').collect();
            for g in 0 .. elements.len()
                {
                    matrix_vector[f][g]=elements[g].parse::<i32>().unwrap() as u32;
                }
        }

    for f in 0 .. EULER_82_SIZE
        {
            for g in 0 .. EULER_82_SIZE
                {
                    print!("{}, ",matrix_vector[f][g]);
                }
            println!();
        }
    for f in (0 ..EULER_82_SIZE -1).rev()
        {
            println!("f={}",f);
            let mut this_row:Vec<u32> = vec![0; EULER_82_SIZE as usize];
            print!(" this row= ");
            for g in 0 .. EULER_82_SIZE
                {
                    this_row[g]=matrix_vector[g][f];
                }
            print_vec(&this_row);
            let mut scratch_vector:Vec<u32> = vec![0; EULER_82_SIZE as usize];
            for g in 0 .. EULER_82_SIZE as i32
                {
                    println!("g={}",g);
                    let mut scratch =0;
                    //let mut candidate =

                    scratch_vector[g as usize]=matrix_vector[g as usize][f as usize]+matrix_vector[g as usize][f as usize +1];
                    if g>=1
                    {
                        scratch=matrix_vector[g as usize][f as usize];
                        for h in (0..g).rev()
                            {
                                println!("h1 = {} {}",h,scratch_vector[g as usize]);
                                scratch+=matrix_vector[h as usize][f as usize];
                                let test_num= scratch+matrix_vector[h as usize][f as usize +1];
                                println!("Test num={}",test_num);
                                if test_num < scratch_vector[g as usize]
                                {
                                    scratch_vector[g as usize] = test_num;
                                }
                            }
                    }
                    scratch=matrix_vector[g as usize][f as usize];

                    for h in g+1 ..(EULER_82_SIZE) as i32
                        {
                            println!("h2 = {} {}",h,scratch_vector[g as usize]);
                            scratch+=matrix_vector[h as usize][f as usize];
                            let test_num= scratch+matrix_vector[h as usize][f as usize +1];
                            println!("Test num={}",test_num);
                            if test_num < scratch_vector[g as usize]
                            {
                                scratch_vector[g as usize] = test_num;
                            }
                            //println!("scratch vector = {}",h);
                           //scratch_vector[h as usize]=scratch ;
                        }
                    //scratch_vector[g]=

                }
            for g in 0 .. EULER_82_SIZE
                {
                    matrix_vector[g as usize][f as usize] = scratch_vector[g];
                }
            let mut sorted_vector:Vec<u32> = scratch_vector.clone();
            sorted_vector.sort();
            let minimum_value = sorted_vector.first().unwrap();
            print_vec(&scratch_vector);
            println!("minimum_value={}",minimum_value);
        }
}
const EULER_83_SIZE:usize =80;

#[derive(Clone)]
#[derive(Debug)]
struct ElementFn83
{
    pub location: Point<u32>,
    pub value: u32,
}


fn calc_path_cost(this_vector:&Vec<ElementFn83>, target:Point<u32>) -> u32
{
    let mut total:u32 = 0;
    for f in this_vector
        {
            total+=f.value;
            if target == f.location
            {
                break;
            }
        }
    total
}

fn check_other_paths(path_vector:&VecDeque<Vec<ElementFn83>>,possible:Point<u32>) -> Option<u32>
{
    for (index,f) in path_vector.iter().enumerate()
        {
            let mut in_that_path = false;
            for g in f.iter()
                {
                    if g.location == possible
                    {
                            return Some(index as u32);
                    }
                }
        }
    None
}

fn check_this_path(this_path:&Vec<ElementFn83>, possible:Point<u32>) ->bool
{
    for f in this_path.iter()
        {
            if f.location == possible
            {
                return true;
            }
        }
    false

}

fn check_new_element(path_vector:&mut VecDeque<Vec<ElementFn83>>,new_possible_element:ElementFn83,this_path:&Vec<ElementFn83>)
{
    let in_this_path = check_this_path(&this_path, new_possible_element.location);
    if !in_this_path
    {
        let other_path=check_other_paths(&path_vector,new_possible_element.location);
        if other_path.is_some()
        {
            let mut old_cost = calc_path_cost(&this_path,new_possible_element.location);
            old_cost += new_possible_element.value;
            let new_cost= calc_path_cost(&path_vector[other_path.unwrap() as usize],new_possible_element.location);
            if old_cost > new_cost
            {
               // retu;
            }
            else
            {
                path_vector.remove(other_path.unwrap() as usize);
                //let x = *this_path.to_vec();
                let mut new_path  = this_path.clone();
                new_path.push(new_possible_element);
                path_vector.push_back(new_path);

            }

        }
        else
        {
            let mut new_path = this_path.clone();
            new_path.push(new_possible_element);
            path_vector.push_back(new_path);
        }

    }

}

fn euler_83() //Path sum: four ways
{
    let mut path_vector:VecDeque<Vec<ElementFn83>> = VecDeque::new();
    let  working_vector_queue:VecDeque<Vec<ElementFn83>> = VecDeque::new();
    //let mut final_vector_queue:VecDeque<Vec<ElementFn83>> = VecDeque::new();
    let mut matrix_vector /*:Vec<Vec<u32>> */ = vec![[0 ; EULER_83_SIZE] ; EULER_83_SIZE ];
    let contents = fs::read_to_string("p083_matrix.txt")
        .expect("Something went wrong reading the file");
    let string_lines:Vec<&str>=contents.split_whitespace().collect();
    for f in 0 .. string_lines.len()
        {
            //println!("{}",string_lines[f]);
            let elements:Vec<&str> = string_lines[f].split(',').collect();
            for g in 0 .. elements.len()
                {
                    matrix_vector[f][g]=elements[g].parse::<i32>().unwrap() as u32;
                }
        }

    for f in 0 .. EULER_83_SIZE
        {
            for g in 0 .. EULER_83_SIZE
                {
                    print!("{}, ",matrix_vector[g][f]);
                }
            println!();
        }
    let x:ElementFn83 = ElementFn83 {location : Point{x:0,y:0},value:matrix_vector[0][0]};
    let mut first_path:Vec<ElementFn83>=Vec::new();
    first_path.push(x);
    path_vector.push_back(first_path);
    while path_vector.len() >0
        {
            println!("active paths {}",path_vector.len());

            let this_path = path_vector.pop_front().unwrap();   //this gets us a path to work on
            println!("evaluating {:?}",this_path);
            let current_element:&ElementFn83 = &this_path[this_path.len()-1];
            println!("Cost={}",calc_path_cost(&this_path,current_element.location));
            println!();
            if (current_element.location.x == EULER_83_SIZE as u32 -1) &
                (current_element.location.y == EULER_83_SIZE as u32 -1)
                {
                    if path_vector.len() == 1
                    {
                        break;
                    }
                    else
                    {
                        path_vector.push_back(this_path.clone());
                    }
                }
            if current_element.location.x !=0
                {

                    let possible:Point<u32> = Point {x:current_element.location.x-1,y:current_element.location.y};
                    let new_possible_element:ElementFn83= ElementFn83 {location:possible,value:matrix_vector[possible.x as usize][possible.y as usize]};
                    check_new_element(&mut path_vector,new_possible_element,&this_path);

                }
            if current_element.location.y !=0
            {

                let possible:Point<u32> = Point {x:current_element.location.x,y:current_element.location.y-1};
                let new_possible_element:ElementFn83= ElementFn83 {location:possible,value:matrix_vector[possible.x as usize][possible.y as usize]};
                check_new_element(&mut path_vector,new_possible_element,&this_path);

            }
            if current_element.location.x != EULER_83_SIZE as u32 -1
            {

                let possible:Point<u32> = Point {x:current_element.location.x+1,y:current_element.location.y};
                let new_possible_element:ElementFn83= ElementFn83 {location:possible,value:matrix_vector[possible.x as usize][possible.y as usize]};
                check_new_element(&mut path_vector,new_possible_element,&this_path);

            }
            if current_element.location.y !=EULER_83_SIZE as u32 -1
            {

                let possible:Point<u32> = Point {x:current_element.location.x,y:current_element.location.y+1};
                let new_possible_element:ElementFn83= ElementFn83 {location:possible,value:matrix_vector[possible.x as usize][possible.y as usize]};
                check_new_element(&mut path_vector,new_possible_element,&this_path);

            }
        }







}


fn euler_84()
{
    play();
}

const EULER_85_X_SIZE:usize =1;
const EULER_85_Y_SIZE:usize =2000;
fn euler_85()  //Counting rectangles
{
    let mut best_fit:i32 =2_000_000;
    let mut best_x=0;
    let mut best_y=0;
    for f in 1 ..=2000
        {
            for g in 1 ..=2000
                {
                    let mut total_rectangles:i32 = 0;
                    for x_size in 1..=f //EULER_85_X_SIZE
                        {
                            for y_size in 1..=g //EULER_85_Y_SIZE
                                {
                                    if total_rectangles > 3_000_000 {break}
                                    total_rectangles += (1 + f - x_size) * (1 + g - y_size);
                                    //println!("Total Rectangles={} {} {}", x_size, y_size, total_rectangles);
                                }
                        }
                    println!("Total Rectangles={} {} {} {} {} {} {}", f,g,total_rectangles,best_fit,best_x,best_y,best_x*best_y);
                    if total_rectangles > 2_000_000
                    {
                        if best_fit > total_rectangles-2_000_000
                        {
                            best_fit=total_rectangles-2_000_000;
                            best_x=f;
                            best_y=g;
                        }
                    }
                    if total_rectangles <= 2_000_000
                    {
                        if best_fit > 2_000_000-total_rectangles
                        {
                            best_fit=2_000_000-total_rectangles;
                            best_x=f;
                            best_y=g;
                        }
                    }
                }
        }
}


const EULER_86_SAMPLE_SIZE:u64 =1800;
fn euler_86()  //Cuboid route
{
    let mut max_size = EULER_86_SAMPLE_SIZE;
    loop
        {
            max_size+=1;
            let mut count: u64 = 0;
            for f in 1..=max_size
                {
                    for g in 1..=f // EULER_86_SAMPLE_SIZE
                        {
                            for h in 1..=g //g ..= EULER_86_SAMPLE_SIZE
                                {
                                    let total = (f * f) + (g + h) * (g + h);
                                    let total_float = total as f64;
                                    let total_sqrt = total_float.sqrt();
                                    if total_sqrt.fract() == 0.0
                                    {
                                        count += 1;
                                       // println!("{} {} {} {} {}", max_size,f, g, h, count);
                                    }
                                }
                        }
                }
            println!("{} {}",max_size,count);
        }
}

const EULER_87_SAMPLE_SIZE:u64 =50_000_000;
fn euler_87()  //Counting rectangles
{
    let mut hit_vector = vec![0 ; EULER_87_SAMPLE_SIZE as usize];
    let sieve = primal::Sieve::new(EULER_87_SAMPLE_SIZE as usize);
    let mut square_vec:Vec<u64> = Vec::new();
    let mut cube_vec:Vec<u64> = Vec::new();
    let mut quad_vec:Vec<u64> = Vec::new();
    for p in sieve.primes_from(2).take_while(|x| *x < EULER_87_SAMPLE_SIZE as usize)
        {
             println!("{}", p);
            let f =p as u64;
            let f2:u64=f*f;
            let f3:u64=f*f*f;
            let f4:u64=f*f*f*f;
            if f2 < EULER_87_SAMPLE_SIZE
            {
                square_vec.push(f2);
            }
            else { break;}
            if f3 < EULER_87_SAMPLE_SIZE
            {
                cube_vec.push(f3);
            }
            if f4 < EULER_87_SAMPLE_SIZE
            {
                quad_vec.push(f4);
            }

        }

    println!("number of squares={}",square_vec.len());
    println!("number of cubes={}",cube_vec.len());
    println!("number of quads={}",quad_vec.len());
    for square in square_vec
        {
            for cube in cube_vec.iter()
                {
                    for quad in quad_vec.iter()
                        {
                            let test_number=square+cube+quad;
                            if  test_number< EULER_87_SAMPLE_SIZE
                            {
                                hit_vector[test_number as usize]=1;
                            }

                        }
                }
        }
    let total:u64 = hit_vector.iter().sum();
    println!("total={}",total);
}



const EULER_88_SAMPLE_SIZE:u32 =12000;
fn euler_88()  //Product-sum numbers
{
 let totals:Vec<Vec<u32>>=Vec::new();
    let mut sum_vector = vec![0;EULER_88_SAMPLE_SIZE as usize +1 ];
    print_vec(&sum_vector);
 for f in 2 ..= EULER_88_SAMPLE_SIZE
     {
         println!("f={}",f);
        let mut test_vec:Vec<u32>= Vec::new();
         test_vec.push(2);
         test_vec.push(f);
         for g in 0 .. f-2
             {
                 test_vec.push(1);
             }
         let sum:u32=test_vec.iter().sum();
         let product:u32= test_vec.iter().product();
         assert_eq!(sum,product);
         println!("total={}",sum);
         let mut test_vec2:Vec<u32>=vec![1;f as usize];
         let mut not_done = true;
         let mut target =sum;
         while not_done
             {
                 for g in 0 .. f
                 {
                   //  println!("g={} {} {}",g,f,target);
                   if g < f-1
                       {
                           if test_vec2[g as usize]+1 <= test_vec2[g as usize +1]
                               {
                                   test_vec2[g as usize]+=1;
                                   for h in 0..g
                                       {
                                           test_vec2[h as usize] = 1;
                                       }
                                   let mut test_product:u32=1;  // test_vec2.iter().product();
                                   for f in test_vec2.iter().rev()
                                       {
                                           test_product *=f;
                                           if test_product > target {break}
                                       }
                                   if  test_product > target
                                   {
                                       test_vec2[g as usize]=1;
//                                       for h in 0 ..= g
//                                           {
//                                               test_vec2[h as usize]=1; //test_vec2[g as usize];
//                                           }
                                       continue;

                                   }
                                   let product:u32= test_product; // test_vec2.iter().product();
                                   let sum:u32 =test_vec2.iter().sum();
                                   if  product == sum
                                     {
                                         if sum<target
                                         {
                                             println!("New target={}", sum);
                                             target = sum;
                                         }
                                     }
                                   break;
                               }
                       }
                     else
                     {
                         if test_vec2[g as usize]+1 < target/2
                             {
                                 test_vec2[g as usize]+=1;
                                 for h in 0 .. g
                                     {
                                         test_vec2[h as usize]=1;
                                     }
                                 let product:u32=test_vec2.iter().product();
                                 let sum:u32 =test_vec2.iter().sum();
                                 if  product == sum
                                 {
                                     if sum<target
                                     {
                                         println!("New target={}", sum);
                                         target = sum;
                                     }
                                 }
                                 //print_vec(&test_vec2);
                             }
                         else
                         {
                             not_done=false;
                             println!("done");
                             break;
                         }
                     }
                 }
             }
         println!("updating sum vector");
         println!("f,target {} {}",f,target);
         sum_vector[f as usize]=target;
         println!("updated sum vector");

     }
    print_vec(&sum_vector);
    sum_vector.sort();
    sum_vector.dedup();
    print_vec(&sum_vector);
    let sum:u32 = sum_vector.iter().sum();
    println!("sum={}",sum);
}


//fn Herons_formula(a:BigInt,b:BigInt,c:BigInt) -> f64
//{
//    let s:f64 =(a as f64 + b as f64 + c as f64)/2.0;
//    let area_squared_float= (s-a as f64)*(s-b as f64)*(s-c as f64)*s;
//    let area = area_squared_float.sqrt();
//    area
//}


//fn euler_89()
//{
//    let contents = fs::read_to_string("p089_roman.txt")
//        .expect("Something went wrong reading the file");
//    //let mut count:i32=0;
//    //let mut total:i32 = 0;
//    let vec: Vec<&str> = contents.split('\n').collect();
//    let mut before=0;
//    let mut after=0;
//    for element in vec
//        {
//            let mut total = 0;
//            before+=element.len();
//            println!("*{}*",element);
//
//            let mut digit = element.chars();
//            //let next_char_possible=digit.next();
//            let mut next_char:char = ' ';
//            //println!("Phase 1");
//            loop
//                {
//                    let next_char_possible=digit.next();
//
//                    match next_char_possible
//                        {
//                            None => break,
//                            Some(x) => next_char = x,
//                        }
//                    //print!("{}",next_char);
//                    match next_char
//                        {
//                            'M' => {
//                                total += 1000;
//                                next_char = ' ';
//                            },
//                            'C' =>
//                                {
//                                    if digit.clone().peekable().peek().is_none() {break}
//                                    if *digit.clone().peekable().peek().unwrap()  == 'M'
//                                    {
//                                        total += 900;
//                                        digit.next();
//                                        //println!("next1");
//                                        next_char = ' ';
//                                        break;
//                                    }
//                                        else if *digit.clone().peekable().peek().unwrap()  == 'D'
//                                        {
//                                            total += 400;
//                                            digit.next();
//                                           // println!("next2");
//                                            next_char = ' ';
//                                            break;
//                                        }
//                                        else { break; }
//                                },
//                            //' '   => (),
//                            _ => break,
//                        }
//                }
//            //println!("total {}",total);
//            //println!("Phase 2 {}",digit.as_str() );
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        //print!("-");
//                        let next_char_possible = digit.next();
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    match next_char
//                        {
//                            'D' => {
//                                total += 500;
//                                next_char = ' ';
//                            },
//                            'M' => {
//                                println!("BAD NUMBER");
//                            },
//                            _ => break,
//                        }
//                }
//            //println!("total {}",total);
//            //println!("Phase 3 {}",digit.as_str() );
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        let next_char_possible = digit.next();
//                        //print!("+");
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    //println!("next char {}",next_char);
//                    match next_char
//                        {
//                            'C' => {
//                                total += 100;
//                                next_char = ' ';
//                            },
//                            'X' =>
//                                {
//                                    if digit.clone().peekable().peek().is_none() {break}
//                                    if *digit.clone().peekable().peek().unwrap()  == 'C'
//                                    {
//                                        total += 90;
//                                        digit.next();
//                                        next_char = ' ';
//                                        break;
//                                    }
//                                    else if *digit.clone().peekable().peek().unwrap()  == 'L'
//                                        {
//                                            total += 40;
//                                            digit.next();
//                                            next_char = ' ';
//                                            break;
//                                        }
//                                    else { break; }
//                                },
//                            'M' | 'D'=> {
//                                println!("BAD NUMBER");
//                            },
//                            ' '   => (),
//                            _ => break,
//                        }
//
//                }
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        //print!("-");
//                        let next_char_possible = digit.next();
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    match next_char
//                        {
//                            'L' => {
//                                total += 50;
//                                next_char = ' ';
//                            },
//                            'M' | 'D' | 'C'  => {
//                                println!("BAD NUMBER");
//                            },
//                            _ => break,
//                        }
//                }
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        let next_char_possible = digit.next();
//                        //print!("+");
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    //println!("next char {}",next_char);
//                    match next_char
//                        {
//                            'X' => {
//                                total += 10;
//                                next_char = ' ';
//                            },
//                            'I' =>
//                                {
//                                    if digit.clone().peekable().peek().is_none() {break}
//                                    if *digit.clone().peekable().peek().unwrap()  == 'X'
//                                    {
//                                        total += 9;
//                                        digit.next();
//                                        next_char = ' ';
//                                        break;
//                                    }
//                                    else if *digit.clone().peekable().peek().unwrap()  == 'V'
//                                    {
//                                        total += 4;
//                                        digit.next();
//                                        next_char = ' ';
//                                        break;
//                                    }
//                                    else { break; }
//                                },
//                            'M' | 'D'  | 'C' | 'L' => {
//                                println!("BAD NUMBER");
//                            },
//                            ' '   => (),
//                            _ => break,
//                        }
//
//                }
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        //print!("-");
//                        let next_char_possible = digit.next();
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    match next_char
//                        {
//                            'V' => {
//                                total += 5;
//                                next_char = ' ';
//                            },
//                            'M' | 'D' | 'C' | 'L'| 'X'  => {
//                                println!("BAD NUMBER");
//                            },
//                            _ => break,
//                        }
//                }
//            loop
//                {
//                    if next_char == ' '
//                    {
//                        //print!("-");
//                        let next_char_possible = digit.next();
//                        match next_char_possible
//                            {
//                                None => break,
//                                Some(x) => next_char = x,
//                            }
//                    }
//                    match next_char
//                        {
//                            'I' => {
//                                total += 1;
//                                next_char = ' ';
//                            },
//                            'M' | 'D' | 'C' | 'L'| 'X' |'V'  => {
//                                println!("BAD NUMBER");
//                            },
//                            _ => break,
//                        }
//                }
//            println!("total {}",total);
//            let mut output_string:String=String::new();
//            let m_count =total / 1000;
//            for f in 0 .. m_count
//                {
//                    output_string.push('M');
//                }
//            total %=1000;
//            if total >=900
//            {
//                output_string.push('C');
//                output_string.push('M');
//                total -=900;
//            }
//            if total >=500
//            {
//                output_string.push('D');
//                total -=500;
//            }
//            if total >=400
//            {
//                output_string.push('C');
//                output_string.push('D');
//                total -=400;
//            }
//            let c_count =total / 100;
//            for f in 0 .. c_count
//                {
//                    output_string.push('C');
//                }
//            total %=100;
//            if total >=90
//            {
//                output_string.push('X');
//                output_string.push('C');
//                total -=90;
//            }
//            if total >=50
//            {
//                output_string.push('L');
//                total -=50;
//            }
//            if total >=40
//            {
//                output_string.push('X');
//                output_string.push('L');
//                total -=40;
//            }
//            let x_count =total / 10;
//            for f in 0 .. x_count
//                {
//                    output_string.push('X');
//                }
//            total %=10;
//            if total >=9
//            {
//                output_string.push('I');
//                output_string.push('X');
//                total -=9;
//            }
//            if total >=5
//            {
//                output_string.push('V');
//                total -=5;
//            }
//            if total >=4
//            {
//                output_string.push('I');
//                output_string.push('V');
//                total -=4;
//            }
//            let i_count =total;
//            for f in 0 .. i_count
//                {
//                    output_string.push('I');
//                }
//            after+=output_string.len();
//            println!("Output {}",output_string);
//            println!("{} {} {}",before,after,before-after);
//        }
//}

fn get_integer_hypot(a:u32,b:u32) -> Option<u32>
{
    let a_squared = a * a;
    let b_squared = b * b;
    let c_squared = a_squared + b_squared;
    let c_squared_float = c_squared as f64;
    let c = c_squared_float.sqrt();
    if c.fract()!=0.0 {None}
    else {Some(c as u32)}
}

fn get_hypot(a:u32,b:u32) -> f64
{
    let a_squared = a * a;
    let b_squared = b * b;
    let c_squared = a_squared + b_squared;
    let c_squared_float = c_squared as f64;
    let c = c_squared_float.sqrt();
    c
}

fn compare_for_90(first:&Vec<u64>,second:&Vec<u64>) -> bool
{
    let mut squares = vec![[0,1], [0,4], [0,9], [1,6], [2,5], [3,6],[4,9], [6,4],  [8,1]];
    for f in first.iter()
        {
            for g in second.iter()
                {
                    squares.retain(|x| [*f,*g] != *x);
                    squares.retain(|x| [*g,*f] != *x);
                   // if squares.contains(vec![first[f],first[g]]) squares
                  //  if squares.len() == 0 {return true;}
                }
        }
    //println!("{:?} {:?} {:?} squares={}",first,second,squares,9-squares.len());
    if squares.len() == 0 {return true;}
        else {return false;}
}
fn euler_90()
{
    let mut set = vec![0,1,2,3,4,5,6,7,8,9];
    let mut perms:Vec<u64>=Vec::new();
    let mut count = 0;
    let mut new_perms_vector:Vec<Vec<u64>> = Vec::new();
    find_permutations_u64(&mut set,10,&mut perms);
    println!("{:?}",perms);
    let mut perms_vector:Vec<Vec<u64>> = perms.iter().map(|x| number_to_vec_u64(*x)).collect();
    println!("{:?}\n",perms_vector);
    for  f in perms_vector.iter_mut()
        {
            f.pop();
            f.pop();
            f.pop();
            f.pop();

            //f.pop();
            f.sort();
           // if perms_vector[f].contains(&6) {perms_vector[f].push(9);}
        }
    perms_vector.sort();
    perms_vector.dedup();
    for  f in perms_vector.iter_mut()
        {

            if f.len()==6
            {
                if (f.contains(&6)) &&(!f.contains(&9))
                    {
                        f.push(9);
                    }
                else if (!f.contains(&6)) &&(f.contains(&9))
                {
                    f.push(6);
                }
                new_perms_vector.push(f.clone());
            }

           // let mut new_vec =f.clone();
//            new_vec.retain(|x| 6 != *x);
//            if new_vec.len() == 5
//            {
//                new_vec.push(9);
//                new_perms_vector.push(new_vec);
//            }
            //;
           // if f.contains(&6) {f.push(9);}
        }
    println!("{:?}",new_perms_vector);
    println!("{} {}",new_perms_vector.len(),new_perms_vector.len() * new_perms_vector.len());
    for f in new_perms_vector.iter()
        {
            for g in new_perms_vector.iter()
                {
                    if compare_for_90(f,g) {count+=1;}
                }
        }
    println!("count ={} ",count);
}
const EULER_91_CONSTANT:i32 =50;
fn euler_91()
{
let mut count=0;
    for ax in 0 ..= EULER_91_CONSTANT
        {
            for bx in 0 ..= EULER_91_CONSTANT
                {
                    for ay in 0 ..=  EULER_91_CONSTANT
                        {

                            for by in 0 ..=EULER_91_CONSTANT
                                {
                                    let cx:i32=(ax-bx).abs();
                                    let cy:i32=(ay-by).abs();
                                    let a_length_squared = ax * ax + ay * ay;
                                    let b_length_squared = bx * bx + by * by;
                                    let c_length_squared = cx * cx + cy * cy;
                                    let mut test_vec:Vec<i32>=Vec::new();
                                    test_vec.push(a_length_squared);
                                    test_vec.push(b_length_squared);
                                    test_vec.push(c_length_squared);
                                    test_vec.sort();
                                    if (test_vec[0]!=0) & (test_vec[0]+test_vec[1]==test_vec[2]) & (test_vec[2] <= 2 * EULER_91_CONSTANT *EULER_91_CONSTANT)
                                    {
                                        count+=1;
                                        println!("a,b,c {} {} {}",test_vec[0],test_vec[1],test_vec[2]);
                                    }



                                }
                        }

                }
        }
    //count-=EULER_91_CONSTANT;
    println!("count={}",count/2);
}
const EULER_92_SAMPLE_SIZE:u32 =10_000_000;
fn euler_92()   //Square digit chains
{
    let mut count =0;
    for f in 1 ..= EULER_92_SAMPLE_SIZE
        {
            let mut scratch_number =f;

            loop
                {
                    let mut digit_vec:Vec<u32>= Vec::new();
                    print!("{} ",scratch_number);
                    while scratch_number > 0
                        {
                            let temp = scratch_number % 10;
                            digit_vec.push(temp);
                            scratch_number /= 10;
                        }
                    //print_vec(&digit_vec);
                    scratch_number=0;
                    for g in digit_vec.iter()
                        {
                            scratch_number+=g*g;
                        }
                    if (scratch_number==1) | (scratch_number ==89)
                    {
                        if scratch_number ==89 {count+=1;}
                        println!("*{}*",count);
                        break;
                    }
                }
        }
}



//fn generate(k : i32, A : Vec<i32>) -> Vec<i32>
//{
//    if k == 1
//    {
//        return A
//    }
//    else
//    {
//// Generate permutations with kth unaltered
//// Initially k == length(A)
//        generate(k - 1, A);
//
//// Generate permutations for kth swapped with each k-1 initial
//        for i in 0 .. k - 1
//            {
//// Swap choice dependent on parity of k (even or odd)
//                if k %2 == 0
//                {
//                    let temp = A[k - 1];
//                    A[k - 1]= A[i];
//                    A[i] = temp;
//
//                }
//                    swap(A[i], A[k - 1]) // zero-indexed, the kth is at k-1
//                }
//                else
//                {
//                    swap(A[0], A[k - 1])
//                }
//            generate(k - 1, A)
//            }
//    }
//    A
//}


//fn vector_permentations(test_vec:Vec<i32>) -> Vec<Vec<i32>>
//{
//    let mut output_vec:Vec<Vec<i32>>=Vec::new();
//    for f in 0 ..test_vec.len()
//        {
//            let mut new_vec=test_vec.clone();
//            new_vec.remove(f);
//            if new_vecs.len() > 0
//            {
//                new_vecs=vector_permentations(new_vec);
//        }
//    output_vec
//}



fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}



const EULER_93_ADD:u32 = '+' as u32;//-1;
const EULER_93_SUBTRACT:u32 = '-' as u32;//-2;
const EULER_93_MULTIPLY:u32 = '*' as u32;//-3;
const EULER_93_DIVIDE:u32 = '/' as u32;//-4;

fn euler_93()
{
    let mut longest = 0;
    let data_superset:[u32;10] = [0,1, 2, 3, 4,5,6,7,8,9];
    let data_superset_vec = data_superset.to_vec();
    let data = [1, 2, 3, 4];
    let mut _outcome_vector = [false;1000];
    //let mut permutations = Vec::with_capacity(10_000);
    let sample_operator_vector = [EULER_93_ADD, EULER_93_SUBTRACT,EULER_93_MULTIPLY,EULER_93_DIVIDE];
    let mut operator_vector:Vec<Vec<u32>>= Vec::new();
    let data_sets=powerset(&data_superset_vec);
    let data_sets_2:Vec<Vec<u32>> = data_sets.into_iter().filter(|x| (*x).len() ==4).collect();
   // println!("{:?}",data_sets_2);

    for f in sample_operator_vector.iter()
        {
            let mut vec_1: Vec<u32> = Vec::new();
            vec_1.push(*f);
            for g in sample_operator_vector.iter()
                {
                    let mut vec_2 = vec_1.clone();
                    vec_2.push(*g);
                    for h in sample_operator_vector.iter()
                        {
                            let mut vec_3 = vec_2.clone();
                            vec_3.push(*h);
                            operator_vector.push(vec_3);
                        }
                }
        }
    println!("{:?}",operator_vector);
    for index in data_sets_2.iter()
        {
            println!("{:?}",index);
            let mut outcome_vector = [false;1000];
            for g in operator_vector.iter()
                {
                    let mut permutations = Vec::with_capacity(10_000);
                    let mut data_2   = (*index).to_vec();
                    data_2.extend(g.to_vec());
                  //  println!("{:?}",data_2);
                    heap_recursive(&mut data_2, |permutation| {
                        permutations.push(permutation.to_vec())
                    });
                   // permutations.sort();
                   // permutations.dedup();
                    for each_permutation  in permutations.iter()
                        {
                            let mut bad_number = false;
                            //if (each_permutation[0] > 10) | (each_permutation[1] > 10) | (each_permutation[each_permutation.len()-1] <20) {continue;}
                            let mut stack: Vec<f64> = Vec::with_capacity(10);
                            for each_element in each_permutation.iter()
                                {
                                    match *each_element
                                        {
                                            EULER_93_ADD =>
                                                {
                                                    let x = stack.pop();
                                                    if x.is_none() { bad_number = true;/*println!("Bad");*/ break; }
                                                    let y = stack.pop();
                                                    if y.is_none() { bad_number = true;/*println!("Bad");*/ break; }
                                                    stack.push(x.unwrap() + y.unwrap());
                                                    //print!(" *{}* ",x.unwrap()+y.unwrap());
                                                }
                                            EULER_93_SUBTRACT =>
                                                {
                                                    let x = stack.pop();
                                                    if x.is_none() { bad_number = true;/*println!("Bad"); */break; }
                                                    let y = stack.pop();
                                                    if y.is_none() { bad_number = true;/*println!("Bad");*/ break; }
                                                    stack.push(x.unwrap() - y.unwrap());
                                                    //print!(" *{}* ",x.unwrap()-y.unwrap());
                                                }
                                            EULER_93_MULTIPLY =>
                                                {
                                                    let x = stack.pop();
                                                    if x.is_none() { bad_number = true;/*println!("Bad");*/ break; }
                                                    let y = stack.pop();
                                                    if y.is_none() { bad_number = true;/*println!("Bad");*/ break; }
                                                    stack.push(x.unwrap() * y.unwrap());
                                                    //print!(" *{}* ",x.unwrap()*y.unwrap());
                                                }
                                            EULER_93_DIVIDE =>
                                                {
                                                    let x = stack.pop();
                                                    if x.is_none() { bad_number = true;/*println!("Bad"); */ break; }
                                                    let y = stack.pop();
                                                    if y.is_none() { bad_number = true;/*println!("Bad); */ break; }
                                                    if y.unwrap() == 0.0 { bad_number = true;/*println!("Bad");  */break; }
                                                    stack.push(x.unwrap() / y.unwrap());
                                                    // print!(" *{}* ",x.unwrap() /y.unwrap());
                                                }
                                            _ => stack.push(*each_element as f64)
                                        }
                                }
                            if bad_number { continue; }
//                    for g in f.iter()
//                        {
//                            print!("{} ", g);
//                        }
//                    print!("    ");
                            let answer = stack.pop();
                            if answer.is_some()
                                {
                                    if (answer.unwrap() > 0.0) & (answer.unwrap() < 1000.0) & (answer.unwrap().fract()== 0.0)
                                        {
                                            outcome_vector[answer.unwrap() as usize] = true;
                                        }
                                    //       println!("{}", answer.unwrap());
                                } else {
                                //     println!();
                            }
                        }
                }
            let mut length:i32 = 0;
            for f in 1 .. 1000
                {
                    if outcome_vector[f] == false
                        {
                            length=f as i32 -1;
                            break;
                        }
                }
            if length > longest
            {
                longest=length;
            }
            println!("length={} {} ",length,longest)    ;
//            for f in 0..100
//                {
//                    if outcome_vector[f] == true
//                        {
//                            println!("{}", f);
//                        }
//                }
        }
}

const EULER_94_SAMPLE_SIZE:u64 =1_000_000_000;
fn euler_94()  //Almost equilateral triangles
{
    let mut total:BigInt=0_i32.to_bigint().unwrap();
    for f in 2 ..=EULER_94_SAMPLE_SIZE
        {
            //println!("f {}",f);
            let a:BigInt=f.to_bigint().unwrap();
            let b:BigInt=f.to_bigint().unwrap();
            let c:BigInt= f.to_bigint().unwrap()-1_i32.to_bigint().unwrap();
            let d:BigInt=f.to_bigint().unwrap()+1_i32.to_bigint().unwrap();
            let four_area_squared=(a.clone()+b.clone()+c.clone())*(-a.clone()+b.clone()+c.clone())*(a.clone()-b.clone()+c.clone())*(a.clone()+b.clone()-c.clone());
            if four_area_squared.clone() % 16_i32.to_bigint().unwrap() != 0_i32.to_bigint().unwrap() { continue}
            let four_area=four_area_squared.sqrt();
            let p1=a.clone()+b.clone()+c.clone();
            let p2 =a.clone()+b.clone()+d.clone();
            if four_area.clone()*four_area.clone()== four_area_squared.clone()
            {
                println!("a,b,c,P {} {} {} {}",a.clone(),b.clone(),c.clone(),a.clone()+b.clone()+c.clone());
                if p1 <= EULER_94_SAMPLE_SIZE.to_bigint().unwrap()
                {
                    total+=p1.to_bigint().unwrap();
                }
            }

            let four_area_squared=(a.clone()+b.clone()+d.clone())*(-a.clone()+b.clone()+d.clone())*(a.clone()-b.clone()+d.clone())*(a.clone()+b.clone()-d.clone());
            if four_area_squared.clone() % 16.to_bigint().unwrap() != 0.to_bigint().unwrap() { continue}
            let four_area=four_area_squared.sqrt();
            let p1=a.clone()+b.clone()+c.clone();
            let p2 =a.clone()+b.clone()+d.clone();
           // println!("p1,p2 {} {}",p1,p2);
            if four_area.clone()*four_area.clone()== four_area_squared.clone()
            {
                println!("a,b,d,P {} {} {} {}",a.clone(),b.clone(),d.clone(),a.clone()+b.clone()+d.clone());
                if p2 <= EULER_94_SAMPLE_SIZE.to_bigint().unwrap()
                {
                    total+=p2.to_bigint().unwrap();
                }
            }


            if p1 >= EULER_94_SAMPLE_SIZE.to_bigint().unwrap() {break};



        }
    println!("Total={}",total);
}

const EULER_95_CONSTANT:u64 =1_000_000;
fn euler_95()
{
    let mut longest_chain:u64=0;
    let mut smallest_member=0;
    for f in 2 ..=EULER_95_CONSTANT
        {
            //print!("{} ", f);
            let mut scratch_number=f;
            let mut test_vec:Vec<u64>= Vec::new();
            loop
                {
                    if scratch_number > 1_000_000 {break}
                    if test_vec.contains(&scratch_number) {break}
                    test_vec.push(scratch_number);
                    let divisor_vec = find_divisors(scratch_number);

                    //print_vec_u64(&divisor_vec);
                    let sum: u64 = divisor_vec.iter().sum();
                    //print!("{} ", sum);
                    scratch_number=sum;


                }
            //println!();
            if scratch_number < 1_000_000
            {
                if scratch_number  == test_vec[0]
                {
                    test_vec.sort();
                    if test_vec.len() as u64 > longest_chain
                    {
                        longest_chain = test_vec.len() as u64;
                        smallest_member = test_vec[0];
                    }
                    println!("Lowest={} {} {} {}", test_vec[0], test_vec.len(), longest_chain, smallest_member);
                }
            }
        }
}

fn show_grid(this_grid:&Vec<[u32;9]>)
{
    println!();
    println!();
    for f in 0..9
        {
            for g in 0..9
                {
                    print!("{}", this_grid[f][g]);
                    if (g == 2) || (g == 5)
                    {
                        print!("|");
                    }
                }
            println!();
            if (f == 2) || (f == 5)
            {
                println!("-----------");
            }
        }
}



fn check_box(this_grid:Vec<[u32;9]>,x:u32,y:u32,possabilities:Vec<u32>) -> Vec<u32>
{
    //let value_to_check = this_grid[x][y];
    let mut box_to_check=Vec::new();
    let mut used_value = Vec::new();
    let mut new_possabilities = Vec::new();
    let box_x = (x/3) * 3;
    let box_y = (y/3) * 3;

    for f in 0 .. 3
        {
            for g in 0 .. 3
                {
                    box_to_check.push(this_grid[box_x as usize +f as usize][box_y  as usize + g as usize]);
                }
        }
    for f in 0 .. possabilities.len()
        {
            if box_to_check.contains(&possabilities[f])
            {
                used_value.push(possabilities[f]);
            }
        }
    //println!("Box {:?}",box_to_check);
    //println!("used {:?}",used_value);
    for f in possabilities
        {
            if !used_value.contains(&f)
            {
                new_possabilities.push(f);
            }
        }
    new_possabilities
}


fn check_collumn(this_grid:Vec<[u32;9]>,x:u32,y:u32,possabilities:Vec<u32>) -> Vec<u32>
{
    //let value_to_check = this_grid[x][y];
    let mut collumn_to_check=Vec::new();
    let mut used_value = Vec::new();
    let mut new_possabilities = Vec::new();
    for f in 0 .. 9
        {
            collumn_to_check.push(this_grid[f][y as usize]);
        }
    for f in 0 .. possabilities.len()
        {
            if collumn_to_check.contains(&possabilities[f])
            {
                used_value.push(possabilities[f]);
            }
        }
    //println!("used {:?}",used_value);
    for f in possabilities
        {
            if !used_value.contains(&f)
            {
                new_possabilities.push(f);
            }
        }
    new_possabilities
}

fn check_row(this_grid:Vec<[u32;9]>,x:u32,y:u32,possabilities:Vec<u32>) -> Vec<u32>
{
    //let value_to_check = this_grid[x][y];
    let row_to_check = this_grid[x as usize];
    let mut used_value = Vec::new();
    let mut new_possabilities = Vec::new();
  for f in 0 .. possabilities.len()
      {
          if row_to_check.contains(&possabilities[f])
          {
              used_value.push(possabilities[f]);
          }
      }
    //println!("used {:?}",used_value);
    //println!("possabilities {:?}",possabilities);
    for f in possabilities
        {
            if !used_value.contains(&f)
            {
               // println!("push {}",f);
                new_possabilities.push(f);
            }
        }
    new_possabilities
}

fn do_puzzle(this_grid:Vec<[u32;9]>) -> Option<u32>
{
    let mut not_done = true;
    let mut working_grid = this_grid.clone();
    while not_done
        {
           // show_grid(&working_grid);
            not_done = false;  //assume we are done util we find that we are not
            'for_each_number: for h in  0 .. 9
                {
                    for f in 0..9  //across the top
                        {
                            for g in 0..9  //down the side
                                {
                                    let mut posabilities = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
                                    let current_digit = working_grid[f][g];
                                    if current_digit == 0
                                    {
                                        not_done =true;  // found an open spot, we are not done
                                        posabilities = check_row(working_grid.clone(), f as u32, g as u32, posabilities);
                                        //println!("After row{:?}", posabilities);
                                        posabilities = check_collumn(working_grid.clone(), f as u32, g as u32, posabilities);
                                        //println!("Before {:?}",posabilities);
                                        //println!("After collumn{:?}", posabilities);
                                        posabilities = check_box(working_grid.clone(), f as u32, g as u32, posabilities);
                                        //println!("After box{:?}", posabilities);
                                        if posabilities.len() == 0
                                        {
                                            println!("Failed");
                                            return None;
                                        }
                                        if posabilities.len() == 1
                                        {
                                            working_grid[f][g] = posabilities[0];
                                            println!("Placed a {}",posabilities[0]);
                                           // show_grid(&working_grid);
                                            break 'for_each_number;
                                        }
                                        else
                                        {
                                            if posabilities.len() == h
                                            {
                                                for i in posabilities
                                                    {
                                                        let mut new_grid = working_grid.clone();
                                                        new_grid[f][g]=i;
                                                        println!("Guessed at a {}",i);
                                                        //show_grid(&new_grid);
                                                        let result =do_puzzle(new_grid) ;
                                                        if result.is_some()
                                                        {
                                                           // let total = working_grid[0][0]+working_grid[0][1]+working_grid[0][2];
                                                           return result;
                                                        }

                                                    }
                                                println!("Failed");
                                                return None;

                                            }

                                        }
                                    }
                                }
                        }

                }
        }
    show_grid(&working_grid);
    let total = working_grid[0][0] *100 +working_grid[0][1] * 10 +working_grid[0][2];

    return Some(total);
}

const EULER_96_NUMBER_OF_PUZZLES:u32 =50;
fn euler_96()
{
    let mut grids = Vec::new();
    let mut total = 0;
    let contents = fs::read_to_string("p096_sudoku.txt")
        .expect("Something went wrong reading the file");
    let string_lines:Vec<&str>=contents.split_whitespace().collect();
    //println!("{:?}",string_lines);
    println!("{}",string_lines[0]);
   // let y:Vec<&str> = string_lines[0].split(',').collect();

    //let mut number_vec:Vec<u32>=Vec::new();
    //let mut exponent_vec:Vec<u32>=Vec::new();
    for f in 0 .. EULER_96_NUMBER_OF_PUZZLES
        {
            let mut grid =vec![[0 ; 9] ; 9 ];
            for g  in 0 .. 9
                {
                    let current_line = string_lines[(f as usize *11)+ g as usize +2];
                    for h in 0 .. 9
                        {
                            grid[g][h]=(current_line.as_bytes()[h]- '0' as u8) as u32 ;
                        }
                }
            grids.push(grid);




        }
    for f in grids
        {
            let new_grid = &f; //grids[1].clone();
            total += do_puzzle(new_grid.clone()).unwrap();
            println!();
            println!("total={}", total);
            println!();
        }

    //show_grid(new_grid);
//    for f in string_lines
//        {
//            let pair:Vec<&str> = f.split(',').collect();
//            let number:u32 = pair[0].parse().unwrap();
//            let exponent:u32 = pair[1].parse().unwrap();
//            number_vec.push(number);
//            exponent_vec.push(exponent);
//            println!("{} {}",number,exponent);
//        }
//    let mut x:Vec<f64> = number_vec.iter().map(|x| *x as f64).collect();
//    let mut ln_x:Vec<f64> = x.iter().map(|x| x.ln()).collect();
//    let z:Vec<f64> =ln_x.iter().zip(exponent_vec).map(|(u,v)| (*u) * v as f64).collect();
//    let mut max:f64 =0.0;

}
const EULER_97_CONSTANT_1:u64 =7830457;
const EULER_97_CONSTANT_2:u64 =28433;
fn euler_97() //Large non-Mersenne prime
{
    let mut total:u64=1;
    for f in 0 .. EULER_97_CONSTANT_1
        {
            total *=2;
            total %= 10000000000;
            if f % 1000  == 0
            {
                println!("Total={} {}",f, total);
            }
        }
    total *=EULER_97_CONSTANT_2;

    total+=1;
    total %= 10000000000;
    println!("Total={}",total);
}


trait Digits
{
    fn digits(&self) -> usize;
}
impl Digits for u64
{
    fn digits(&self) -> usize
    {
        let mut digit_count:usize=0;
        let mut working_number=*self;
        while working_number >0
            {
                working_number/=10;
                digit_count+=1;
            }
        digit_count
    }
}


fn check_for_square_matches(word_1:&Vec<char>,word_2:&Vec<char>,squares:&Vec<u64>) -> Option<u64>
{
    let mut scratch_1:Vec<Option<u64>>=vec![None;10];
    let mut scratch_2:Vec<Option<u64>>=vec![None;10];
    let mut total = 0;
    let mut real_total = 0;
    //let mut current_letter
    'squares: for f in squares
        {
            scratch_1=vec![None;10];
            scratch_2=vec![None;10];
            let mut square_vector = number_to_vec_u64(*f);
            square_vector.reverse();
            if square_vector.len() == word_1.len()
            {
              //  println!(".{}. {}",f,squares.len());
                //for each digit in the square
                'number: for (index, g) in square_vector.iter().enumerate()
                    {
                        //println!("Word 1 = {:?} {:?}",word_1,scratch_1);
                        let current_letter = word_1[index];
                        //println!("Current Letter = {} {}",current_letter,*g);
                        if (scratch_1[index] == None) | (scratch_1[index] == Some(*g))
                        {
                            scratch_1[index] = Some(*g);
                        }
                        else // less than 5 volts = 0 ohms less than 200 ma = 2000 ohms
                        {
                            //can make number match. leave
                            //println!("Cannot make a square out of {:?} 1",word_1);
                            //return None;
                            continue 'squares;
                        }
                        for h in index+1 .. word_1.len()
                            {
                                //println!("<{}>",h);
                                if word_1[h] == current_letter
                                {
                                    if scratch_1[h].is_none()
                                    {
                                        scratch_1[h] = Some(*g);
                                    }
                                    else
                                    {
                                        //println!("Cannot make a square out of {:?} 2",word_1);
                                        //return None;
                                        continue 'squares;
                                    }
                                }
                                if square_vector[h] == square_vector[index]
                                {
                                    if word_1[h]  == word_1[index]
                                    {
                                        if (scratch_1[h].is_none())
                                        {
                                            scratch_1[h] = Some(*g);
                                        } else if scratch_1[h] != Some(*g)
                                        {
                                            //println!("Cannot make a square out of {:?} 3", word_1);
                                            //return None;
                                            continue 'squares;
                                        }
                                    }
                                    else
                                    {
                                        //println!("Cannot make a square out of {:?} 4", word_1);
                                        //return None;
                                        continue 'squares;
                                    }
                                }
                            }

                        //LUCAS
                        //  for scratch in scratch_1.iter_mut().zip(word_1.iter()).skip(index).filter(|(_,letter)| **letter == current_letter) {
                        //           *scratch.0 = *g;
                        //  }
                        //println!("Moving to phase 2");
                        for h in 0..word_2.len()
                            {
                                if (h == 0) && (Some(*g) == Some(0))
                                {
                                    println!("Cannot match {:?} {:?} 7",word_2,scratch_2);
                                    continue 'squares;
                                }
                                if word_2[h] == current_letter
                                {
                                    if (scratch_2[h] == None)  ||  (scratch_2[h] == Some(*g))
                                    {
                                        scratch_2[h] = Some(*g);
                                    }
                                    else

                                    {
                                        println!("Cannot match {:?} {:?} 5",word_2,scratch_2);
                                        continue 'squares;
                                    }
                                }
                            }
                    }
                total = 0;
                println!("Building Number");
                for index in 0 .. word_2.len()
                    {
                        if scratch_2[index].is_none()
                        {
                            total=0;
                            println!("Cannot match {:?} {:?} 6",word_2,scratch_2);
                            break;
                        }
                        total *= 10;
                        total += scratch_2[index].unwrap();
                    }
                println!("{}  {}", total, f);
                if  squares.contains(&total)
                {
                    println!("SQUARE FOUND");
                    if *f > total  {total = *f ;}
                }
                else {total=0;}
            }
            if total > real_total {real_total =total;}
        }

    Some(real_total)
}


//fn euler_98() //Anagramic squares
//{
//    let mut square_vector:Vec<u64>= Vec::new();
//    let contents = fs::read_to_string("p098_words.txt")
//        .expect("Something went wrong reading the file");
//    let mut strings:Vec<&str>=contents.split(',').collect();
//    let mut square_value:Option<u64> = Some(0);
//    let mut highest:u64 = 0;
//    let trimed_strings:Vec<String>=strings.iter().map(|x| (*x).replace("\"","")).collect();
//
//    for f in 1 .. 100_000
//        {
//            let f_squared = f *f;
//            if f_squared >= 10_000_000_000
//            {
//                break;
//            }
//            square_vector.push(f_squared)
//        }
//    for f in (4 .. 10).rev()
//        {
//            println!("letters={}",f);
//            let corrrect_sized_squares:Vec<&u64>=square_vector.iter().filter(|&&x|x.digits() ==f).collect();
//            let mut same_size_strings:Vec<&String>=trimed_strings.iter().filter(|x| x.len() ==f).collect();
//            println!("{:?}",same_size_strings);
//            let mut same_size_sorted_char_vectors:Vec<Vec<char>> = Vec::new();
//            let mut same_size_char_vectors:Vec<Vec<char>> = Vec::new();
//            for g in same_size_strings
//                {
//                    let mut char_vector:Vec<char> =(*g).chars().collect();
//                    same_size_char_vectors.push(char_vector.clone());
//                    char_vector.sort();
//                    same_size_sorted_char_vectors.push(char_vector.clone());
//                   // println!("*{:?}*", char_vector);
//                }
//            for g in same_size_sorted_char_vectors.iter()
//                {
//                    let count= same_size_sorted_char_vectors.iter().filter(|&n| *n == *g).count();
//                    let location_vector:Vec<usize>= same_size_sorted_char_vectors.iter().enumerate().map(|(i,n)| if*n == *g {i} else {0}).collect();
//                    let locations:Vec<&usize> = location_vector.iter().filter(|&&x|x!=0).collect();
//
//                    //print!("count={}  ",count);
//                    //println!("*{:?}*", g);'                   //println!("*{:?}*", locations);
//
//                    if count > 1
//                    {
//                        println!("***");
//                        let mut char_vectors_sets:Vec<Vec<char>> = Vec::new();
//                        for h in locations
//                            {
//                                char_vectors_sets.push(same_size_char_vectors[*h].clone());
//
//                                println!("{:?}",same_size_char_vectors[*h]);
//                            }
//
//
//
//
//                        if count == 2
//                        {
//                            square_value =check_for_square_matches(&char_vectors_sets[0],&char_vectors_sets[1],&square_vector);
//                            if square_value.is_some()
//                            {
//                                if square_value.unwrap() > highest
//                                {
//                                    highest= square_value.unwrap();
//                                }
//                            }
//                        }
//                        else
//                        {
//                            for i in 0 .. count
//                                {
//                                    for j in i+1 ..=count
//                                        {
//                                            square_value=check_for_square_matches(&char_vectors_sets[0], &char_vectors_sets[1], &square_vector);
//                                            if square_value.is_some()
//                                            {
//                                                if square_value.unwrap() > highest
//                                                {
//                                                    highest= square_value.unwrap();
//                                                }
//                                            }
//                                        }
//                                }
//                        }
//                    }
//                }
//
//
//        }
//    println!("Highest={}",highest);
//}

fn euler_99() //Largest exponential
{
    let contents = fs::read_to_string("p099_base_exp.txt")
        .expect("Something went wrong reading the file");
    let string_lines:Vec<&str>=contents.split_whitespace().collect();
    println!("{:?}",string_lines);
    println!("{}",string_lines[0]);
    let y:Vec<&str> = string_lines[0].split(',').collect();

    let mut number_vec:Vec<u32>=Vec::new();
    let mut exponent_vec:Vec<u32>=Vec::new();
    for f in string_lines
        {
            let pair:Vec<&str> = f.split(',').collect();
            let number:u32 = pair[0].parse().unwrap();
            let exponent:u32 = pair[1].parse().unwrap();
            number_vec.push(number);
            exponent_vec.push(exponent);
            println!("{} {}",number,exponent);
        }
    let x:Vec<f64> = number_vec.iter().map(|x| *x as f64).collect();
    let ln_x:Vec<f64> = x.iter().map(|x| x.ln()).collect();
    let z:Vec<f64> =ln_x.iter().zip(exponent_vec).map(|(u,v)| (*u) * v as f64).collect();
    let mut max:f64 =0.0;
    let mut max_index=0;
    for (index,f,) in z.iter().enumerate()
        {
            if *f>max
            {
                max = *f;
                max_index = index;
            }
        }
    println!("max index ={}",max_index);


}



fn euler_100()
{
    let mut g_start:u64 = 1; //707_100_000_000  *   40;
    //for f in 40_000_000_000_000 .. 60_000_000_000_000
    for f in 1 .. 60_000
        {
            if f % 10_000_000_000 == 0
            {
                println!("f={}",f);
            }
            for g in g_start .. f
                {
                    //println!("g={}",g);
                    let total = (g  * (g-1)) as f64 / (f * (f-1)) as f64 ;
                    //println!("{}",(g as f64 / f as f64) * ((g-1) as f64 / (f-1) as f64));
//                    if (g as f64 / f as f64) * ((g-1) as f64 / (f-1) as f64) == 0.5
//                    {
//                        println!("{}/{} {}/{}",g,f,g-1,f-1);
//                    }
                    if (total >= 0.49) && (total <= 0.501)
                        {
                            let top = g * (g-1);

                            let mut bottom = f * (f-1);
                            bottom = bottom /2;
                            //println!("Possible");
//                            let mut f_prime = prime_factors(f);
//                            println!("1");
//                            let mut g_prime = prime_factors(g);
//                            println!("2");
//                            let mut f1_prime = prime_factors(f - 1);
//                            println!("3");
//                            let mut g1_prime = prime_factors(g - 1);
//                            println!("4");
//                            f_prime.append(&mut f1_prime);
//                            println!("5");
//                            g_prime.append(&mut g1_prime);
//                            println!("6");
//                            f_prime.sort();
//                            let f_prime2: Vec<&u32> = f_prime.iter().rev().collect();
//                            g_prime.push(2);
//                            g_prime.sort();
//                            let g_prime2: Vec<&u32> = g_prime.iter().rev().collect();
//                            let matching= g_prime2.iter().zip(&f_prime2).filter(|&(a, b)| a != b).count();
//                            //println!("matching {}", matching);
//                            //let dif = f_prime.drain(g_prime);
//                            //println!("{:?}",dif);
//                            //let y:Vec<&u32> = f_prime.iter().filter(|x| !g_prime.contains(x)).collect();
                            if top == bottom
                                {
                                    //println!("x={:?}",y);
                                    println!("{}/{} {}/{}", g, f, g - 1, f - 1);
                                  //  println!("{:?}/{:?}  {:?}/{:?}", g_prime2, f_prime2, g1_prime, f1_prime);
                                }
                        }
                    if total > 0.5 {break;}
                    if total < 0.5 {g_start =g;}

                }
        }
}
//#[macro_use]
//extern crate nalgebra as nalg;
//type KFDim = nalg::U2;
//type CovMatrix = nalg::MatrixMN<f64, KFDim, KFDim>;
//
//use nalg::Matrix2;
//use nalg::MatrixN;

fn euler_101() //Optimum polynomial
{
    let coeffecient_vector:Vec<f64> = vec![1.0,-1.0,1.0,-1.0,1.0,-1.0,1.0,-1.0,1.0,-1.0,1.0];
    let mut correct_vector:Vec<f64> = Vec::new();
    //let m2:MatrixN<u64,nalg::U2> = MatrixN::n    //::new(0,1,2,3);
    //println!("{}",m2);
    //let m = CovMatrix::f\
    //use rulinalg::matrix::Matrix;



    for f in 1 .. 11
        {
            let mut y:f64 =0.0;
            let f_64:i64 = f as i64;
            for g in 0 ..=10
                {
                     y += coeffecient_vector[g as usize] * num_traits::pow(f_64, g as usize) as f64;
                     //y = f as f64  *f as f64 *f as f64;

                }
            correct_vector.push(y);
            println!("y={}",y);
        }
    let mut real_total=0.0;
    for f in 1 ..=10
        {

            let mut test:Vec<f64>=Vec::new();
            for g in 1 ..= f
                {
                    for h in 0 ..f
                        {
                            test.push(num_traits::pow(g, h as usize) as f64);
                            //println!("push value {}",num_traits::pow(g, h as usize) as f64);
                        }
                }
            let mut m = Matrix::new(f,f,test);
            let mut test:Vec<f64> =  Vec::new();
            for g in 0 .. f
                {
                    test.push(correct_vector[g])
                }
            let y2 = Vector::new(test);

            let x = m.clone().solve(y2.clone()).unwrap();
            //println!("m {:?}",m);
           println!("y {:?}",y2);
            println!("x {:?}",x);
            let mut total:f64 =0.0;
            for g in 1 ..=x.size() +1
                {
                    total=0.0;
                    for h in  0 .. x.size()
                        {
                            total+=x[h] as f64  * num_traits::pow(g, h as usize) as f64;
                        }

                    print!("{} ",total);
                }
            real_total+=total;
            println!("   total={}",real_total);
        }

}
fn euler_102() //Triangle containment
{
    let triangle:Vec<f32> =  Vec::new();
    let contents = fs::read_to_string("p102_triangles.txt")
        .expect("Something went wrong reading the file");
    let string_lines: Vec<&str> = contents.split_whitespace().collect();
    let mut triangles:Vec<Vec<f32>> = Vec::new();
    //println!("{:?}", string_lines);

    let y: Vec<&str> = string_lines[0].split(',').collect();
    for f in string_lines
        {
            println!("{}", f);
            let mut new_vector:Vec<f32>=Vec::new();
            let values: Vec<&str> = f.split(',').collect();
            for g in values
                {
                    new_vector.push(g.parse().unwrap());
                }
            triangles.push(new_vector.clone());
            println!("{:?}",new_vector);
        }
//    for f in triangles
////        {
////            let mut lines:Vec<Line>=Vec::new();
////            let line:Line = Line {p:{[Point { x:f[0],y:f[1]},Point {x:f[2],y:f[3]}]}};
////            lines.push(line);
////            let line:Line = Line {p:{[Point { x:f[2],y:f[3]},Point {x:f[4],y:f[5]}]}};
////            lines.push(line);
////            let line:Line = Line {p:{[Point { x:f[4],y:f[5]},Point {x:f[0],y:f[1]}]}};
////            lines.push(line);
////
////        }
    let mut total=0;
    for f in triangles
        {
            let p1 = Point { x:f[0],y:f[1]};
            let p2 = Point { x:f[2],y:f[3]};
            let p3 = Point { x:f[4],y:f[5]};

            let alpha:f32 = ((p2.y - p3.y)*(0.0 - p3.x) + (p3.x - p2.x)*(0.0 - p3.y)) /
            ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
            let beta:f32 = ((p3.y - p1.y)*(0.0 - p3.x) + (p1.x - p3.x)*(0.0 - p3.y)) /
            ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
            let gamma:f32 = 1.0 - alpha - beta;
            if (alpha>0.0) && (beta >0.0) && (gamma > 0.0) {total+=1;}
        }
    print!("Total={}",total);

}


fn special_sum_check(set_to_check:Vec<u32>) -> bool
{
    let list_of_subsets=subsets(set_to_check.clone());
    for f in &list_of_subsets
        {
            if (f.len() != 0) && (f.len() != set_to_check.len())
            {
                // println!("{:?}",f);
                for g in &list_of_subsets
                    {
                        if f != g
                        {
                            let sum_f = f.iter().fold(0, |a, &b| a + b);
                            let sum_g = g.iter().fold(0, |a, &b| a + b);

                            if sum_f == sum_g
                            {
                                return false;
                            }
                            if (g.len() > f.len()) & (sum_f > sum_g)
                            {
                                return false;
                            }
                            if (f.len() > g.len()) & (sum_g > sum_f)
                            {
                                return false;
                            }
                        }
                    }
            }
        }
    //println!("no equal sums");
    true

}
fn euler_103() //Special subset sums: optimum
{
    //let test_set = vec![20,31,38,39,40,42,45];
    let test_set = vec![1,2,3,4];
    let mut possible_sets:Vec<Vec<u32>>=Vec::new();
    let mut new_possible_sets:Vec<Vec<u32>>=Vec::new();
    possible_sets.push(test_set.clone());
    //for h in possible_sets
    let vector_length = test_set.len();
    for f in 0 .. test_set.len()
        {
           for g in 0 .. 5
               {
                   for h in &possible_sets
                       {
                           let mut new_set = h.clone();
                           new_set[f]-=2;
                           new_set[f]+=  g;
                           new_possible_sets.push(new_set);
                       }
               }
            possible_sets=new_possible_sets;
            new_possible_sets= Vec::new();
        }
    for f in possible_sets.iter_mut()
        {
            f.sort();
            f.dedup();
            if f.len() ==  vector_length
            {
                new_possible_sets.push(f.clone());
            }
        }
    possible_sets=new_possible_sets;
    possible_sets.sort();

    for f in &possible_sets
        {
            if special_sum_check(f.clone())
            {
                println!("{:?}", f);
            }

        }
    println!("possible set length {}",possible_sets.len());
    //special_sum_check(test_set);
}


//let fib_cache:Vec<u32>=vec:new();
fn calc_fib(rank:u32) -> BigUint
{
    if rank >= 100
        {
            if rank % 2 != 0
                {
                    let half_rank = (rank+1) /2;
                    let half_rank_minus = half_rank -1;
                    let new_fib_1 = calc_fib(half_rank);
                    let new_fib_2 = calc_fib(half_rank_minus);
                    return (new_fib_1.clone() * new_fib_1) + (new_fib_2.clone() * new_fib_2);
                }
            else
            {
                let half_rank = (rank) /2;
                let half_rank_minus = half_rank -1;
                let new_fib_1 = calc_fib(half_rank);
                //println!("fib1 {}",new_fib_1);
                let new_fib_2 = calc_fib(half_rank_minus) * 2_u32.to_biguint().unwrap();
                //println!("fib2 {}",new_fib_2);
                return (new_fib_1.clone() + new_fib_2.clone()) * new_fib_1;
            }
        }
    else
    {
        return  VEC[rank as usize].clone();
    }


}

fn populate_fib_cache() -> Vec<BigUint>
{
    let mut return_vector:Vec<BigUint> = Vec::new();
    let  zero: BigUint = Zero::zero();
    let mut first: BigUint = One::one();
    let mut second: BigUint = One::one();
    return_vector.push(zero.clone());
    return_vector.push(first.clone());
    return_vector.push(second.clone());
    for _f in 3 ..100
        {
            let scratch = first;
            first = second.clone();
            second = scratch + second;
            return_vector.push(second.clone());
        }
    return_vector
}



lazy_static! {
    static ref VEC: Vec<BigUint> = {
        let m = populate_fib_cache();
        m
    };
}
fn euler_104() //Pandigital Fibonacci ends
{
    let mut first: u64 = 1; //One::one();
    let mut second: u64 = 1; //One::one();
    for g in 0 .. 100
        {
            let this_fib=calc_fib(g);
            println!("{} {}",g,this_fib);
        }
    for f in 3 ..1_000_000_000
        {
            let scratch = first;
            first = second.clone();
            second = scratch + second;

           // println!("{} {:?}",f,number_vec);
            if second >=100_000_000
            {
                second = second % 1_000_000_000;
                let number_vec=number_to_vec_u64(second.clone());
                let last_digits = number_vec.clone(); //&number_vec[number_vec.len()-9..number_vec.len()];
                //println!("{} {} {:?}",f,second.clone(),last_digits);
                let mut trimmed_last_digits:Vec<&u64>=last_digits.iter().filter(|&x| *x!=0).collect();
                trimmed_last_digits.sort();
                trimmed_last_digits.dedup();
                if trimmed_last_digits.len() == 9
                {
                    println!("Pandigital {} ",f);
                    let full_fib = calc_fib(f);
                    let full_fib_vector = big_uint_to_vec(full_fib);
                    let first_digits = &full_fib_vector[full_fib_vector.len()-9..full_fib_vector.len()];
                    println!("{} {:?}",f,first_digits);
                    //println!("{:?}",trimmed_last_digits);
                    let mut trimmed_first_digits:Vec<&u32>=first_digits.iter().filter(|&x| *x!=0).collect();
                    trimmed_first_digits.sort();
                    trimmed_first_digits.dedup();
                    println!("{:?}",trimmed_first_digits);
                    if trimmed_first_digits.len() == 9
                    {
                        println!("Pandigital {} ",f);
                        break;
                    }
                }


            }
        }
}



fn euler_105() //Special subset sums: testing
{
    let contents = fs::read_to_string("p105_sets.txt")
        .expect("Something went wrong reading the file");
    let string_lines: Vec<&str> = contents.split_whitespace().collect();
    let mut test_vectors:Vec<Vec<u32>>=Vec::new();
    for f in string_lines
        {
            let mut new_vector:Vec<u32>=Vec::new();
            let values: Vec<&str> = f.split(',').collect();

            for g in values
                {
                    new_vector.push(g.parse().unwrap());
                }
            test_vectors.push(new_vector.clone());
            println!("{:?}",new_vector);
        }
    println!("test vectors size={}",test_vectors.len());
    let mut vector_sum = 0;
    for f in test_vectors
        {
            if special_sum_check(f.clone())
            {
                vector_sum+=f.iter().fold(0, |a, &b| a + b);
                println!("{:?}", f);
            }
        }
    println!("sum={}", vector_sum);
}


fn euler_106()  //Special subset sums: meta-testing
{
    let vector_list:Vec<Vec<u32>>= Vec::new();
   // let test_vector = vec![1,2,3,4,5,6,7];
    let test_vector = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    //let test_vector = vec![1,2,3,4];
    let test_vector_length = test_vector.len();
    for f in 0 .. test_vector_length
        {

        }
    let vector_subsets=subsets(test_vector.clone());
    //let vector_subsets_filtered:Vec<Vec<u32>>=vector_subsets.iter().filter(|x| x.len() !=4).collect();
    let mut working_set = Vec::new();
    for f in vector_subsets
        {
            if (f.len() !=0)  && f.len() != test_vector_length
                {
                    //println!("f={:?}  ", f);
                    let mut new_set = test_vector.clone();
                    new_set.retain(|x| !f.contains(x));
                    //println!("{:?}", new_set);
                    let mut new_vector_subsets=subsets(new_set.clone());
                    for g in new_vector_subsets.iter_mut()
                        {
                            if g.len() > 0
                            {
                                g.sort();
                                //println!("{:?} {:?}",f,g);
                                let mut set_of_sets:Vec<Vec<u32>> =Vec::new();
                                set_of_sets.push(f.clone());
                                set_of_sets.push((*g).clone());
                                set_of_sets.sort();
                                //println!("{:?}",set_of_sets);
                                working_set.push(set_of_sets);
                            }
                        }
                }

        }
    working_set.sort();
    working_set.dedup();
    working_set.retain(|x| x[0].len()!=1 || x[1].len()!=1 );  //if both have one element
//    working_set.retain(|x| x[0].len() !=1 ||  x[0][0]!= 1 );
    working_set.retain(|x| x[0].len()== x[1].len() );  //if they are same size
    //working_set.retain(|x| !(x[0][x[0].len()-1] < x[1][0]));
    //working_set.retain(|x| x[0].len() >2 || (x[0][0] < x[1][0]  &&  x[0][x[1].len()-1] > x[1][x[1].len()-1]));
   // working_set.retain(|x| !(x[0].len() ==2 && x[0][0] < x[1][0]  &&  x[0][1] < x[1][1]));
   // working_set.retain(|x| !(x[0].len() ==3 && x[0][0] < x[1][0]  &&  x[0][1] < x[1][1] ));
    let mut count =0;
    for f in working_set.iter()
        {
            let zipped_sets=f[0].iter().zip(f[1].clone()).filter(|&(a, b)| *a < b).count() == f[0].len();
            println!("{:?}",zipped_sets);
            if !zipped_sets {count+=1;}
        }
        println!("count={}",count)
//    for (index,f) in working_set.iter().enumerate()
//        {
//            //for g in \\
//            print!("{} {:?}",index+1,f);
//            let sum1:u32 = ((*f)[0]).iter().sum();
//            let sum2:u32 = ((*f)[1]).iter().sum();
//            if sum1 == sum2
//            {
//                println!(" equal");
//            }
//            else {println!();}
//        }



}
const EULER_107_GRID_SIZE:usize = 40;
const EULER_107_TEST_GRID:&str ="-,16,12,21,-,-,-,16,-,-,17,20,-,-,12,-,-,28,-,31,-,21,17,28,-,18,19,23,-,20,-,18,-,-,11,-,-,31,19,-,-,27,-,-,-,23,11,27,-";


fn euler_check_path(this_grid:[[u32; EULER_107_GRID_SIZE]; EULER_107_GRID_SIZE]) -> u32
{
    let mut connection_check:[u32; EULER_107_GRID_SIZE] =[0;EULER_107_GRID_SIZE];
    let mut path_vector:[u32; EULER_107_GRID_SIZE] =[1;EULER_107_GRID_SIZE];

    for f in 0 .. EULER_107_GRID_SIZE
        {
            let mut connection_vector:[u32; EULER_107_GRID_SIZE] =[0;EULER_107_GRID_SIZE];
            let mut connection_count=0;
            let mut path_queue: VecDeque<u32> = VecDeque::new();
            path_queue.push_back(f as u32);
            'while1: while !path_queue.is_empty()
                {
                    let node = path_queue.pop_front().unwrap();
                    //println!("Node ={}",node);
                    let connections =this_grid[node as usize];
                    connection_vector[f]=1;
                    connection_count+=1;
                    for (index,g,) in connections.iter().enumerate()
                        {

                                if *g != 0
                                {
                                    if index < f
                                    {
//                                        for h in 0 .. EULER_107_GRID_SIZE
////                                            {
////                                                connection_vector[h] =1;
////                                            }
                                        connection_count=EULER_107_GRID_SIZE;
                                        break 'while1;
                                    }
                                    if connection_vector[index] == 0
                                    {
                                        path_queue.push_back(index as u32)
                                    }
                                    connection_vector[index] = 1;
                               //let x:u32=connection_vector.iter().sum();
                                if  connection_count== EULER_107_GRID_SIZE //(x as usize == connection_vector.len())
                                {
                                    break 'while1;
                                }
                                }
                        }


                }
            //let x:u32=connection_vector.iter().sum();
            //if (x as usize == connection_vector.len())
            if  connection_count== EULER_107_GRID_SIZE
            {
                connection_check[f] =1;
            }
           // for g in f .. EULER_107_GRID_SIZE

        }
    let x:u32=connection_check.iter().sum();
    if (x as usize == connection_check.len())
    {
        let mut flat_vec:Vec<u32> = Vec::new();
        for i in this_grid.iter() {for j in i.iter() {flat_vec.push(*j) }}
//        let flattened:= this_grid.iter().iter().collect();
//         //   .iter().flatten().collect::<Vec<u32>>();
//        //let y =this_grid.to_vec().iter();
        let x=flat_vec.iter().fold(0, |a, &b| a + b);
       // println!("x={}",x/2);
        (x/2) as u32
    }
    else
    {
        //println!("Not Connected");
        0
    }

}

fn strip_network_2(this_grid:[[u32; EULER_107_GRID_SIZE]; EULER_107_GRID_SIZE]) -> u32
{
    let mut best_path = 0;

    let mut my_grid = this_grid.clone();
    let this_vec = my_grid.clone().to_vec();
    let mut flat_vec:Vec<u32> = Vec::new();
    for i in my_grid.iter() {for j in i.iter() {flat_vec.push(*j) }}
    let highest_connection = flat_vec.iter().max().unwrap();
    println!("Highest connection ={}",highest_connection);
    for f in (01..= *highest_connection).rev()
        {
            for g in 0..EULER_107_GRID_SIZE
                {
                    for h in g..EULER_107_GRID_SIZE
                        {
                            if my_grid[g][h] == f
                            {
                                let mut new_grid = my_grid;
                                new_grid[g][h] = 0;
                                new_grid[h][g] = 0;
                                let path_cost = euler_check_path(new_grid);
                                if path_cost != 0
                                {
                                    my_grid = new_grid.clone();
                                    best_path=path_cost;
                                }
                            }
                        }
                }
        }
    if best_path != 0
    {
        println!("bp={}", best_path);
    }
    best_path
}

fn strip_network(this_grid:[[u32; EULER_107_GRID_SIZE]; EULER_107_GRID_SIZE]) -> u32
{
    let mut best_path=0;
    for f in 0 .. EULER_107_GRID_SIZE
        {
            for g in f .. EULER_107_GRID_SIZE
                {
                    if this_grid[f][g] != 0
                    {
                        let mut new_grid = this_grid.clone();
                        new_grid[f][g] = 0;
                        new_grid[g][f] = 0;
                        let path_cost = euler_check_path(new_grid);
                        if path_cost !=0
                        {
                            if ( path_cost < best_path) || (best_path==0)
                            {
                                best_path = path_cost;
                            }
                            let new_path_cost = strip_network(new_grid);
                            if new_path_cost != 0
                            {
                                if (new_path_cost < best_path) || (best_path == 0)  { best_path =new_path_cost}
                               // println!("bp={}",best_path);
                            }
                        }

                    }

                }
        }
    if best_path != 0
    {
        println!("bp={}", best_path);
    }
    best_path

}




fn euler_107() //Minimal network
{
    //p107_network
    let mut grid:[[u32; EULER_107_GRID_SIZE]; EULER_107_GRID_SIZE] = [[0; EULER_107_GRID_SIZE]; EULER_107_GRID_SIZE];
    let contents:String = EULER_107_TEST_GRID.to_string();
    let now = Instant::now();
    let contents = fs::read_to_string("p107_network.txt")
      .expect("Something went wrong reading the file");
    let string_bits: Vec<&str> = contents.split(|c| c == ',' || c == '\n').collect();
    let mut string_bits_iter = string_bits.iter();
    for f in 0 .. EULER_107_GRID_SIZE
        {
            for g in 0 .. EULER_107_GRID_SIZE
                {
                    let next_char = string_bits_iter.next();
                    let num_result = (*next_char.unwrap()).parse::<u32>() ;
                    let mut num=0;
                    match num_result
                        {
                            Ok(v) => num=v,
                            Err(e) =>  num= 0,
                        }
                    println!("{} {} {} ",f,g,num);
                    grid[f][g]=num;
                }
        }
    println!("before check");
    let initial_path=euler_check_path(grid);
    let best_path =strip_network_2(grid);
    println!("Best Path={} {} {}",best_path,initial_path,initial_path-best_path);
    println!("{}",now.elapsed().as_millis());

}

fn euler_108() //Diophantine reciprocals I
{
    let factors:Vec<u64> = vec![2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,5,5,5,7,7,11,13,17,19,23];
    let factors:Vec<u64> = vec![2,2,3,3,5,5,5,7];
    let factors:Vec<u64> = vec![11,13,17,19,23];
    let factors:Vec<u64> = vec![2,2,2,2,3,3,3,5,5,7,7,11,11,13,17,19,21];
    let mut factors_subsets = subsets_64(factors);
    let mut factors_subsets:Vec<&Vec<u64>>=factors_subsets.iter().filter(|x| (*x).len() >1).collect();
//    for f in factors_subsets.iter()
//        {
//            println!("{:?}",f);
//        }
    factors_subsets.sort();
    factors_subsets.dedup();
    //factors_subsets.sort_by(|a:&Vec<u64>,b:&Vec<u64>| (*a).iter().product().cmp((*b).iter().product()).unwrap());
    let mut products:Vec<u64> = Vec::new();
    for f in factors_subsets.iter()
        {
            products.push(f.iter().product());
        }
    products.sort();
    println!("{}",factors_subsets.len());
    let mut highest_count =1;
    for n in products
        {
            let n_product:u64 = n; //n.iter().product();
            let target:Ratio<i64> = Ratio::new(1,n_product as i64);
            let mut count=0;

            for f in n_product+1 ..= n_product *2
                {
                    let first:Ratio<i64> = Ratio::new(1,f as i64);
                    let second:Ratio<i64> = target - first;
                    if *second.numer() == 1
                    {
                        count+=1;
                       // print!("({} {} {}) ",target.denom(),*first.denom(),*second.denom())
                    }
                    //println!("{} - {} = {}",target,first,second)

                }
            //println!();
            let dif:i32 = (count-1000);
            if count > highest_count
            //if (dif > 0) && (dif < highest_count)
                {
                    highest_count = count;
                    let factors = prime_factors(n);
                    //if count > highest_count {highest_count=count;}
                    println!("n {}, count {} highest_count {} {:?}", n_product, count, highest_count,factors);
                }

        }
}


const EULER_109_DARTS_SINGLE:[u32;21] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,25];
const EULER_109_DARTS_DOUBLE:[u32;21] = [200,400,600,800,1000,1200,1400,1600,1800,2000,2200,2400,2600,2800,3000,3200,3400,3600,3800,4000,5000];
const EULER_109_DARTS_TRIPLE:[u32;20] = [30000,60000,90000,120000,150000,180000,210000,240000,270000,300000,330000,360000,390000,420000,450000,480000,510000,540000,570000,600000];
fn euler_109() //Darts
{
    let single_throw:Vec<u32> =  EULER_109_DARTS_DOUBLE.to_vec();
    let mut double_throw:Vec<Vec<u32>> = Vec::new();
    let triple_throw:Vec<u32> = Vec::new();
    for f in EULER_109_DARTS_SINGLE.to_vec().into_iter().chain((EULER_109_DARTS_DOUBLE.to_vec().clone()).into_iter().chain((EULER_109_DARTS_TRIPLE.to_vec().clone()).into_iter()))
        {

            for g in EULER_109_DARTS_DOUBLE.to_vec()
                {
                    let new_throw:Vec<u32>=vec![f,g];
                    //let scrubbed_new_throw:Vec<u32> =new_throw.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();
                    //let scrubbed_new_throw2:Vec<u32> =scrubbed_new_throw.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();
                    double_throw.push(new_throw);
                }
        }
   // double_throw.sort();
  // double_throw.dedup();
    let mut double_throw2:Vec<Vec<u32>> = Vec::new();
    for f in double_throw
        {
            let scrubbed_new_throw:Vec<u32> =f.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();
            let scrubbed_new_throw2:Vec<u32> =scrubbed_new_throw.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();
            double_throw2.push(scrubbed_new_throw2);
        }

    let mut scratch:Vec<Vec<u32>> = Vec::new();
    for f in EULER_109_DARTS_SINGLE.to_vec().into_iter().chain((EULER_109_DARTS_DOUBLE.to_vec().clone()).into_iter().chain((EULER_109_DARTS_TRIPLE.to_vec().clone()).into_iter()))
        {

            for g in EULER_109_DARTS_SINGLE.to_vec().into_iter().chain((EULER_109_DARTS_DOUBLE.to_vec().clone()).into_iter().chain((EULER_109_DARTS_TRIPLE.to_vec().clone()).into_iter()))
                {
                    let mut new_throw:Vec<u32>=vec![f,g];
                    new_throw.sort();
                    scratch.push(new_throw);
                }
        }
    scratch.sort();
    scratch.dedup();

    let mut final_set:Vec<Vec<u32>> = Vec::new();
    for f in scratch
        {
            for g in EULER_109_DARTS_DOUBLE.to_vec()
                {
                    let mut new_throw:Vec<u32>=vec![f[0],f[1],g];
                    let scrubbed_new_throw:Vec<u32> =new_throw.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();
                    let scrubbed_new_throw2 =scrubbed_new_throw.iter().map(|x| if *x > 100 {*x /100} else {*x}).collect();

                    final_set.push(scrubbed_new_throw2);
                }
        }
    let mut total=0;
    for f in final_set.clone()
        {
            let vector_sum:u32 = f.iter().sum();
            print!("final {:?} ",f);
            if vector_sum < 100
            {
                total+=1;
                println!("Hit")
            }
            else
            {
                println!();
            }

        }
    for f in double_throw2.clone()
        {
            let vector_sum:u32 = f.iter().sum();
            print!("double {:?} ",f);
            if vector_sum < 100
            {
                total+=1;
                println!("Hit")
            }
            else
            {
                println!();
            }

        }
//    for f in single_throw.clone()
//        {
//            let mut temp=f;
//            if temp > 100 { temp /=100;}
//            if temp > 100 { temp /=100;}
//            println!("{:?}",temp);
//        }
    total+=single_throw.len();
    println!("total={}",total);
    println!("total={}",final_set.len()+double_throw2.len()+single_throw.len());
}





lazy_static!
{
    static ref BAR_CACHE: Mutex<Vec<Option<u64>>> = Mutex::new(vec![]);
}

fn calc_bars(length:usize) -> u64
{
    let mut total =0;
    if length < EULER_114_BLOCK_SIZE {return 0;}
    if BAR_CACHE.lock().unwrap()[length].is_some()
    {
        BAR_CACHE.lock().unwrap()[length].unwrap()
    }
    else {
        for size in (EULER_114_BLOCK_SIZE..=length).rev()
            {
                for position in 0..=length - size
                    {
                        total += 1;
                        //if position > 0 { total += calc_bars(position - 1); }
                        if position + size < length { total += calc_bars(length - (position + size + 1)); }
                    }
            }
        println!("Cache miss on {} of {}",length,total);
        BAR_CACHE.lock().unwrap()[length]=Some(total);
        total
    }
}

const EULER_110_PRIMES:[u64;10] = [2,3,5,7,11,13,17,19,23,29];
//4042913
//11175604627086900000
//1100494818423000000
//335388897043200
//867385078560000
//279490747536000
//260858031033600
//2021187649141140480
//62382334850035200
//51985279041696000
//32346395848166400
//24259796886124800
//20216497405104000
//19234553245427520
//14247817218835200
//10685862914126400
//9350130049860600



fn euler_110()  //Diophantine reciprocals II
{
    //x=km(m+n), y=kn(m+n), z=kmn,
    let mut highest_count=0;
    //for f in(24094029960*31..2_000_000_000*11*7*13*17*19*23).step_by(360*11*7*13*17*19*23*29*31)
    //for g in 1 .. 10_000_000_000
    let mut g=0;
    while true
        {
            g+=1;
            let mut g_vec =number_to_vec_u64(g);
            for h in (0 ..g_vec.len()).rev()
                {
//                    if g_vec[h]==7
//                    {
//                        let temp = 3 * num_traits::pow(10, h);
//                        g += temp;
//                        g_vec =number_to_vec_u64(g);
//                        break;
//                    }
                }
            let mut total = 1;
            for h in 0 .. g_vec.len()
                {
//                    if g_vec[h] >5
//                    {
//                        g+=
//                    }
                    let new_value = num_traits::pow(EULER_110_PRIMES[h],g_vec[h] as usize);
                    total=total*new_value;
                }

          // let f =total*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2;
            let f =total*2*3*5*7*11*13*17*19*23*29*31*37;
            //let f =1260;
            if f >= 9350130049860600 { println!(" ."); continue;}
            //println!();
            let f_f : f64 = f as f64;
            let f_cbrt_f = f_f.cbrt();
            let f_cbrt=f_cbrt_f as u64;
            let mut count=0;
            let mut target_vector:Vec<u64> = Vec::new();
            for k in 1 ..=f_cbrt
                {
                    if f%k !=0 {continue;}
                    let temp = f/k;
                    let f_temp : f64 = temp as f64;
                    let temp_sqrt_f = f_temp.sqrt();
                    let temp_sqrt=temp_sqrt_f as u64;
                    for m in 1 ..=temp_sqrt
                        {
                            if temp%m !=0 {continue;}
                            //count+=1;
                            let n = temp/m;
                            let mut  factor_vector:Vec<u64> = Vec::new();
                            let base = k *(m+n);
                            let x = m * base;
                            let y = n * base;
                            //let x = k*m*(m+n);
                            //let y = k*n*(m+n);
                            if x<y
                            {
//                                factor_vector.push(x);
//                                factor_vector.push(y);
                                target_vector.push(x);
                            }
                            else
                            {
//                                factor_vector.push(y);
//                                factor_vector.push(x);
                                target_vector.push(y);
                            }
//                            factor_vector.push(x);
//                            factor_vector.push(y);
//                            factor_vector.sort();
                            //target_vector.push(factor_vector);

                            let mut  factor_vector:Vec<u64> = Vec::new();
                            let base = m * (n+k);
                            let x = n * base;
                            let y = k * base;
                            //let x = m*n*(n+k);
                            //let y = m*k*(n+k);
                            if x<y
                            {
//                                factor_vector.push(x);
//                                factor_vector.push(y);
                                target_vector.push(x);
                            }
                            else
                            {
//                                factor_vector.push(y);
//                                factor_vector.push(x);
                                target_vector.push(y);
                            }
//                            factor_vector.push(x);
//                            factor_vector.push(y);
//                            factor_vector.sort();
                            //target_vector.push(factor_vector);

                            let mut  factor_vector:Vec<u64> = Vec::new();
                            let base =  n * (k+m);
                            let x = m * base;
                            let y = k * base;
                           // let x = n*m*(k+m);
                            //let y = n*k*(k+m);
                            if x<y
                            {
                                target_vector.push(x);
//                                factor_vector.push(x);
//                                factor_vector.push(y);
                            }
                            else
                            {

                                target_vector.push(y);
//                                factor_vector.push(y);
//                                factor_vector.push(x);
                            }
//                            factor_vector.push(x);
//                            factor_vector.push(y);
//                            factor_vector.sort();
                          //  target_vector.push(factor_vector);


                            //  println!("1/{} + 1/{} = 1/{}",k*m*(m+n),k*n*(m+n),k*m*n);
//                            if n!=m
//                            {
//                                count+=1;
//                                println!("{}      {},{},{}",count,k,n,m);
//                                println!("1/{} + 1/{} = 1/{}",k*n*(m+n),k*m*(m+n),k*m*n);
//                            }

                        }
                }
            target_vector.sort();
            target_vector.dedup();
            count=target_vector.len();
            println! ("total={} {} {:?} {} {}",total, g,g_vec,count,highest_count);
            if count > highest_count
            {

//                for i in &target_vector
//                    {
//                        println!("{:?}", i);
//                    }
                highest_count =count;
                println!("n={}, count={}, highest count={}",f,count,highest_count);
            }

        }
}


const EULER_DIGIT_COUNT:u64=10;
fn euler_111()  //Primes with runs
{
    let mut total = 0;
    for f in 0 ..=9   //current digit
        {
            let mut marker_digit =0;
            if f == 0 {marker_digit =1;}
            let mut test_vector:Vec<u64> = Vec::new();

            for g in 0 .. EULER_DIGIT_COUNT
                {
                    test_vector.push(marker_digit);  //test vector is now full of the marker digit
                }

            for g in (1 ..EULER_DIGIT_COUNT).rev()  //occurrences
                {
                    let mut vector_of_numbers:Vec<Vec<u64>> =Vec::new();
                    let mut new_test_vector = test_vector.clone();
                    for h in 0 ..g
                        {
                           // println!("h={}",h);
                            new_test_vector[h as usize]=f;  //fill the correct spots with our digit
                        }
                    let mut perms:Vec<u64>=Vec::new();
                    find_permutations_u64(&mut new_test_vector,EULER_DIGIT_COUNT,&mut perms);
//                    for h in perms.iter()
//                        {
//                            println!("*{:?}",h);
//                        }
                    //let mut filtered_subset:Vec<&Vec<u32>> = test_subsets.iter().filter(|x| x.len() == EULER_DIGIT_COUNT as usize).collect();
                    for h in perms
                        {
                            let mut test_number_vector= number_to_vec_u64(h);
                            //println!("*{:?}",test_number_vector);
                            while test_number_vector.len() < EULER_DIGIT_COUNT as usize
                                {
                                    test_number_vector.push(0); //insert(0,0);
                                }
                            vector_of_numbers.push(test_number_vector);
                            //println!("{:?}",test_number_vector);
                        }
                    vector_of_numbers.sort();
                    vector_of_numbers.dedup();
//                    for h in vector_of_numbers.iter()
//                        {
//                            println!(">{:?}",h);
//                        }
                    let mut working_vector = vector_of_numbers.clone();
                    for i in working_vector.iter_mut()
                        {
                            for h in 0 .. EULER_DIGIT_COUNT
                                {
                                    if i[h as usize] == marker_digit {i[h as usize] = 10;}
                                }
                           // println!("i {:?}",i);
                        }
                    let mut final_vector:Vec<Vec<u64>>=Vec::new();
                    while !working_vector.is_empty()
                        {
                            let mut new_working_vector:Vec<Vec<u64>>=Vec::new();
                            for j in working_vector
                                {
                                   // println!("<{:?}>",j);
                                    //if j.binary_search(&10).is_ok()
                                    if j.contains(&10)
                                        {
                                          //  println!("+");
                                            let mut index =0;
                                            for k in 0 .. EULER_DIGIT_COUNT
                                                {
                                                    if j[k as usize] == 10
                                                    {
                                                        index = k;
                                                        break;
                                                    }
                                                }
                                           // let index = j.binary_search(&10).unwrap();
                                            for h in 0 ..=9
                                                {
                                                    if h != f
                                                        {
                                                            let mut temp_vec=j.clone();
                                                            temp_vec[index as usize]=h;
                                                            new_working_vector.push(temp_vec);
                                                        }
                                                }


                                        }
                                    else
                                    {
                                        let temp_vec=j.clone();
                                     //   println!("-");
                                        //println!("{:?}",temp_vec);
                                        final_vector.push(temp_vec);
                                    }
                                }
                            //println!("=======");
                            working_vector=new_working_vector;
                        }
                     let mut prime_count=0;
                     let mut prime_total=0;
                    for h in final_vector
                        {
                            if h[0] != 0
                            {
                                let test_number = vec_to_number_u64(h);
                                if prime_check_u64(test_number) { prime_count += 1; prime_total+=test_number}
                                //println!("number={} digit={} occurrences={} prime count={} total={}", test_number, f, g, prime_count,prime_total);
                            }
                        }
                    println!("digit={} occurrences={} prime count={} total={}", f, g, prime_count,prime_total);

                    //println!("*******");
                    total+=prime_total;
                    if prime_count > 0 {break;}
                }

        }
    println!("total={}",total);


}




fn euler_112()
{
    let mut non_bouncy:u32 = 99;
    let mut bouncy:u32 = 0;
    let mut increasing:bool = false;
    for f in 100 .. 2_000_000
        {
            let number_vec:Vec<u32>=number_to_vec(f);
            let mut sorted_number_vec=number_vec.clone();
            sorted_number_vec.sort();
            if !(sorted_number_vec == number_vec)
                {
                    sorted_number_vec.reverse();
                    if !(sorted_number_vec == number_vec)
                        {
                            bouncy+=1;

                        }
                    else
                    {
                        non_bouncy+=1;
                    }
                }
            else
            {
                non_bouncy+=1;
            }
            println!("{},{}",f,(bouncy*100)/(bouncy+non_bouncy));
            if (bouncy*100)/(bouncy+non_bouncy) >= 99 {break};
        }

}

const EULER_113_SIZE:usize = 101;

fn recursive_113(this_number_vector:Vec<u8>,size:usize,decreasing:bool) -> u64
{
    let mut count =0;let length=this_number_vector.len();
    if length < size
    {
        for f in 0..=9
            {
                if decreasing
                {
                    if this_number_vector[length - 1] == 9
                    {
                       // println!("-{:?}",this_number_vector);
                        count=1;
                    }
                    else if this_number_vector[length - 1] <= f
                    {
                        let mut new_number_vector = this_number_vector.clone();
                        new_number_vector.push(f);
                        count += recursive_113(new_number_vector, size, decreasing);
                    }
                }
                else
                {
                    if this_number_vector[length - 1] == 0
                    {
                       // println!("+{:?}",this_number_vector);
                        count=1;
                    }
                    else if this_number_vector[length - 1] >= f
                    {
                        let mut new_number_vector = this_number_vector.clone();
                        new_number_vector.push(f);
                        count += recursive_113(new_number_vector, size, decreasing);
                    }
                }
            }
    }
    else
    {
       // println!("{:?}",this_number_vector);
        count=1;
    }
    count
}
fn euler_113_old()  //Non-bouncy numbers
{
    let mut number_vector:Vec<u8> = Vec::new();
    let mut count: u64 =0;
    let mut now = Instant::now();
    for g in 2 .. EULER_113_SIZE
        {
            let mut now = Instant::now();
            for f in 1..=9
                {
                    let mut new_vector = number_vector.clone();
                    new_vector.push(f);
                    count += recursive_113(new_vector.clone(),g,true);
                    count += recursive_113(new_vector,g,false);

                }
            count-=9;  //remove overlap
            println!("count {} {} {}", g, count,now.elapsed().as_secs());
        }
            count+=9;  //add back in the single digit numbers
            println!("count {}", count);
}

fn euler_113()  //Non-bouncy numbers
{
    let mut decreasing_cache_vector:Vec<u64> =vec![1,1,1,1,1,1,1,1,1,1];
    let mut increasing_cache_vector:Vec<u64> =vec![1,1,1,1,1,1,1,1,1,1];
    let mut sum = 9;
    for f in 2 .. EULER_113_SIZE
        {
            let mut increasing_temp_vector:Vec<u64>=vec![0,0,0,0,0,0,0,0,0,0];
            let mut decreasing_temp_vector:Vec<u64>=vec![0,0,0,0,0,0,0,0,0,0];
            //increasing numbers
            for g in 1 ..=9
                {
                    for h in g ..=9
                        {
                            increasing_temp_vector[g]+=increasing_cache_vector[h];
                        }
                }

            //decreasing numbers
            for g in 0 ..=9
                {
                    for h in 0 ..=g
                        {
                            decreasing_temp_vector[g]+=decreasing_cache_vector[h];
                        }
                }
            decreasing_cache_vector=decreasing_temp_vector;
            increasing_cache_vector=increasing_temp_vector;
            //println!("{:?} {:?}",decreasing_cache_vector,increasing_cache_vector);
            let my_sum_d:u64 = decreasing_cache_vector.iter().sum();
            println!("D: {:?} {}",decreasing_cache_vector,my_sum_d);
            let my_sum_i:u64 = increasing_cache_vector.iter().sum();
            println!("I: {:?}{}",increasing_cache_vector,my_sum_i);
            sum=sum+my_sum_d+my_sum_i-10;
            println!("{} {}",f,sum)
        }
}
const EULER_114_SIZE:usize =167;
const EULER_114_BLOCK_SIZE:usize =50;
//Note this also solved 115
fn euler_114()  //Counting block combinations I
{
    for _f in 0 ..= EULER_114_SIZE
        {
            let option_value:Option<u64> =None;
            BAR_CACHE.lock().unwrap().push(option_value);
        }
    let new_total = calc_bars(EULER_114_SIZE);
    println!("new total={}",new_total+1);
}


fn calc_bars2(length:usize,block_size:usize) -> u64
{
    let mut total =0;
    if length < block_size {return 0;}
    if BAR_CACHE.lock().unwrap()[length].is_some()
    {
        BAR_CACHE.lock().unwrap()[length].unwrap()
    }
    else {
                let size = block_size;
                for position in 0..=length - size
                    {
                        total += 1;
                        //if position > 0 { total += calc_bars(position - 1); }
                        if position + size < length { total += calc_bars2(length - (position + size),block_size); }
                    }
        println!("Cache miss on {} of {}",length,total);
        BAR_CACHE.lock().unwrap()[length]=Some(total);
        total
    }
}


const EULER_116_SIZE:usize =50;
fn euler_116() //Red, green or blue tiles
{
    for _f in 0 ..= EULER_116_SIZE
        {
            let option_value:Option<u64> =None;
            BAR_CACHE.lock().unwrap().push(option_value);
        }
    let mut new_total = calc_bars2(EULER_116_SIZE,2);
    println!("new total={}",new_total);
    BAR_CACHE.lock().unwrap().clear();
    for _f in 0 ..= EULER_116_SIZE
        {
            let option_value:Option<u64> =None;
            BAR_CACHE.lock().unwrap().push(option_value);
        }
    new_total += calc_bars2(EULER_116_SIZE,3);
    println!("new total={}",new_total);
    BAR_CACHE.lock().unwrap().clear();
    for _f in 0 ..= EULER_116_SIZE
        {
            let option_value:Option<u64> =None;
            BAR_CACHE.lock().unwrap().push(option_value);
        }
    new_total += calc_bars2(EULER_116_SIZE,4);
    println!("new total={}",new_total);
}




fn calc_bars3(length:usize) -> u64
{
    let mut total =0;
    if length < 2 {return 0;}
//println!("length={}",length);
    if BAR_CACHE.lock().unwrap()[length].is_some()
        {
            BAR_CACHE.lock().unwrap()[length].unwrap()
        }
        else
        {
            for size in (2..=4).rev()
                {
                    if length>=size
                    {
                        for position in 0..=length - size
                            {
                                total += 1;
                                if position + size < length { total += calc_bars3(length - (position + size)); }
                            }
                    }
                }
            println!("Cache miss on {} of {}",length,total);
            BAR_CACHE.lock().unwrap()[length]=Some(total);
            total
        }
}

const EULER_117_SIZE:usize =50;
const EULER_117_BLOCK_SIZE_1:usize =2;
const EULER_117_BLOCK_SIZE_2:usize =3;
const EULER_117_BLOCK_SIZE_3:usize =4;

fn euler_117() //Red, green, and blue tiles
{
    for _f in 0 ..= EULER_117_SIZE
        {
            let option_value:Option<u64> =None;
            BAR_CACHE.lock().unwrap().push(option_value);
        }
    let new_total = calc_bars3(EULER_117_SIZE);
    println!("new total={}",new_total+1);
}


fn generate_pandigitals() -> Vec<u32>
{
    let mut return_vector:Vec<u32>=Vec::new();
    let mut final_vector:Vec<Vec<u32>>=Vec::new();
    let mut starting_vector:Vec<u32>=Vec::new();
    let mut working_vector:Vec<Vec<u32>>=Vec::new();
    working_vector.push(starting_vector);
    for h in 1 ..= 9
        {
            let mut new_working_vector:Vec<Vec<u32>>=Vec::new();
            for g in working_vector.iter()
                {
                    for f in 1..=9
                        {
                            let mut new_vector = g.clone();
                            if !new_vector.contains(&f)
                            {
                                new_vector.push(f);
                                new_working_vector.push(new_vector);
                            }

                        }
                }
            working_vector=new_working_vector;
        }
    final_vector=working_vector;
    for f in final_vector
        {
            let num =f.iter().fold(0, |acc, x| acc*10 + x);
            //println!("{:?}",num);
            return_vector.push(num);

        }
    //println!("pandigital count ={}",return_vector.len());
    return_vector
}

fn ordered_subsets(input_vector:Vec<u32>) -> Vec<Vec<Vec<u32>>>
{
    let mut return_vec:Vec<Vec<Vec<u32>>> = Vec::new();
    let mut final_vector:Vec<Vec<u32>> = Vec::new();

    let mut working_vector:Vec<Vec<u32>> = Vec::new();
    let empty_vector:Vec<u32>=Vec::new();
    working_vector.push(empty_vector);
    let size=input_vector.len();
    while working_vector.len() > 0
        {
            let mut new_working_vector:Vec<Vec<u32>> = Vec::new();
            for f in working_vector.iter()
                {
                    for g in 1..=size
                        {
                            let mut total:u32 =f.iter().sum();
                            total+=g as u32;
                            if total<size as u32
                            {
                                let mut new_vector =f.clone();
                                new_vector.push(g as u32);
                                new_working_vector.push(new_vector);
                            }
                            else if total == size as u32
                            {
                                let mut new_vector =f.clone();
                                new_vector.push(g as u32);
                                final_vector.push(new_vector);
                            }
                        }
                }
            working_vector = new_working_vector;

        }
    for f in final_vector
        {
            let mut new_vec_vec:Vec<Vec<u32>>= Vec::new();
           // println!("f={:?}",f);
            let mut count=0;
            for g in f
                {
                    let mut new_vec:Vec<u32> = Vec::new();
                    for h in 0 .. g
                        {
                            new_vec.push(input_vector[count]);
                            count+=1;
                        }
                    new_vec_vec.push(new_vec);

                }
            //println!("{:?}",new_vec_vec);
            return_vec.push(new_vec_vec);

        }
   // println!("ordered set count ={}",return_vec.len());
    return_vec
}

fn euler_118() //Pandigital prime sets
{
    let mut pandigital_primes:Vec<Vec<u32>> = Vec::new();
    let mut test_vec = vec![1,2,3,4,5,6,7,8,9];
    let pandigitals=generate_pandigitals();
    let mut count=0;
    for h in pandigitals
        {
            let number_vec=number_to_vec(h);
            let result_vec = ordered_subsets(number_vec);
            //println!("{:?}",result_vec);
            let mut usable_vector: Vec<Vec<u32>> = Vec::new();
            for f in result_vec
                {
                    let mut new_vec: Vec<u32> = Vec::new();
                    for g in f
                        {
                            let num = g.iter().fold(0, |acc, x| acc * 10 + x);
                            new_vec.push(num);
                        }
                    new_vec.sort();
                    usable_vector.push(new_vec);
                }
            usable_vector.sort();

            for f in usable_vector
                {
                    if  f.len() > 1
                    {
                        let mut prime = true;
                        for g in f.iter()
                            {
                                if !prime_check_u64(*g as u64)
                                {
                                    prime = false;
                                    break;
                                }
                            }
                        if prime
                        {
                            count += 1;
                            pandigital_primes.push(f.clone());
                            println!("usable {:?} count ={}", f, count);
                        }
                    }
                }
        }
    pandigital_primes.sort();
    pandigital_primes.dedup();
    for f in pandigital_primes.iter()
        {
            println!("{:?}",f);
        }
    println!("total ={}",pandigital_primes.len());
}

const euler_119_limit:u64 = 1_000_000_000_000_000;

#[derive(Copy, Clone)]
#[derive(Eq)]
struct power_struct
{
    pub number: u64,
    pub number_power: u64,
}



impl Ord for power_struct
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.number_power.cmp(&other.number_power)
    }
}

impl PartialOrd for power_struct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number_power.cmp(&other.number_power))
    }
}



impl PartialEq for power_struct {
    fn eq(&self, other: &Self) -> bool {
        (self.number == other.number)  && (self.number_power == other.number_power)
    }
}

fn euler_119() //Digit power sum
{
    //make list
    let mut number_list:Vec<power_struct> = Vec::new();

    for f in 0.. 1_000_000  //numbers
    {
        for g in 2 .. 100  //powers
        {
            let x = num_traits::pow(f, g as usize);
            if x < euler_119_limit
            {
                let  temp:power_struct = power_struct{number:f,number_power:x};
                if (!number_list.contains(&temp)) && (x>9)
                {

                    number_list.push(temp);

                }
            }
            else { break; }

        }
    }
    number_list.sort();
    for f in 0.. number_list.len()
    {
        println!("{} {}",number_list[f].number,number_list[f].number_power);
    }

    let mut count =0;
    for f in 0 ..number_list.len()
        {
            let number_power_temp = number_list[f].number_power;
            let number_sum:u64=number_to_vec_u64(number_power_temp).iter().sum();
            if number_sum ==  number_list[f].number
            {
                count+=1;
                println!("count={} mumber = {}",count,number_list[f].number_power);
            }
        }

}


fn euler_120() //Square remainders
{
    let mut total=0;
    for f in 3 ..=1000
        {
            let a_minus_one: BigUint = f.to_biguint().unwrap() - 1_u32.to_biguint().unwrap();
            let a_plus_one: BigUint = f.to_biguint().unwrap() + 1_u32.to_biguint().unwrap();

            for h in 4 ..= 10
                {
                    let mut test_vector:Vec<u32> = Vec::new();
                    for g in 0..num_traits::pow(4, h)
                        {
                            let a_sum: BigUint = num_traits::pow(a_minus_one.clone(), g) + num_traits::pow(a_plus_one.clone(), g);
                            let remainder = a_sum % (f.to_biguint().unwrap() * f.to_biguint().unwrap());
                            //println!("{}",remainder);
                            test_vector.push(truncate_biguint_to_u32(&remainder));
                        }
                    let x = find_pattern(test_vector);
                    if x.is_some()
                    {
                        //println!("{:?}",x.clone().unwrap());
                        total += *(x.unwrap().iter().max().unwrap());
                        println!("f={} total={}", f, total);
                        break;
                    } else {
                        println!("No Match");
                    }
                }

        }
}


fn euler_121()   //Disc game prize fund
{
    let mut wins=0;
    let total_games =1_000_000_000;
    for f in 1 .. total_games   //games
        {
            //let mut red =1;
           // let mut blue =1;
            let mut good_pick = 0;
            let mut total = 1; //red+blue;
            for g in 1 ..=15
                {

                    let pick = rand::thread_rng().gen_range(0,total+1);

                    total+=1;
                    if pick == 0
                        {
                            good_pick+=1;
                        }
                    if total - good_pick >= 9 {break;}
                }
            if good_pick>=8
                {
                    wins+=1;
                }
        }
    let payout_f = wins as f64 / total_games as f64;
    let payout = (1 as f64 /payout_f) as u32 ;
    println!("Wins={} {} {}",wins, payout_f,payout);
}


fn euler_122_old()  //Efficient exponentiation
{
    let mut test_vector=Vec::new();
    let mut test_member_vector = Vec::new();

    let dummy_vector = Vec::new();
    test_vector.push(0);
    test_member_vector.push(dummy_vector.clone());
    test_vector.push(0);
    test_member_vector.push(dummy_vector.clone());
    test_vector.push(1);
    test_member_vector.push(dummy_vector.clone());
    let mut total =1;
    for f in 3 ..=20
        {
            let mut lowest =f;
            let mut lowest_new_member_vector =Vec::new();
            for g in (f/2 .. f).rev()
                {
                    let mut possible_new_member_vector =Vec::new();
                    //print!("({},{}) ",g ,test_vector[g]);
                    if g != f - g
                    {
                        let mut new_value = test_vector[g]+1;
                        possible_new_member_vector.push(g);
                        possible_new_member_vector.extend(test_member_vector[g].clone());
                        if !test_member_vector[g].contains(&(f-g))
                        {
                            possible_new_member_vector.push(f-g);
                            possible_new_member_vector.extend(test_member_vector[f-g].clone());
                            new_value += test_vector[f - g];
                        }
                        if lowest > new_value
                        {
                            lowest = new_value;
                            lowest_new_member_vector = possible_new_member_vector.clone();
                        }
                        if lowest == new_value
                        {
                            lowest_new_member_vector.extend(possible_new_member_vector.clone())
                        }
                    }
                    else
                    {
                        let new_value = test_vector[g] +1;
                        possible_new_member_vector.push(g);
                        possible_new_member_vector.extend(test_member_vector[g].clone());
                        if lowest > new_value
                        {
                            lowest = new_value;
                            lowest_new_member_vector = possible_new_member_vector.clone();
                        }
                        if lowest == new_value
                        {
                            lowest_new_member_vector.extend(possible_new_member_vector.clone())
                        }
                    }

                }
            println!();
           // println!("f={} multiplication={} {:?}",f, lowest,lowest_new_member_vector);
            total+=lowest;
            println!("f={} multiplication={}  {}",f, lowest,total);
            test_vector.push(lowest);
            test_member_vector.push(lowest_new_member_vector);
        }


}


fn euler_122()  //Efficient exponentiation
{
    let mut test_vector=Vec::new();
    let mut test_member_vector = Vec::new();
    let dummy_vector:Vec<u32> = Vec::new();
    test_vector.push(0);
    test_member_vector.push(dummy_vector.clone());
    test_vector.push(0);
    test_member_vector.push(dummy_vector.clone());
    test_vector.push(1);
    test_member_vector.push(dummy_vector.clone());
    let mut total =1;
    for f in 3 ..=200
        {
//            if f % 2 ==0   //if even
//            {
//                let mut new_value = test_vector[f/2]+1;
//                test_vector.push(new_value);
//            }
            //find highest factor
            let mut highest_factor =1;
            let mut best_new_value =100;
            for g in 1 .. f
                {
                    if f%g == 0
                    {
                        highest_factor = g;
                        let remaining_factor = f-highest_factor;
                        let new_value=test_vector[remaining_factor]+1;
                        if new_value < best_new_value {best_new_value=new_value}
                    }
                }
            //let remaining_factor = f-highest_factor;
           // let final_new_value=test_vector[remaining_factor]+1;
            test_vector.push(best_new_value);
            println!("Added {} {}",f,best_new_value);
            
            

        }
    let total:u32 = test_vector.iter().sum();
    println!("total = {}",total);


}

fn euler_123() //Prime square remainders
{
    let mut prime_vector = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2 .. 1_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    println!("primes gathered");
    for f in 1 .. prime_vector.len()
        {
            let a=prime_vector[f];
            let a_minus_one: BigUint = a.to_biguint().unwrap() - 1_u32.to_biguint().unwrap();
            let a_plus_one: BigUint = a.to_biguint().unwrap() + 1_u32.to_biguint().unwrap();
            let a_sum:BigUint = num_traits::pow(a_minus_one.clone(), f) + num_traits::pow(a_plus_one.clone(), f);
            let remainder = a_sum % (a.to_biguint().unwrap() * a.to_biguint().unwrap());
            println!("n={} {}",f,remainder);
            if remainder > 10_000_000_000_u64.to_biguint().unwrap()
            {
                println!("n={}",f);
                break;
            }


        }
}

#[derive(Copy, Clone)]
#[derive(Eq)]
struct rad_struct
{
    pub number: u32,
    pub rad: u32,
}



impl Ord for rad_struct
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if self.rad != other.rad  { self.rad.cmp(&other.rad)}
        else {self.number.cmp(&other.number) }
    }
}

impl PartialOrd for rad_struct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        if self.rad != other.rad
        {
            Some(self.rad.cmp(&other.rad))
        }
        else {Some(self.number.cmp(&other.number)) }
    }
}



impl PartialEq for rad_struct {
    fn eq(&self, other: &Self) -> bool {
        (self.rad == other.rad)  && (self.number == other.number)
    }
}


fn euler_124() //Ordered radicals
{
    let mut rad_vector:Vec<rad_struct> = Vec::new();
    for f in 1..=100_000
        {
            let mut factors = prime_factors(f);
            factors.sort();
            factors.dedup();
            let this_rad:u32=factors.iter().product();
            println!("n={}, rad={}",f,this_rad);
            let new_rad:rad_struct = rad_struct{number:f as u32,rad:this_rad};
            rad_vector.push(new_rad);
        }
    rad_vector.sort();
    for (index,f) in rad_vector.iter().enumerate()
        {
            if  index < 10_001
            {
                println!("{}   {}   {}", f.number, f.rad, index + 1);
            }
        }

}

const EULER_125_LIMIT:u64 = 100000;
fn euler_125()//Palindromic sums
{
    let mut squares_vector = Vec::new();
    for f in 1 .. EULER_125_LIMIT  //base numbers
        {
            for g in 2 ..= EULER_125_LIMIT-f  // number in sequence
                {
                    let mut running_total:u64 =0;
                    for h in f ..g+f
                        {
                            //print!("{} ",h );
                            running_total+=h*h;
                            if running_total > 100_000_000/*EULER_125_LIMIT * EULER_125_LIMIT*/ {break;}
                        }
                    if running_total > 100_000_000 /*EULER_125_LIMIT * EULER_125_LIMIT*/ {println!(); break;}
                    println!("={}",running_total);
                    squares_vector.push(running_total);
                }
        }
    println!("size={}",squares_vector.len());
    let mut square_vector_vector:Vec<Vec<u64>> = squares_vector.iter().map(|x| number_to_vec_u64(*x as u64)).collect();
    square_vector_vector.retain(|x| {let mut y=x.clone(); y.reverse(); x==&y});
    let mut pal_square_numbers:Vec<u64> = square_vector_vector.iter().map(|x| vec_to_number_u64((*x).clone())).collect();
    pal_square_numbers.sort();
    pal_square_numbers.dedup();
    let total:u64=pal_square_numbers.iter().sum();

    for f in pal_square_numbers
        {
            println!("<{} {}>",f,total);
        }
}


const EULER_126_TEST_SPACE_SIZE:usize =  100 ;
const EULER_126_SHIFT_VALUE:usize =  50 ;

fn euler_126() //Cuboid layers
{
    //let mut target = 1000;
    let mut count = 0;
    let mut now = Instant::now();
    //let mut tally_count:[u32; 100000] =[0; 100000];

    for target in 6 .. 100_000
    {
        count=0;
        for i in 1..target/2
        {
            //count=0;
            for j in 1..=i
            {
                if (i * j * 2) + (2 * i) + (2 * j) > target {  break; }
                for k in 1..=j
                {
                   // print!("{} {} {}: ",i,j,k);
                    let first_row = ((i * j) + (j * k) + (i * k)) * 2;
                    if first_row == target { count += 1; }
                    //if first_row > target { print!("\n");  }
                    //print!("{} ",first_row);
                    let second_row = ((((i + 1) * (j + 1)) + ((j + 1) * (k + 1)) + ((i + 1) * (k + 1))) * 2) - 6;
                    if second_row == target { count += 1; }
                    //if second_row > target { print!("\n"); }
                    //print!("{} ",second_row);
                    let mut dif = second_row - first_row;
                    let mut last_number = second_row;
                    while last_number <= target
                    {
                        dif += 8;
                        last_number += dif;
                        //print!("{} ",last_number);
                        if last_number == target { count += 1; }
                    }
                    //print!("\n");
                }
            }
        }
        print!("\nTarget ={} Count={}\n",target,count);
        if count == 1000 {break;}


    }
    //let  mut test_space = vec![vec![vec![vec![0; target]; target]; target]; target];
    //for index in 6 .. 1000  //(6 ..=target).step_by(2)
    //loop
    //
    // {
    //     count = 0;
    //     // let index =1000;
    //     //target=index;
    //     for i in 1..target / 2
    //     {
    //         //count=0;
    //         for j in 1..=i
    //         {
    //             if (i * j * 2) + (2 * i) + (2 * j) > target { break; }
    //             for k in 1..=j
    //             {
    //                 //print!("Dimentions={} {} {}\n",i,j,k);
    //                 let mut Dimensions = [i, j, k];
    //                 let mut result = layer_cuboid(Dimensions, target);
    //                 let mut dif = result[1] - result[0];
    //                 let mut last_number = result[1];
    //                 let mut target_number = 0;
    //                 for index in 3..100
    //                 {
    //                     dif += 8;
    //                     result.push(last_number + dif);
    //                     last_number += dif;
    //                 }
    //                 //print! {"result={},{},{}  {:?}\n", i, j, k, result};
    //                 for test in result
    //                 {
    //                     if test < 2000
    //                     {
    //                         tally_count[test as usize] += 1;
    //                         print!("value at {} is {}\n", test, tally_count[test as usize]);
    //                     }
    //                 }
    //                 //test_space[i - 1][j - 1][k - 1] = result.clone();
    //                // let temp_count = result.iter().filter(|&n| *n == target as u32).count();
    //
    //             }
    //         }
    //     }
    //}
    // for target2 in 6 .. 1000
    // {
    //     count = 0;
    //     for i in 1..target / 2
    //     {
    //         for j in 1..=i
    //         {
    //             for k in 1..=j
    //             {
    //                 count += test_space[i - 1][j - 1][k - 1].iter().filter(|&n| *n == target2 as u32).count();
    //             }
    //         }
    //         // count += test_space.iter().filter(|&n| *n == target as u32).count();
    //
    //     }
    //     let total_time = now.elapsed().as_secs();
    //     print!("count={} {}             {}\n", target2, count, total_time);
    // }






}
fn layer_cuboid(Dimensions:[usize;3],break_point:usize) -> Vec<u32>
{
    //let Dimensions=[3,2,2];
    let  test_space_size =(Dimensions[0]*2) + 20; //(break_point);
    let test_space_shift=(1+test_space_size-Dimensions[0])/2;

    let  mut test_space = vec![vec![vec![0; test_space_size]; test_space_size]; test_space_size];

    let mut x_dim=test_space_size; //Dimensions[0];
    let mut y_dim=test_space_size; //Dimensions[1];
    let mut z_dim=test_space_size; //Dimensions[2];
    let mut count_vector:Vec<u32> = Vec::new();
    let  mut start_point = 0;// test_space_shift-1 ;
    //print!("test space = {}, shift={}, start point ={}\n",test_space_size,test_space_shift,start_point);
    //print!("Initial Dimensions: {} {} {} ",Dimensions[0],Dimensions[1],Dimensions[2]);
    for i in 0 .. x_dim
    {
        //initial setup
        for j in 0 .. y_dim
            {
                for k in 0 .. z_dim
                    {
                        if i<  Dimensions[0] && j< Dimensions[1] && k < Dimensions[2]
                        {
                            test_space[i+test_space_shift][j+test_space_shift][k+test_space_shift] = 1;
                        }

                    }
            }
    }


    //let layer_count = 4;

    //loop
    for index in 0 ..2
        {
            for i in start_point.. x_dim
                {
                    for j in start_point..y_dim
                        {
                            for k in start_point..z_dim
                                {
                                    if test_space[i][j][k] == 1
                                    {
                                        if test_space[i][j][k + 1] == 0
                                        {
                                            test_space[i][j][k + 1] = 2;
                                        }
                                        if test_space[i][j][k - 1] == 0
                                        {
                                            test_space[i][j][k - 1] = 2;
                                        }
                                        if test_space[i][j + 1][k] == 0
                                        {
                                            test_space[i][j + 1][k] = 2;
                                        }
                                        if test_space[i][j - 1][k] == 0
                                        {
                                            test_space[i][j - 1][k] = 2;
                                        }
                                        if test_space[i + 1][j][k] == 0
                                        {
                                            test_space[i + 1][j][k] = 2;
                                        }
                                        if test_space[i - 1][j][k] == 0
                                        {
                                            test_space[i - 1][j][k] = 2;
                                        }
                                    }
                                }
                        }
                }
            let mut count = 0;
            for k in 0..test_space_size
                {
                    for j in 0..test_space_size
                        {
                            for i in 0..test_space_size
                                {
                                    if test_space[i][j][k] == 2 { count += 1; }
                                    //print!("{} ", test_space[i][j][k])
                                }
                           // print!("\n");
                        }
                   // print!("\n");
                   // print!("\n");
                }
            //println!("cuboid count: {}", count);
           // if count>0
           // {
                count_vector.push(count);
           // }
            for k in 0..test_space_size
                {
                    for j in 0..test_space_size
                        {
                            for i in 0..test_space_size
                                {
                                    if test_space[i][j][k] == 2 { test_space[i][j][k] = 1; }

                                }

                        }

                }
            //if count > break_point as u32 { break;}
            //print!("New Start point = {}",start_point);
            if start_point==1 {print!("BAD {} {} {}*********************************************************\n",x_dim,y_dim,z_dim);}
           // start_point-=1;

            //x_dim+=2;
            //y_dim+=2;
            //z_dim+=2;

        }
    count_vector

}



const EULER_127_LIMIT:u64 =  120_000 ;

fn euler_127() //abc-hits
{
    let numbers:Vec<u64> = (0..EULER_127_LIMIT).collect();
    let mut number_factors:Vec<Vec<u64>> = numbers.iter().map(|x| prime_factors_64(*x)).collect();
    //println!("{:?}",numbers);
    //let dead_numbers =
    for f in number_factors.iter_mut()
    {
        f.sort();
        f.dedup();
    }
    let mut count=0;
    let mut total =0;
    for c in 3 .. EULER_127_LIMIT
        {

            for a in 1 ..= (c/2)
            {

                let b =c -a;
                //println!("{} {} {}",a,b,c);
                if a== b {continue;}
                let mut a_factors=number_factors[a as usize].clone();
                let mut b_factors=number_factors[b as usize].clone();
                let mut c_factors=number_factors[c as usize].clone();
                //println!("<{} {} {} {:?} {:?} {:?}>",a_factors.len(),b_factors.len(),c_factors.len(),a_factors,b_factors,c_factors);
                let total_count=a_factors.len()+b_factors.len()+c_factors.len();
                let mut total_abc=a_factors.clone();
                total_abc.extend(b_factors.clone());
                total_abc.extend(c_factors.clone());
                total_abc.sort();
                total_abc.dedup();
                if total_abc.len() != total_count { continue;}
                let product:u64=total_abc.iter().product();
                if product as u64  >= c {continue;}
                total+=c;
                count+=1;
                println!("ABC hit {} {} {} {} {}",a,b,c,total,count);



            }
        }
}

enum Euler_128_phase_Enum
{
    NewRing,
    Top,
    Left,
    UpperLeft,
    LowerLeft,
    Bottom,
    LowerRight,
    Right,
    UpperRight,
    Ring_done,
}

#[derive(Copy, Clone)]
//#[derive(Eq)]
struct Hex
{
    pub number: u32,
    pub connection: [u32;6],
}

const EULER_128_COUNT:u32 =100;
fn euler_128()
{
    let mut hex_map:Vec<Hex>=Vec::new();
    let mut tile_count:u32 =1;
    let mut ring =1;

    let mut first_tile:Hex =Hex{number:1,connection:[0,0,0,0,0,0]};
    let mut Start_tile:Hex=first_tile.clone();
    tile_count+=1;
    hex_map.push(first_tile);
    let mut base_tile_index=0;
    print!("added tile {} with connections {} {} {} {} {} {}\n",first_tile.number,first_tile.connection[0],first_tile.connection[1],first_tile.connection[2],first_tile.connection[3],first_tile.connection[4],first_tile.connection[5]);
    let base_tile= &hex_map[base_tile_index].clone(); //.get(base_tile_index).unwrap();
    let mut current_phase:Euler_128_phase_Enum =Euler_128_phase_Enum::NewRing;
    for f in 1 ..=20
        {


            match  current_phase
                {
                    Euler_128_phase_Enum::NewRing=>
                        {
                            if ring == 1
                            {
                                base_tile_index=0
                            }
                            else
                            {
                                base_tile_index=(hex_map.get_mut(base_tile_index).unwrap().connection[0]-1) as usize;
                                print!("basetileindex={}\n",base_tile_index)
                            }
                            let base_tile= &hex_map[base_tile_index].clone();
                            let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                            tile_count+=1;
                            {
                                let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                mutable_base_tile.connection[0]=next_tile.number;
                            }
                            next_tile.connection[3]=base_tile.number;
                            current_phase=Euler_128_phase_Enum::UpperLeft;
                            Start_tile=next_tile.clone();
                            print!("added new ring tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                            hex_map.push(next_tile);
                        },
                    Euler_128_phase_Enum::UpperLeft=>
                        {

                            for g in 0 .. ring
                                {
                                    let base_tile= &hex_map[base_tile_index+g].clone();
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let last = hex_map.last().unwrap().clone();
                                    next_tile.connection[5]=last.number; //connection[2];
                                    next_tile.connection[4]=base_tile.number; //connection[1];
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[1] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[2]=next_tile.number;
                                    }
                                    if g !=ring-1
                                    {
                                        next_tile.connection[3]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[0]=next_tile.number;
                                        }
                                        //  base_tile = hex_map.get_mut(base_tile.connection[2] as usize).unwrap();
                                    }
                                    //base_tile = hex_map.get_mut(base_tile.connection[2] as usize).unwrap();
                                   // if
                                    print!("added upper left tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            // let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                            // {
                            //     let last = hex_map.last().unwrap().clone();
                            //     //let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                            //     next_tile.connection[0] = last.number;
                            //     next_tile.connection[5] = base_tile.number;
                            //     print!("added tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                            //     hex_map.push(next_tile);
                            //
                            // }
                            current_phase=Euler_128_phase_Enum::Left;
                        },
                    Euler_128_phase_Enum::Left=>
                        {

                            for g in 0 .. ring
                                {
                                    let base_tile= &hex_map[base_tile_index+g].clone();
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let mut last = hex_map.last_mut().unwrap();
                                    next_tile.connection[0]=last.number; //connection[3];
                                    next_tile.connection[5]=base_tile.number; //connection[2];
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[2] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[3]=next_tile.number;
                                    }
                                    //if base_tile.connection[3] != 0
                                    if g !=ring-1
                                    {
                                        next_tile.connection[4]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[1]=next_tile.number;
                                        }
                                      //  base_tile = hex_map.get_mut(base_tile.connection[2] as usize).unwrap();
                                    }
                                    print!("added left tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            current_phase=Euler_128_phase_Enum::LowerLeft;
                        },
                    Euler_128_phase_Enum::LowerLeft=>
                        {
                            for g in 0 .. ring
                                {
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let mut last = hex_map.last_mut().unwrap();
                                    next_tile.connection[1]=last.number;
                                    next_tile.connection[0]=base_tile.number;
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[3] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[4]=next_tile.number;
                                    }
                                    if g !=ring-1
                                    {
                                        next_tile.connection[5]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[2]=next_tile.number;
                                        }
                                    }
                                    print!("added lower left tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            current_phase=Euler_128_phase_Enum::LowerRight;
                        },
                    Euler_128_phase_Enum::Bottom=>
                        {

                        },
                    Euler_128_phase_Enum::LowerRight=>
                        {
                            for g in 0 .. ring
                                {
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let mut last = hex_map.last_mut().unwrap();
                                    next_tile.connection[2]=last.number;
                                    next_tile.connection[1]=base_tile.number;
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[4] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[5]=next_tile.number;
                                    }
                                    if g !=ring-1
                                    {
                                        next_tile.connection[0]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[3]=next_tile.number;
                                        }
                                    }
                                    print!("added lower right tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            current_phase=Euler_128_phase_Enum::Right;
                        },
                    Euler_128_phase_Enum::Right=>
                        {
                            for g in 0 .. ring
                                {
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let mut last = hex_map.last_mut().unwrap();
                                    next_tile.connection[3]=last.number;
                                    next_tile.connection[2]=base_tile.number;
                                    //next_tile.connection[1]=Start_tile.number;
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[5] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[0]=next_tile.number;
                                    }
                                    // {
                                    //     let mut mutable_start_tile = hex_map.get_mut((Start_tile.number-1) as usize).unwrap();
                                    //     mutable_start_tile.connection[4]=next_tile.number;
                                    // }

                                    if g !=ring-1
                                    {
                                        next_tile.connection[1]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[4]=next_tile.number;
                                        }
                                        //  base_tile = hex_map.get_mut(base_tile.connection[2] as usize).unwrap();
                                    }

                                    print!("added right tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            current_phase=Euler_128_phase_Enum::UpperRight;
                            ring+=1;
                            for h in 1 ..=7
                            {
                                let mut mutable_base_tile = hex_map.get_mut(h-1).unwrap();
                                print!("Reshowing tile {} with connections {} {} {} {} {} {}\n", mutable_base_tile.number, mutable_base_tile.connection[0], mutable_base_tile.connection[1], mutable_base_tile.connection[2], mutable_base_tile.connection[3], mutable_base_tile.connection[4], mutable_base_tile.connection[5]);
                            }
                        },
                    Euler_128_phase_Enum::UpperRight=>
                        {
                            print!("ring={}\n",ring);
                            for g in 0 .. ring-2
                                {
                                    let mut next_tile:Hex = Hex{number:tile_count,connection:[0,0,0,0,0,0]};
                                    tile_count+=1;
                                    let mut last = hex_map.last_mut().unwrap();
                                    next_tile.connection[4]=last.number;
                                    next_tile.connection[3]=base_tile.number;
                                    {
                                        let mut mutable_base_tile = hex_map.get_mut(base_tile_index).unwrap();
                                        mutable_base_tile.connection[0] = next_tile.number;
                                    }
                                    {
                                        let mut mutable_last_tile = hex_map.last_mut().unwrap();
                                        mutable_last_tile.connection[1]=next_tile.number;
                                    }

                                    if g !=ring-1
                                    {
                                        next_tile.connection[1]=base_tile.number+1; //connection[2];
                                        {
                                            let mut mutable_base_tile = hex_map.get_mut(base_tile_index+1).unwrap();
                                            mutable_base_tile.connection[4]=next_tile.number;
                                        }
                                        //  base_tile = hex_map.get_mut(base_tile.connection[2] as usize).unwrap();
                                    }
                                    else
                                    {
                                        next_tile.connection[1]=Start_tile.number;
                                    }
                                    print!("added upper right tile {} with connections {} {} {} {} {} {}\n",next_tile.number,next_tile.connection[0],next_tile.connection[1],next_tile.connection[2],next_tile.connection[3],next_tile.connection[4],next_tile.connection[5]);
                                    hex_map.push(next_tile);
                                }
                            current_phase=Euler_128_phase_Enum::UpperRight;
                            ring+=1;

                            current_phase=Euler_128_phase_Enum::NewRing;
                        },
                    _ => (),
                }

        }

}



fn euler_129() //Repunit divisibility
{
    let mut big_one: BigUint = One::one();
    let big_zero: BigUint = Zero::zero();
    let mut prime_vector = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2 .. 2_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    println!("primes gathered");
    let mut highest_k =1;
    //for n in prime_vector
        for n in 1_000_000 ..2_000_000
        {
            if n< 7  /*1_000_000*/ {continue;}
            if (n%5==0) || (n%2==0) {continue;}
            let mut x = big_one.clone();
            x=x*10_u32.to_biguint().unwrap();
            for k in 1 .. 3_000_000
                {
                    let big_k=k.to_biguint().unwrap();
                    let n_times_9 =n * 9;
                    let big_n=n_times_9.to_biguint().unwrap();
                    if x.modpow(&big_k,&big_n) ==big_one
                    {
                        if k > highest_k {highest_k=k;}
                        println!("n={},k={},highest_k={}",n,k,highest_k);
                        break;
                    }
                }
        }

}


fn euler_130() //Composites with prime repunit property
{
    let mut big_one: BigUint = One::one();
    let big_zero: BigUint = Zero::zero();
    let mut prime_vector = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2..2_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    println!("primes gathered");
    let mut highest_k = 1;
    //for n in prime_vector
    let mut count=0;
    let mut sum =0;
    for n in 1..2_000_000
        {
            if prime_vector.contains(&(n as u64)) {continue;}
            if n < 7  /*1_000_000*/ { continue; }
            if (n % 5 == 0) || (n % 2 == 0) { continue; }
            let mut x = big_one.clone();
            x = x * 10_u32.to_biguint().unwrap();
            for k in 1..3_000_000
                {
                    let big_k = k.to_biguint().unwrap();
                    let n_times_9 = n * 9;
                    let big_n = n_times_9.to_biguint().unwrap();
                    if x.modpow(&big_k, &big_n) == big_one
                    {
                        //println!("n={},k={},",n,k,);
                        if (n-1) % k ==0
                        {
                            count+=1;
                            sum+=n;
                            println!("n={},k={},count={},sum={}",n,k,count,sum);
                        }
                        break;
                    }
                }
        }
}

fn euler_131() //Prime cube partnership
{
    let mut prime_vector = Vec::new();
    let mut cube_vector:Vec<u64> = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2..2_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    for f in 1..1_000 //_000
        {
                cube_vector.push(f*f*f);
                println!("f={}, f cubed={}",f,f*f*f)
        }
    let mut count =0;
    for p in prime_vector
        {
            //let p = prime_vector[f]
            if p > 1_000_000 {break;}
            for n in &cube_vector
                {
                    let big_n =n.to_biguint().unwrap();
                    let big_p = p.to_biguint().unwrap();
                    let big_total = big_n.clone()*big_n.clone()*big_n.clone() + big_p.clone()*big_n.clone()*big_n.clone();
                   // big_total.
                   // let f_total: f64 = total as f64;
                   let f_cbrt_total = big_total.cbrt();
                    //let total_cbrt=f_cbrt_total as u64;
                   //let mut cube=false;
                    if f_cbrt_total.clone()*f_cbrt_total.clone()*f_cbrt_total.clone() == big_total
                    {
                        count+=1;
                        println!("p={} n={} total={}, count={}",p,n,big_total,count);
                        break;
                    }
                }
        }

}

fn euler_132() //Large repunit factors
{
    let mut big_one: BigUint = One::one();
    let big_zero: BigUint = Zero::zero();
    let mut prime_vector = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2 .. 2_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    println!("primes gathered");
    let mut sum=0;
    let mut count=0;
    let k=1_000_000_000;
    let big_k=k.to_biguint().unwrap();
    for n in prime_vector
    //for n in 1_000_000 ..2_000_000
        {
            if n< 7  /*1_000_000*/ {continue;}
            if (n%5==0) || (n%2==0) {continue;}
            let mut x = big_one.clone();
            x=x*10_u32.to_biguint().unwrap();
            let n_times_9 =n * 9;
            let big_n=n_times_9.to_biguint().unwrap();
            if x.modpow(&big_k,&big_n) ==big_one
            {
                sum+=n;
                count+=1;
                println!("n={},count={},sum={}",n,count,sum);
                        //break;
            }

        }

}


fn euler_133() //Repunit nonfactors
{
    let mut big_one: BigUint = One::one();
    let big_zero: BigUint = Zero::zero();
    let mut prime_vector = Vec::new();
    prime_vector.push(1);  //not really a prime
    for f in 2 .. 3_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    println!("primes gathered");
    let mut sum=0;
    let mut count=0;
    //let big_k=k.to_biguint().unwrap();

    for n in prime_vector.clone()
        //for n in 1_000_000 ..2_000_000
        {
            if n < 7  /*1_000_000*/ { continue; }
            if n > 100_000 {break;}
            if (n % 5 == 0) || (n % 2 == 0) { continue; }
            for f in 1 .. 200
                {
                    let big_f = f.to_biguint().unwrap();
                    let f_to_the_n = num_traits::pow(10_u32.to_biguint().unwrap(), f);
                    let mut x = big_one.clone();
                    x = x * 10_u32.to_biguint().unwrap();
                    let n_times_9 = n * 9;
                    let big_n = n_times_9.to_biguint().unwrap();
                    if x.modpow(&f_to_the_n, &big_n) == big_one
                    {
                        sum += n;
                        count += 1;
                        println!("n={},count={},sum={}", n, count, sum);
                        break;
                    }
                }
        }
    let mut sum_2 = 0;
    for prime in prime_vector
        {
            print!("{},",prime);
            if prime > 100_000 {break;}
            sum_2 += prime;
        }
    println!("sum1={}, sum2={}, sum3={} ",sum,sum_2,sum_2-sum)



}

fn euler_134()  //Prime pair connection
{
    let mut prime_vector = Vec::new();
    for f in 2 .. 3_000_000
        {
            if prime_check_u64(f)
            {
                prime_vector.push(f);
            }
        }
    let mut total =0;
    for f in 0 .. prime_vector.len()
        {
            let p1=prime_vector[f];
            if p1 < 5 {continue;}
            if p1 > 1_000_000 {break;}
            let mut digit_count =0;
            let mut temp =p1;
            while temp>0
                {
                    temp/=10;
                    digit_count+=1;
                }

            let p2=prime_vector[f+1];
            let x =p2-p1;
            for g in 1 .. 1_000_000
                {

                    let power=num_traits::pow(10,digit_count);
                    let new_num =g * power;
                    if new_num%p2 == x
                    {
                        total+=new_num+p1;
                        println!("p1={} p2={} number={} total={}",p1,p2,new_num+p1,total);
                        break;
                    }
                }


        }
}

fn euler_135()  //Same differences
{
    let mut count_vector:Vec<u32> =  vec![0; 50_000_000];
    for f in 1 .. 50_000_000
        {
            for gap in 1.. 5000_000_000
            {
                let z:i64=f;
                let y=f+gap;
                let x=y+gap;
                let total =x*x-y*y-z*z;
                if total < 0 {continue;}
                println!("x={} y={} z={} total={}",x,y,z,total);

                if total >= 50_000_000
                {
                    break;
                }
                else
                {
                    count_vector[total as usize]+=1 ;
                }
            }
        }
    //println!("total_array={}",count_array);
    let mut total=0;
    for f in count_vector.iter()
        {
           if *f == 1
           {
               total+=1;
           }
          //  print!("{}, ",f)
        }
    println!();
    println!("total={}",total);

}

fn euler_137()  //Fibonacci golden nuggets
{  //derived fromt the "fibinoti gwenerating sequence"+--
    let mut count = 0;
    //let mut n_plus_1_squared = 0;
    for i in 1_120_100_000_000 .. 10_000_000_000_000
    {
        //sqrt{(n+1)^2 + 4n^2}
        let four_n_squared:u128 = 4 * i * i;
        let n_plus_1_squared = ( i + 1) * (i + 1);
        //i * i * 5 + 2 * i +1
        let final_number = four_n_squared + n_plus_1_squared;
        if can_be_square_u128(final_number)
        {
            let potential_quare_root = final_number.nth_root(2);
            if potential_quare_root * potential_quare_root == final_number
            {
                count += 1;
                print!("Golden Nugget {} {}\n", count, i);
            }
        }

    }
}
fn euler_138()
{
    let limit: u128 = 100000000000;
    let mut l_sum: BigUint = Zero::zero();
    let mut count:u64 = 0;
    //BigUint
    for b in (2..limit).step_by(2)
        {
            let b_over_2:u128   = b / 2;
            let b_squared = b_over_2 * b_over_2;
            let h_minus = b - 1;
            let h_plus = b + 1;
            let L1_u64:u128 = b_squared + (h_minus * h_minus);
            let last_digit=L1_u64%10;
            if can_be_square_u128(L1_u64)
            {
                let L1: BigUint = L1_u64.to_biguint().unwrap();
                let L1_sqrt = L1.sqrt();
                if L1_sqrt.clone() * L1_sqrt.clone() == L1
                //if is_big_square(L1)
                {
                    //let l_float = L1  as f64;
                    //let l_root = l_float.sq
                    l_sum = l_sum + L1_sqrt.clone();
                    count += 1;
                    print!("L={}, h_minus={} b={} {} {}\n", L1_sqrt.clone(), h_minus, b, l_sum, count);
                }
            }
            let L2_u64:u128 = b_squared + (h_plus * h_plus);
            //let last_digit=L2_u64%10;
            if can_be_square_u128(L2_u64)
            {
                let L2: BigUint = L2_u64.to_biguint().unwrap();
                let L2_sqrt = L2.sqrt();
                if L2_sqrt.clone() * L2_sqrt.clone() == L2
                {
                    l_sum = l_sum + L2_sqrt.clone();
                    count += 1;
                    print!("L={}, h_plus={} b={} {} {}\n", L2_sqrt.clone(), h_plus, b, l_sum, count);
                }
                if count >= 12 { break }
            }
        }




//{
//    let Limit:u64 = 100000000000;
//    let mut l_sum =0;
//    let mut count =0;
//    //BigUint
//    for b in (2 .. Limit).step_by(2)
//        {
//            let b_over_2 = b/2;
//            let b_squared =  b_over_2*b_over_2;
//            let h_minus = (b-1);
//            let h_plus = (b+1);
//            let L1 = b_squared + ( (h_minus * h_minus));
//            if is_big_square(L1)
//            {
//                let l_float = L1  as f64;
//                let l_root = l_float.sqrt();
//                if l_root.fract() != 0.0
//                {
//                    print!("ERROR")
//                }
//                l_sum+=l_root as u64;
//                count+=1;
//                print!("L={}, h_minus={} h={} {} {}\n",l_root,h_minus,b,l_sum, count);
//            }
//            let L2 = b_squared + ( h_plus * h_plus);
//            if is_big_square(L2)
//            {
//                let l_float = L2  as f64;
//                let l_root = l_float.sqrt();
//                if l_root.fract() != 0.0
//                {
//                    print!("ERROR")
//                }
//                l_sum+=l_root as u64;
//                count+=1;
//                print!("L={}, h_plus={} b={} {} {}\n",l_root,h_plus,b,l_sum, count);
//            }
//            if count >=12 {break}
//
//        }
//    for L in 1.. Limit
//        {
//            for b_over_2 in 1 .. L
//                {
//                    let hsquared:f64 = (L as f64 * L as f64)-(b_over_2 as f64 * b_over_2 as f64) as f64;
//                    let h=hsquared.sqrt();
//                    if h.fract() != 0.0 {continue}
//                    let h_int:u32 = h as u32;
//                    let dif:i32 = (2 * b_over_2) as i32 - h_int as i32;
//                    if dif.abs() == 1
//                    {
//                        print!("L={}, b={} h={} {}\n",L,2*b_over_2,h_int,dif);
//                    }
//
//                }
//        }
}





fn Pell()
{
    let mut result:u64 = 0;
    let limit:u64 = 100000000;

    let mut x:u64 = 1;
    let mut y:u64 = 1;

    while x+y < limit
        {
            let xnew:u64 = 3 * x + 4 * y;
            let ynew:u64 = 2 * x + 3 * y;

            x = xnew;
            y = ynew;

            result += limit / (x + y);

            print!("x = {}, y = {}", xnew, ynew);
        }
        print!("Number of squares less than 100000000 : {}", result);
}

fn euler_139()  //Pythagorean tiles
{
    let limit:u64 =100_000_000;
    let mut hit_vec:Vec<Vec<u64>>=Vec::new();
    let mut total=0;
    for m in 2 .. limit/2
        {
            let m2=m*m;
            if m2 > limit {break;}
            for n in  1 .. m
                {
                    let n2 =n*n;
                    //let m2=m*m;
                    let  a = m2-n2;
                    let  b = 2*n*m;
                    let  c =m2+n2;
                    if c>= limit {break;}
                    if b>= limit {break;}
                    if (n+m) % 2 == 0 {continue;}
                    if gcd(n,m)!=1 {continue;}
                    let mut diff:u64 =0;
                    if a>b {diff=a-b;}
                    else {diff = b-a;}
                    if c +a+b < limit
                    {
                        if c % (diff) == 0
                        {
                            let mut k =1;
                            total+=limit/(a + b + c);
                            loop
                                {
                                    let a1 = a * k;
                                    let b1 = b * k;
                                    let c1 = c *k;
                                    if a1+b1+c1 >=limit {break}
                                    //if c1> 300 {break}
                                    let mut temp_vec: Vec<u64> = Vec::new();
                                    temp_vec.push(a1);
                                    temp_vec.push(b1);
                                    temp_vec.push(c1);
                                    temp_vec.sort();
                                    hit_vec.push(temp_vec);
                                    println!("m={} a={} b={} c={}", m, a1, b1, c1);
                                    k+=1;
                                    //total +=1;

                                }


                        }
                    }

                }
        }
    //it_vec.sort();
    //hit_vec.dedup();
    println!("{:?}",hit_vec);
    println!("count={} {}",hit_vec.len(),total);
    Pell();
}

fn euler_140() //Modified Fibonacci golden nuggets
{
    let mut fib1:u128=1;
    let mut fib2:u128=4;
    let mut count=1;
    let mut fib_count=0;
    let mut test_number =2;
    let mut total=2;
    for i in 1u128  .. 200
    {
        //sqrt{(n+1)^2 + 4n^2}
        let CurrentFib= fib1 + fib2;
        print!("Fib={}\n",CurrentFib);
        fib1=fib2;
        fib2=CurrentFib;
        if fib_count %4 ==0
        {
            test_number += CurrentFib;
            count += 1;
            print!("Golden Nugget {} {}\n", count, test_number);
        }
        fib_count+=1;
        //     let four_n_squared:u128 = 4 * CurrentFib * CurrentFib;
        // let n_plus_1_squared = ( 1 + CurrentFib) * (1 + CurrentFib);
        // //i * i * 5 + 2 * i +1
        // let final_number = four_n_squared + n_plus_1_squared;
        // if can_be_square_u128(final_number)
        // {
        //     let potential_quare_root = final_number.nth_root(2);
        //     if potential_quare_root * potential_quare_root == final_number
        //     {
        //         count += 1;
        //         print!("Golden Nugget {} {}\n", count, CurrentFib);
        //     }
        // }

    }

}


fn euler_141() //Investigating progressive numbers, n, which are also square
{
    let mut square_vector:Vec<u64> = Vec::new();
    let mut number_vector:Vec<u64> = Vec::new();
    let mut total=0;
    for num in 3 .. 1_000_000  //square root of 100,000
        {
            let square = num * num;
            square_vector.push(square);
            number_vector.push(num);

        }
    for x in 0 .. square_vector.len()
        {
            let num=number_vector[x];
            let square=square_vector[x];
            for q in 1 .. num
                {
                    let d=square/q;
                    let r=square%d;
                    let n=square;
                    //let mut myvec:Vec<u64> = Vec::new();
                    //myvec.push(q);
                    //myvec.push(r);
                    //myvec.push(d);
                    //myvec.sort();
                    //if myvec[0] as f64 / myvec[1] as f64 == myvec[1] as f64 / myvec[2] as f64

                    //if ((d as f32) / (q as f32)) as f32 == ((q as f32) / (r as f32)) as f32
                    if q * q == d * r
                    {
                        total+=n;
                        print!("{} {} {} {} {} {}\n",num,n,d,q,r,total);
                    }



                }
        }
}
fn euler_142() //Perfect Square Collection
{
    let mut square_vector:Vec<u32> = Vec::new();
    for num in 3 .. 1000
        {
            let square = num * num;
            square_vector.push(square);
        }

    for square in square_vector
        {
            print!("Trying {} \n", square);
            for x in (square/2)+1 .. square
                {
                    let y = square -x;
                    let xminus_y = x - y;
                    if ! is_square(xminus_y) {continue;}
                    for z in 1 .. y
                        {
                            let xplus_z = x + z;
                            if ! is_square(xplus_z) {continue;}
                            let xminus_z = x - z;
                            if ! is_square(xminus_z) {continue;}
                            let yplus_z = y + z;
                            if ! is_square(yplus_z) {continue;}
                            let yminus_z = y- z;
                            if ! is_square(yminus_z) {continue;}
                            print!("Perfect square {} {} {}\n",x,y,z)
                        }
                }
        }

//    for x in 1..1000000
//        {
//            if x%10000 ==0
//            {
//                print!("Trying {}\n", x);
//            }
//            for y in x+1..1000000
//                {
//                    let xplus_y = x + y;
//                    if ! is_square(xplus_y) {continue;}
//                    let xminus_y = x - y;
//                    if ! is_square(xminus_y) {continue;}
//                    for z in y+1 .. 1000000
//                        {
//                            //print!("Trying {} {} {}\n" x,y,z);
//                            let xplus_z = x + z;
//                            if ! is_square(xplus_z) {continue;}
//                            let xminus_z = x - z;
//                            if ! is_square(xminus_z) {continue;}
//                            let yplus_z = y + z;
//                            if ! is_square(yplus_z) {continue;}
//                            let yminus_z = y- z;
//                            if ! is_square(yminus_z) {continue;}
//                            print!("Perfect square {} {} {}\n",x,y,z)
//                        }
//                }
//        }
}

fn euler_145()  //How many reversible numbers are there below one-billion
{
    let mut reverse_count = 0;
    for digit_count  in 1 ..=8
        {
            let x1 = u32::pow(10, digit_count);
            let x2= u32::pow(10, digit_count+1);
            print!("x1,x2 {} {}\n",x1,x2);
            for index in x1..(x2/1)
                {
//                    if index % 10000 == 0
//                    {
//                        print!("num1 =={}\n", index);
//                    }
                    let num1 = number_to_vec(index);
                    let mut num2 = num1.clone();
                    num2.reverse();
                    let mut match_count = 0;
                    if num1[0] == 0 || num2[0] == 0 { continue };
                    let num3_int = vec_to_number(num1) + vec_to_number(num2);
                    let num3 = number_to_vec(num3_int);
                    //if index > 100000000
                    //{print!("num3 =={} {:?}\n",num3_int,num3);}
                    for index2 in num3.iter()
                        {
                            if index2 % 2 == 1
                            {
                                match_count += 1;
                            }
                        }
                    if match_count == num3.len()
                    {
                        reverse_count += 1;
                        print!("{},{}\n", index, reverse_count)
                    }
                }
        }

//    for index in 100..1000
//        {
//            let num1 = number_to_vec(index);
//            let mut num2 = num1.clone();
//            num2.reverse();
//            let mut match_count = 0;
//            if num1[0] == 0 || num2[0] == 0{continue};
//            let num3_int=vec_to_number(num1)+vec_to_number(num2);
//            let num3 =number_to_vec(num3_int);
//            //print!("num3 =={} {:?}\n",num3_int,num3);
//            for index2 in num3.iter()
//                {
//                    if index2 %2 == 1
//                    {
//                        match_count += 1;
//                    }
//                }
//            if match_count == num3.len()
//            {
//                reverse_count += 1;
//                print!("{},{}\n",index,reverse_count)
//            }
//        }

}


const EULER_146_SIZE:usize =150_000_000;
fn euler_146()  //Investigating a Prime Pattern
{
    let now = Instant::now();
    //n2+1, n2+3, n2+7, n2+9, n2+13, and n2+27
    let mut total:u64=0;

    let mut primes:Vec<u64> = Vec::new();

    for index in 3 .. 10_000
    {
        if prime_check_u64(index)
        {
            primes.push(index)
        }
    }
    for n in (10 .. EULER_146_SIZE).step_by(10)
        {
            if n % 3 == 0 { continue; }
            //if n % 7 == 0 { continue; }
            //if n % 13 == 0 { continue; }
            //if n % 27 == 0 { continue; }

            if n % 1_000_000 == 0
            {
                let total_time = now.elapsed().as_secs();
                print!("Value={} Duration={}\n",n,total_time);
            }
            let n2:u64 = n as u64* n as u64;
            let mut prime_pattern:Vec<u64> = Vec::new();
            prime_pattern.push(n2+1);
            prime_pattern.push(n2+3);
            prime_pattern.push(n2+7);
            prime_pattern.push(n2+9);
            prime_pattern.push(n2+13);
            prime_pattern.push(n2+27);
            let mut is_prime = true;
            for current_prime in &primes
                {
                    for test_number in &prime_pattern
                        {
                            if test_number % current_prime == 0 && test_number != current_prime
                            {
                                is_prime = false;
                               // print!("failed {} {} {}\n",n,test_number,current_prime);
                                break;
                            }
                        }
                    if is_prime == false {break;}
                }
            if is_prime == true
            {
                for test_number in &prime_pattern
                    {
                        if prime_check_u64(*test_number) == false
                        {
                            is_prime = false;
                            break;
                        }
                    }
            }
            if is_prime == true
            {
                if prime_check_u64(n2+5) {continue}
                if prime_check_u64(n2+11) {continue}
                if prime_check_u64(n2+15) {continue}
                if prime_check_u64(n2+17) {continue}
                if prime_check_u64(n2+19) {continue}
                if prime_check_u64(n2+21) {continue}
                if prime_check_u64(n2+23) {continue}


                total += n as u64;
                print!("Prime Value={} {}\n", n, total);
                let total_time = now.elapsed().as_secs();
                print!("Duration={}\n", total_time);
            }
        }
    let total_time=now.elapsed().as_secs();
    print!("Duration={}",total_time);
}

fn lagged_fibonacci_generator() ->Vec<i64>
{
    //For 1 ≤ k ≤ 55, sk = [100003 − 200003k + 300007k3] (modulo 1000000) − 500000.
    //For 56 ≤ k ≤ 4000000, sk = [sk−24 + sk−55 + 1000000] (modulo 1000000) − 500000.
    let mut return_vector:Vec<i64> = Vec::new();
    for index in 1 ..= 55
        {
            let sk:i64 = ((100003 -200003 * index + 300007*index*index*index) % 1000000) - 500000;
            return_vector.push(sk);
            if index == 10 {print!("s10={}\n",sk)}
        }
    for index in 56 ..= 4000000
        {
            let sk:i64 = ((return_vector[index-25] + return_vector[index-56] + 1000000 as i64) % 1000000 as i64) - 500000 as i64;
            return_vector.push(sk);
            if index == 100 {print!("s100={}\n",sk)}
        }
    return_vector
}

fn euler_148_test()
{
    let mut startpoint=6;
    let mut base_row = 7;
    let mut enteries = 21;
    let mut reduction_factor = 1;
    let mut total_count = 0;
    for row in 7 .. 150
    {

        let new_value = startpoint - (row-base_row) * reduction_factor;
        total_count+=new_value;
        enteries+=row;
        println!(" row {} enteries {} count: {} total count {}",row, enteries, new_value,total_count);
        if new_value == 0
        {
            //print!("reset\n");
            base_row= row+1;
            startpoint+=6;
            reduction_factor+=1;
        }

    }
}
fn euler_148()  //Exploring Pascal's triangle
{
    let mut LastRow:Vec<BigInt>=Vec::new();
    let mut enteries =0;
    let mut new_value =0;
    let mut old_value =0;
    let mut block_count =0;
    let seven_pow_2 = 7_u64.pow(2);
    let seven_pow_3 = 7_u64.pow(3);
    let seven_pow_4 = 7_u64.pow(4);
    let seven_pow_5 = 7_u64.pow(5);
    let seven_pow_6 = 7_u64.pow(6);
    let seven_pow_7 = 7_u64.pow(7);
    let seven_pow_8 = 7_u64.pow(8);
    let seven_pow_9 = 7_u64.pow(9);
    let seven_pow_10 = 7_u64.pow(10);
    let seven_pow_11 = 7_u64.pow(11);
    let seven_pow_12 = 7_u64.pow(12);
    let seven_pow_13 = 7_u64.pow(13);

    let mut test_row_start_value = 0 ;
    //euler_148_test();
     //   return;
    let mut total_count: u64 =0;
    LastRow.push(1.to_bigint().unwrap());
    for row in 1 ..=1_000_000_000
    {
        let mut NewRow:Vec<BigInt>=Vec::new();
        // for index in 0 ..LastRow.len()
        // {
        //
        //     //print!("{} ",LastRow[index]);
        //     if index == 0 //|| index== LastRow.len()-1
        //     {
        //         NewRow.push(1.to_bigint().unwrap())
        //     }
        //     else
        //     {
        //         let newvalue = LastRow[index-1].clone()+LastRow[index].clone();
        //         NewRow.push(newvalue);
        //     }
        //
        // }
        NewRow.push(1.to_bigint().unwrap());
        enteries+=row;
        old_value = new_value;
        new_value = LastRow.iter().filter(|&n| (*n).clone() % 7.to_bigint().unwrap() ==0.to_bigint().unwrap()).count();
        //total_count+=new_value;
        //println!(" row {} enteries {} count: {} total count {}", row,enteries,new_value,total_count);
        //print!("{}", new_value);
        let effective_row = row-1;
        let seven_0 = effective_row %7;
        let seven_1 = (effective_row %(seven_pow_2))/7;
        let seven_2 = (effective_row %(seven_pow_3))/(seven_pow_2);
        let seven_3 =   (effective_row %(seven_pow_4))/(seven_pow_3);
        let seven_4 =   (effective_row %(seven_pow_5))/(seven_pow_4);
        let seven_5 =   (effective_row %(seven_pow_6))/(seven_pow_5);
        let seven_6 =   (effective_row %(seven_pow_7))/(seven_pow_6);
        let seven_7 =   (effective_row %(seven_pow_8))/(seven_pow_7);
        let seven_8 =   (effective_row %(seven_pow_9))/(seven_pow_8);
        let seven_9 =   (effective_row %(seven_pow_10))/(seven_pow_9);
        let seven_10 =   (effective_row %(seven_pow_11))/(seven_pow_10);
        let seven_11 =   (effective_row %(seven_pow_12))/(seven_pow_11);
        let seven_12 =        (effective_row %(seven_pow_13))/(seven_pow_12);

        let current_block = (((effective_row / seven_pow_2))+1) ;
        let effective_block =  current_block -1;
        let mut start_of_block = (48 * effective_block ) + (effective_block/7) * (7-(current_block%7)) + (effective_block/49) * ((7 *(current_block%7))-(current_block%7));
        if current_block % 7 ==0 { start_of_block -= (current_block -7);  }
        let row_in_block = (effective_row / (7 ) % 7);
        let start_of_row = start_of_block+row_in_block * (7-current_block) + (7-1) * row_in_block *(effective_block/7); ////  - row_in_block * (effective_block-1)/7 ;
        let current_value = start_of_row-(row_in_block  * (effective_row % 7)+ (row_in_block * effective_block * (effective_row % 7) ) + ((effective_row % 7)*(current_block -1))) ;
        let starter = (seven_pow_12-1) * seven_12
                        + (seven_pow_11-1) * seven_11
                        + (seven_pow_10-1) * seven_10
                        + (seven_pow_9-1) * seven_9
                        + (seven_pow_8-1) * seven_8
                        + (seven_pow_7-1) * seven_7
                        + (seven_pow_6-1) * seven_6
                         + (seven_pow_5-1) * seven_5
                         + (seven_pow_4-1) * seven_4
                         +  (seven_pow_3-1) * seven_3
                         + (seven_pow_2-1) * seven_2
                         + (seven_1 * (7-1));
        let lev1 = (seven_0 * seven_1);
        let lev2 = (seven_0 * seven_1 *seven_2)+ (seven_2*seven_0) + (seven_2 * seven_1);
        let lev3 = (seven_3 *
                        (seven_2+ seven_1+seven_0 +
                            lev1 +
                            lev2));
        let lev4 = (seven_4 *
                        (seven_3 + seven_2 +seven_1 +seven_0
                            /*(seven_0 * seven_1)*/
                           /* (seven_0 * seven_2)+
                            (seven_1 * seven_2)+
                            (seven_0 * seven_1 * seven_2)*/
                            +lev1
                            +lev2
                            +lev3));
        let lev5 = (seven_5 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4
                +lev1
                +lev2
                +lev3
                +lev4));
        let lev6 = (seven_6 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
            ));
        let lev7 = (seven_7 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
            ));
        let lev8 = (seven_8 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6 +seven_7
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
                +lev7
            ));
        let lev9 = (seven_9 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6 +seven_7 + seven_8
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
                +lev7
                +lev8
            ));
        let lev10 = (seven_10 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6 +seven_7 + seven_8 +seven_9
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
                +lev7
                +lev8
                +lev9
            ));
        let lev11 = (seven_11 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6 +seven_7 + seven_8 +seven_9+seven_10
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
                +lev7
                +lev8
                +lev9
                +lev10
            ));
        let lev12 = (seven_12 *
            (seven_0 + seven_1 +seven_2 +seven_3 +seven_4 +seven_5 +seven_6 +seven_7 + seven_8 +seven_9+seven_10 + seven_11
                +lev1
                +lev2
                +lev3
                +lev4
                +lev5
                +lev6
                +lev7
                +lev8
                +lev9
                +lev10
                +lev11
            ));


        let current_value = starter - lev1  -lev2 - lev3 -lev4 - lev5 -lev6 -lev7 -lev8 -lev9 -lev10 -lev11-lev12;
        total_count+=current_value as u64;
        //print!("[{}]({}) ", current_value,total_count);
        if row % 100000 == 0
        {
            println!("      {}  {}    {} {} {} {} {} {} {} {} {} {} {} {}",row,total_count-(row*(row+1))/2,seven_11,seven_10,seven_9,seven_8,seven_7,seven_6,seven_5,seven_4,seven_3,seven_2,seven_1,seven_0);
        }
        if row % 49 == 0
        {
            block_count+=1;

            //println!("{}",block_count);
            //println!("{}",current_block);
        }
        //print!("\n");
        LastRow=NewRow.clone();


    }
    print!("total={} {}",total_count,(1_000_000_000*(1_000_000_000+1))/2-total_count);
}




const EULER_149_SIZE:usize =2000;
fn euler_149() //Searching for a maximum-sum subsequence
{
    //note.  the rows turned out to be the highest sequence, so I did not need to implement the columns or the diags
    let mut now = Instant::now();
    let mut highest_value:i64=0;
    let  mut matrix_vector /*:Vec<Vec<u32>> */ = vec![[0 ; EULER_149_SIZE] ; EULER_149_SIZE ];
    let base_number_array=lagged_fibonacci_generator();
    let mut index =0;
    for x in 0 .. EULER_149_SIZE
        {
            for y in 0 .. EULER_149_SIZE
                {
                    matrix_vector[x][y]=base_number_array[index];
                    index+=1;
                }
        }
    //row totals
    for column_number in 0 .. EULER_149_SIZE
        {
            //let this_row = matrix_vector[column_number];

            for start_location in 0 .. EULER_149_SIZE
                {
                    let mut total = 0;
                    for length in 0 .. EULER_149_SIZE-start_location
                        {
                            total+=matrix_vector[start_location+length][column_number];
                            if total > highest_value
                            {
                                highest_value= total;
                                print!("column={}, sl={}, length={}, total={}\n", column_number, start_location, length, total)
                            }
                        }
                }
        }
    let total_time=now.elapsed().as_secs();
    print!("Duration={}",total_time);
}

const CAP_SIZE:u32 =60 ; ///18*17*8*5*7*13*11;

fn cap_calculator(current_cap:Fraction, caps_left:usize) -> Vec<Fraction>
{
    let mut new_cap = Fraction::new(0 as u32,1 as u32);
    let one_frac = Fraction::new(1 as u32,1 as u32);
    let cap_size_frac = Fraction::new(CAP_SIZE as u32,1 as u32);
    let mut new_vec:Vec<Fraction> = Vec::new();
    for cap_count in 1 ..= caps_left
    {
        let cap_count_fract = Fraction::new(cap_count as u32,1 as u32);
        if current_cap != Fraction::new(0 as u32,1 as u32)
        {

            new_cap = one_frac / (one_frac / current_cap + one_frac / (cap_count_fract * cap_size_frac));
        }
        else
        {
            new_cap =cap_count_fract * cap_size_frac;
        }
        print!("Loop {}\n",cap_count);


       if cap_count!=caps_left
       {
           print!("Current cap ={} cap count ={} caps left ={}\n",current_cap,cap_count,caps_left);

           let mut temp=cap_calculator( new_cap,caps_left  - cap_count);
           for element in temp
           {
               let new_value = element;  //+ ( cap_count_fract * cap_size_frac);
               if !new_vec.iter().any(|&i| i == new_value)
               {
                   print!("Pushing 1  {} {} {}\n",element,new_value,new_vec.len());
                   new_vec.push(new_value);
               }

           }
        }
        else  //if cap_count==caps_left
        {
            if current_cap != Fraction::new(0 as u32,1 as u32)
            {

                new_cap = one_frac / (one_frac / current_cap + one_frac / (cap_count_fract * cap_size_frac));
            }
            else
            {
                new_cap =cap_count_fract * cap_size_frac;
            }
            print!("Current cap ={} cap count ={}\n",current_cap,cap_count);
            if new_cap!= Fraction::new(0 as u32,1 as u32)
            {
                if !new_vec.iter().any(|&i| i == new_cap)
                {
                    print!("Pushing {}\n",new_cap);
                    new_vec.push(new_cap);
                }
            }
        }

    }
    print!("return vector of size {}\n",new_vec.len());
    new_vec
}

const EULER_150_NUMBER_OF_LINES:usize = 1000;
const EULER_150_PYRAMID: &'static str =
"15
-14 -7
20 -13 -5
-3  8 23 -26
 1 -4 -5 -18 5
-16 31 2 9 28 3";
fn euler_150() //Searching a triangular array for a sub-triangle having minimum-sum
{
    let array_size =1000;
    //let mut test_array:[[i32; EULER_150_NUMBER_OF_LINES]; EULER_150_NUMBER_OF_LINES] = [[0; EULER_150_NUMBER_OF_LINES]; EULER_150_NUMBER_OF_LINES];
    //let mut work_array:[[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut work_vector:Vec<i32> = Vec::new();
    let mut work_array:Vec<Vec<i32>> = Vec::new();
    let mut work_array = vec![vec![0; array_size]; array_size];
    let mut current_position = 0;
    let mut current_line_number = 0;
    let  iter = EULER_150_PYRAMID.split_whitespace();
    let mut lowest =0;
    let mut t = 0;
    for k in 1 ..= 500500
    {
        t = (615949 * t + 797807) % 2_i64.pow(20);
        let sk = t - 2_i64.pow(19);
        work_vector.push(sk as i32 );
    }

    print!("s1={} s2 ={} s3={}\n",work_vector[0],work_vector[1],work_vector[2]);
    // for f in iter
    // {
    //
    //     println!("number = {}", f);
    //     work_array[current_position][current_line_number]=f.parse::<i32>().unwrap();
    //     current_position+=1;
    //     if current_position>current_line_number
    //     {
    //         current_position = 0;
    //         current_line_number +=1;
    //     }
    // }
    let mut count = 0;
    for index1 in 0 .. array_size
    {
        for index2 in 0 .. array_size
        {
            work_array[index2][index1]=work_vector[count];
            count+=1;
            print!("{} ",work_array[index2][index1]);
            if index2 >= index1 {break;}
        }
        print!("\n");
    }
    for start_row in 0 .. array_size
    {
        print!("row {}\n", start_row);
        for end_row in start_row  .. array_size
        {
            for start_column in 0 ..=start_row
            {
                //for end_column in start_column ..=start_row
                let end_column = start_column+ (end_row - start_row);
                {
                    //let mut end_column = start_column + (end_row-);
                    if end_column < array_size
                    {
                        let mut sum = 0;
                        //print!("start row ={} endrow={} start col={} end col ={}\n", start_row, end_row, start_column, end_column);
                        for x in start_row..=end_row
                        {
                            for y in start_column..=start_column + (x-start_row)
                            {
                                //print!("CN={} {} {}\n ", x, y, work_array[y][x]);
                                sum += work_array[y][x];
                            }
                        }
                        if sum < lowest
                        {
                            lowest = sum;
                            print!("start row={} end row={} start collum={} total={} lowest ={}\n", start_row, end_row, start_column, sum, lowest);
                        }
                        //print!("row {} {}\n", start_row, sum);
                    }
                }

            }

        }
    }


}


fn euler_155() //Counting Capacitor Circuits
{
    //let xfraction = CAP_SIZE as Fraction; //Fraction::new(CAP_SIZE,1);
    //let mut list:Vec<Fraction> = Vec::new();
    let end_vect = cap_calculator(Fraction::new(0 as u32,1 as u32),3);
    print!("Final List =");
    for element in end_vect.clone()
    {
        print!("{} ",element.to_string())
    }
    print!("     Size={}\n",end_vect.len());

}


const LIMIT_VALUE:usize =100_000_000_000 ;
fn euler_156() //Counting Digits
{
    let d =9;
    let mut now = Instant::now();
    let mut total =0;
    let mut equal_total =0;
    let mut minus_1 = 0;
    let mut minus_2 =0;
    let mut dif = 00;
    let mut calc_dif =0;
    let mut neg_calc_dif =0;
    let mut index_add =0;
    //  1/10,20/100,300/1000,4000/10000,50000/100000,600_000/1_000_000,7_000_000/10_000_000,80_000_000/100_000_000
    let mut cache:Vec<usize> =  Vec::new();
    for index in 0 .. 1_000
    {
        let x = number_to_vec_u64(index);
        let new_value = x.iter().filter(|&n| *n==d).count();
        cache.push(new_value);
    }
    let mut index =0;
    while index <  LIMIT_VALUE //00_000
    {
        //print!("index ={} {}\n",index,total);
        let low = index % 1000;
        let mid = (index/1000) % 1000;
        let top = (index/1_000_000) % 1000;
        let very_top = (index/1_000_000_000) % 1000;
        //print!("top={} mid ={} low ={}\n",top,mid,low);
        if total  > index
        {
            calc_dif =0;
            neg_calc_dif = total - index;
        }
        else //index > total
        {
            neg_calc_dif =0;
            calc_dif = index - total ;
        }
        //print!("count={} index={} diff={}\n",total,index,calc_dif);
        if(calc_dif > cache[very_top] * 1000 + cache[top] * 1000 +  cache[mid] * 1000 + 300) && (LIMIT_VALUE - index > 2000)  && index % 1000 == 0
        {
            //print!("count={} index={} diff={}\n",total,index,calc_dif);
            //print!("top={} mid ={} low ={}\n",top,mid,low);
            //print!("dif change = {}\n",cache[top] * 1000 +  cache[mid] * 1000 + 300);
            total += cache[very_top] * 1000 + cache[top] * 1000 +  cache[mid] * 1000 + 300;
            index+=1000;
        }
        else if(neg_calc_dif >  cache[very_top] * 1000 + cache[top] * 1000 +  cache[mid] * 1000 + 700) && (LIMIT_VALUE - index > 2000)  && index % 1000 == 0
        {
            total += cache[very_top] * 1000 + cache[top] * 1000 +  cache[mid] * 1000 + 300;
            index+=1000;
        }
        else
        {

            let x = number_to_vec_u64(index as u64);
            let new_value = x.iter().filter(|&n| *n == d).count();
            let minus_2 = minus_1;
            let minus_1 = dif;
            total += new_value;
            //index+=1;

            //print!("{}\n",total as f64 / index  as f64);
            if total == index as usize
            {
                equal_total += total;
                print!("EQUAL {} {} {} {}\n", index, new_value, total, equal_total);
            }
            // else if minus_2/10000 < minus_1/10000 && dif/10000 < minus_1/10000
            // {
            //     print!("CAP {} {} {}\n", index,dif, total);
            // }
            // else if minus_2/10000 > minus_1/10000 && dif/10000 > minus_1/10000
            // {
            //     print!("BOTTOM {} {} {}\n", index, dif, total);
            // }
            else if minus_1 > 0 && dif < 0
            {
                print!("Crossover {} {} {}\n", index, dif, total);
            } else if minus_1 < 0 && dif > 0
            {
                print!("Crossover {} {} {}\n", index, dif, total);
            } else if dif != minus_1
            {
                // print!("Diff {}\n", dif);
            }
            index+=1;
        }
    }
    print!("Total Digits={} {} in {}",equal_total,total,now.elapsed().as_secs());


}
const EULER_158_MAX_CHARS:usize = 3;
fn calc_less_than(calc_value:u32,excluded_value:u32,location:u32) -> u32
{
    let mut possibilities = 0;
    if location+1 == EULER_158_MAX_CHARS as u32
    {
        if excluded_value < calc_value
        {
            possibilities =   calc_value -2;
        }
        else
        {
        possibilities = calc_value - 1;
        }
    }
    else
    {
        for new_calc_value in 1 .. calc_value
        {
            if new_calc_value != excluded_value
            {
                possibilities += calc_less_than(new_calc_value, excluded_value, location + 1);
            }
        }
    }
    return possibilities;
}


fn euler_158()//Exploring strings for which only one character comes lexicographically after its neighbour to the left
{
    let mut less_than_array:[u32;26] = [0;26];
    let mut total:u32 = 0;
    for index in 0 .. 26
    {
            total +=index +1;
            less_than_array[index as usize] = total;
             print!("total={}\n",total);
    }
    let mut possabilities = 0;
    for cusp_number in 1 ..=EULER_158_MAX_CHARS
    {
        for cusp_value in 1 ..=25
        {
            if cusp_number==EULER_158_MAX_CHARS
            {
                possabilities+=26-cusp_value;

            }
            else
            {
                for cusp_follower in cusp_value + 1..26
                {
                    possabilities += calc_less_than(cusp_follower, cusp_value, cusp_number as u32);
                }
            }
        }
    }

}