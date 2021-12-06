use num_bigint::BigUint;
use num_bigint::{ToBigUint};
use num_traits::{Zero, One};
use num_traits::ToPrimitive;
use Point;
pub fn truncate_biguint_to_u32(a: &BigUint) -> u32
{
    use std::u32;
    let mask = BigUint::from(u32::MAX);
    (a & mask).to_u32().unwrap()
}


//pub fn factors(numbber:u32) -> Vec<u32>
//{
//
//}

pub fn factor_multiply(number_1:&mut Vec<(usize,usize)>,number_2:&Vec<(usize,usize)>) //-> Vec<(usize,usize)>
{
    //let mut ResultVector:Vec<(usize,usize)> = number_1.clone(); //Vec::with_capacity(20);

    for f in number_2.iter()
        {
            let (factor_2,exponent_2)=f;
            match number_1.binary_search_by_key(factor_2, |(a,_b)| *a)
                {
                    Ok(factor_number) =>
                        {
                            let ( factor_1 , mut exponent_1)=number_1[factor_number];
                            //factor_1+=factor_2;
                            exponent_1+=exponent_2;
                            number_1[factor_number]=(factor_1,exponent_1);
                        }
                    Err(_e) =>
                        {
                            number_1.push((*factor_2,*exponent_2));
                        }
                }
        }
    //number_1
}


//this assumes number_1 is sorted
pub fn factor_divide(number_1:&Vec<(usize,usize)>,number_2:&Vec<(usize,usize)>) -> Option<Vec<(usize,usize)>>
{
    for f in number_2.iter().rev()
        {
            let (factor_2,_exponent_2)=f;
            if !number_1.binary_search_by_key(factor_2, |(a,_b)| *a).is_ok()
            {
                return None;
            }
        }
    let mut result_vector:Vec<(usize,usize)> = number_1.clone(); //Vec::with_capacity(20);
    for f in number_2.iter().rev()
        {
            let (factor_2,exponent_2)=f;
            match result_vector.binary_search_by_key(factor_2, |(a,_b)| *a)
                {
                    Ok(number) =>
                        {
                            let (mut factor_1 , mut exponent_1)=result_vector[number];
                            if exponent_1<*exponent_2 {return None}
                            else {exponent_1-=exponent_2;}
                            result_vector[number]=(factor_1,exponent_1);
                        },
                    Err(_e)   => return None
                }
        }

    Some(result_vector)
}

pub fn prime_factors_64(number:u64) -> Vec<u64>
{
    let mut index:u64 =2;
    let mut working_number=number;
    let mut prime_vector:Vec<u64> = Vec::new();
    if prime_check_u64(working_number)
    {
        prime_vector.push(working_number as u64);
    }
    else
    {
        //let float_working_number = working_number as f64;
        //let square_root_working_number = float_working_number.sqrt();
        while working_number > 1
            {
                if prime_check(index as i32)
                {
                    while working_number % index as u64 == 0
                        {
                            prime_vector.push(index);
                            working_number /= index as u64;
                        }
                }
                index += 1;
            }
    }
    prime_vector
}



pub fn prime_factors(number:u64) -> Vec<u32>
{
    let mut index:u32 =2;
    let mut working_number=number;
    let mut prime_vector:Vec<u32> = Vec::new();
    if prime_check_u64(working_number)
    {
        prime_vector.push(working_number as u32);
    }
    else
    {
        //let float_working_number = working_number as f64;
        //let square_root_working_number = float_working_number.sqrt();
        while working_number > 1
            {
                if prime_check(index as i32)
                {
                    while working_number % index as u64 == 0
                        {
                            prime_vector.push(index);
                            working_number /= index as u64;
                        }
                }
                index += 1;
            }
    }
    prime_vector
}



pub fn prime_check(number:i32) -> bool
{
    let mut  result:bool=true;
    let f_number : f64 = number as f64;
    let target = f_number.sqrt();
    let int_target = target as i32  +1;
    if number< 30
    {
        result = PRELIM_PRIME_FILTER[number as usize];
    }
    else
    {
        if !PRIME_FILTER[(number % 30) as usize]
        {
            result = false;
        } else {
            for f in 2..int_target
                {
                    if number % f == 0
                    {
                        result = false;
                        break;
                    }
                }
        }
    }
    result
}


const PRELIM_PRIME_FILTER:[bool;30]=[false,false,true,true,false,true,false,true,false,false,false,true,false,true,false,false,false,true,false,true,false,false,false,true,false,false,false,false,false,true];
const PRIME_FILTER:[bool;30]=[false,true,false,false,false,false,false,true,false,false,false,true,false,true,false,false,false,true,false,true,false,false,false,true,false,false,false,false,false,true];

fn prime_factors_2(number:u64) -> Vec<u32>
{
    let mut prime_vector:Vec<u32> = Vec::with_capacity(20);
    let mut working_number=number;
    let mut index:u64 =2;
    if number <= 1 {return prime_vector}
    while working_number >= index * index
        {
            if working_number % index ==0
            {
                prime_vector.push(index as u32);
                working_number/=index;
            }
            else
            {
                index+=1;
            }
        }
    prime_vector.push(working_number as u32);
    prime_vector.sort();
    prime_vector
}




impl Digits for BigUint
{
    fn digits(&self) -> usize
    {
        let digit_count:usize;

        let working_number:BigUint=self.clone();
        digit_count=working_number.to_string().len();
//        while working_number >0
//            {
//                working_number/=10;
//                digit_count+=1;
//            }
        digit_count
    }
}

impl Digits for u32
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

pub fn reverse_digits(num:&BigUint) -> BigUint
{
    let mut return_num:BigUint = 0.to_biguint().unwrap();
    let mut working_number:BigUint = num.clone();
    while working_number > 0.to_biguint().unwrap()
        {
            return_num *= 10.to_biguint().unwrap();
            return_num+=working_number.clone() % 10.to_biguint().unwrap();
            working_number/=10.to_biguint().unwrap();
        }
    return_num
}

pub fn find_permutations( a:&mut Vec<u32>, n:u32, return_vec:&mut Vec<u32>)
{
    if n== 1
    {
        let mut number =0;
        //print_vec(&a);
        for f in 0 .. a.len()
            {
                number = number *10;
                number = number + a[f];
                //println!("temp={}",number);
            }
        //if !return_vec.contains(&number)
        //{
        return_vec.push(number);
        //}
        //println!("number = {}",number);
    }
    else
    {
        for i in 0 .. n
            {
                a.swap(i as usize,(n-1) as usize); // (remove the ith element)
                find_permutations(a, n-1,return_vec);
                a.swap(i as usize, (n-1) as usize);  // (restore for the next round)
            }
    }
}



pub fn find_permutations_u64( a:&mut Vec<u64>, n:u64, return_vec:&mut Vec<u64>)
{
    if n== 1
    {
        let mut number =0;
        //print_vec(&a);
        for f in 0 .. a.len()
            {
                number = number *10;
                number = number + a[f];
                //println!("temp={}",number);
            }
        //if !return_vec.contains(&number)
        //{
        return_vec.push(number);
        //}
        //println!("number = {}",number);
    }
    else
    {
        for i in 0 .. n
            {
                a.swap(i as usize,(n-1) as usize); // (remove the ith element)
                find_permutations_u64(a, n-1,return_vec);
                a.swap(i as usize, (n-1) as usize);  // (restore for the next round)
            }
    }
}


pub fn big_uint_to_vec(num:BigUint)->Vec<u32>
{
    let mut vec:Vec<u32> = Vec::new();
    let mut working_number = num.to_biguint().unwrap();
    while working_number !=0.to_biguint().unwrap()
        {
            let digit_as_bigint:BigUint = working_number.clone() % 10.to_biguint().unwrap();
            let x=truncate_biguint_to_u32(&digit_as_bigint);
            vec.push(x);
            working_number/=10.to_biguint().unwrap();
        }
    vec
}

pub fn number_to_vec_u64(num:u64)->Vec<u64>
{
    let mut vec:Vec<u64> = Vec::new();
    let mut working_number = num;
    while working_number !=0
        {
            vec.push(working_number % 10);
            working_number/=10;
        }
    vec
}


pub fn number_to_vec_u128(num:u128)->Vec<u128>
{
    let mut vec:Vec<u128> = Vec::new();
    let mut working_number = num;
    while working_number !=0
    {
        vec.push(working_number % 10);
        working_number/=10;
    }
    vec
}



pub fn number_to_vec(num:u32)->Vec<u32>
{
    let mut vec:Vec<u32> = Vec::new();
    let mut working_number = num;
    while working_number !=0
        {
            vec.push(working_number % 10);
            working_number/=10;
        }
    vec
}


pub fn vec_to_number(this_vec:Vec<u32>) -> u32
{
    let mut return_val =0;
    for f in this_vec
        {
            return_val *=10;
            return_val+=f;
        }
    return_val
}

pub fn vec_to_number_u64(this_vec:Vec<u64>) -> u64
{
    let mut return_val =0;
    for f in this_vec
        {
            return_val *=10;
            return_val+=f;
        }
    return_val
}

pub fn print_vec_u64(vector:&Vec<u64>)
{
    print!("Vector=");
    for f in 0 .. vector.len()
        {
            print!("{} ",vector[f]);
        }
    println!();

}

pub fn print_vec(vector:&Vec<u32>)
{
    print!("Vector=");
    for f in 0 .. vector.len()
        {
            print!("{} ",vector[f]);
        }
    println!();

}

pub fn is_permutation(number_1:u64,number_2:u64) -> bool
{
    let mut digit_count_vec:Vec<i8> = [0;20].to_vec();
    let mut working_number_1=number_1;
    let mut working_number_2=number_2;
    let mut no_match = false;
    if working_number_1.digits() != working_number_2.digits() { false }
    else
    {
        while working_number_1>0
            {
                let digit:usize = (working_number_1 %10) as usize;
                //print!("{} ",digit);
                digit_count_vec[digit] +=1;
                working_number_1/=10;
            }
        while working_number_2>0
            {
                let digit:usize = (working_number_2 %10) as usize;
                //print!("{} ",digit);
                digit_count_vec[digit] -=1;
                working_number_2/=10;
            }
        //println!();
        //print!("Digit Count=");
        for f in 0 .. 20
            {
                // print!("{} ",digit_count_vec[f]);
                if digit_count_vec[f]!=0
                {
                    no_match = true;
                    break;
                }
            }
        //println!();
        //println!("perm = {} {} {}",number_1,number_2,!no_match);
        if no_match {false}
        else {true}
    }


}


pub fn prime_factor(number:u32) -> Vec<u32>
{
    let mut vec:Vec<u32> = Vec::new();
    let target = number /2;
    //print!("prime factor for {} =",number);
    let mut working_number =number;
    for g in 2 .. target+1
        {

            if prime_check (g as i32)
            {
                //          println!("g={}",g);
                while working_number % g == 0
                    {
                        // print!("{},",g);
                        working_number/=g;
                        vec.push(g);
                    }
            }
            if working_number == 1
            {
                break;
            }
        }
    // println!("");
    vec
}


pub fn is_triangle(num:u32) -> bool
{
    let mut return_val=false;
    let mut adder=2;
    let mut total =1;
    while total<=num
        {
            total+=adder;
            if total == num
            {
                return_val =true;
            }
            adder+=1;


        }
    return_val

}
pub fn is_square(num:u32)  -> bool
{
    let f_square = num as f64;
    let target = f_square.sqrt();
    if target.fract() == 0.0
    {
        true
    }
    else
    {
        false
    }
}

pub fn is_square_64(num:u64)  -> bool
{
    let f_square = num as f64;
    let target = f_square.sqrt();
    if target.fract() == 0.0
    {
        if target * target == f_square
        {
            true
        }
        else
        {
            false
        }
    }
    else
    {
        false
    }
}

pub fn is_square_128(num:u64)  -> bool
{
    let f_square = num as f64;
    let target = f_square.sqrt();
    if target.fract() == 0.0
    {
        if target * target == f_square
        {
            true
        }
        else
        {
            false
        }
    }
    else
    {
        false
    }
}



pub fn get_digital_root(num:u64) -> u64
{
    let mut sum=num;
    loop
        {
            let num_vec = number_to_vec_u64(sum);
            sum = num_vec.iter().sum();
            if sum < 10 {break}
        }
    sum
}

pub fn get_digital_root_u128(num:u128) -> u128
{
    let mut sum=num;
    loop
    {
        let num_vec = number_to_vec_u128(sum);
        sum = num_vec.iter().sum();
        if sum < 10 {break}
    }
    sum
}


pub fn can_be_square(num:u64) -> bool
{
    let last_digit = num%10;
    let second_last_digit=(num/10)%10;
    if last_digit ==2 || last_digit ==3 || last_digit == 7 && last_digit ==8 {return false}
    if last_digit  == 0 && second_last_digit !=0 {return false}
    if last_digit == 6 && second_last_digit % 2 ==0 {return false}
    if last_digit !=6 && second_last_digit %2 ==1 {return false}
    if last_digit == 5 && second_last_digit !=2 {return false}
    let digital_root=get_digital_root(num);
    //0, 1, 4 or 7
    if digital_root ==2 || digital_root == 3 || digital_root == 5 || digital_root == 6 || digital_root == 8 || digital_root == 9 {return false}
    return true
}




pub fn can_be_square_u128(num:u128) -> bool
{
    let last_digit = num%10;
    let second_last_digit=(num/10)%10;
    if last_digit ==2 || last_digit ==3 || last_digit == 7 && last_digit ==8 {return false}
    if last_digit  == 0 && second_last_digit !=0 {return false}
    if last_digit == 6 && second_last_digit % 2 ==0 {return false}
    if last_digit !=6 && second_last_digit %2 ==1 {return false}
    if last_digit == 5 && second_last_digit !=2 {return false}
    let digital_root=get_digital_root_u128(num);
    //0, 1, 4 or 7
    if digital_root ==2 || digital_root == 3 || digital_root == 5 || digital_root == 6 || digital_root == 8 || digital_root == 9 {return false}
    return true
}



pub fn is_big_square(num:u64)  -> bool
{
    let f_square = num as f64;
    let target = f_square.sqrt();
    if target as u128 * target as u128 == num as u128
    {
        true
    }
    else
    {
        false
    }
}


pub fn is_pentagon(num:u32) -> bool
{
    let mut return_val=false;
    let mut adder=4;
    let mut total =1;
    while total<=num
        {
            total+=adder;
            if total == num
            {
                return_val =true;
            }
            adder+=3;


        }
    return_val
}

pub fn is_hexagon(num:u32) -> bool
{
    let mut return_val=false;
    let mut adder=5;
    let mut total =1;
    while total<=num
        {
            total+=adder;
            if total == num
            {
                return_val =true;
            }
            adder+=4;


        }
    return_val

}

pub fn is_heptagon(num:u32) -> bool
{
    let mut return_val=false;
    let mut adder=7;
    let mut total =1;
    while total<=num
        {
            total+=adder;
            if total == num
            {
                return_val =true;
            }
            adder+=6;


        }
    return_val

}

pub fn is_octagon(num:u32) -> bool
{
    let mut return_val=false;
    let mut adder=6;
    let mut total =1;
    while total<=num
        {
            total+=adder;
            if total == num
            {
                return_val =true;
            }
            adder+=5;


        }
    return_val

}



pub fn char_value(my_char:char) -> u8
{
    print!("{},",my_char as u8 - 64 );
    my_char as u8 - 64
}

pub fn triangle_number(count:u32) -> Vec<u32>
{
    let mut vec:Vec<u32> =Vec::new();
    for f in 1 .. count
        {
            let number = (f * (f + 1)) / 2;
            //println!("{}",number);
            vec.push(number);
        }
    vec
}


pub fn digits_in_number(num:u32) -> u32
{
    let mut digits=0;
    let mut working_number=num;
    while working_number >0
        {
            digits+=1;
            working_number/=10;
        }
    digits
}


pub fn is_pandigital_zero(num:u64) -> bool
{
    let mut return_val = true;
    let mut used:[bool;10]=[false;10];
    //used[0]= true;
    let mut working_number = num;
    while working_number >0
        {
            let digit = working_number %10;
            if used[digit as usize]
            {
                return_val = false;
            }
            else
            {
                used[digit as usize]=true;
            }
            working_number/=10;
        }
    return_val

}



pub fn is_pandigital(num:u32) -> bool
{
    let mut return_val = true;
    let mut used:[bool;10]=[false;10];
    used[0]= true;
    let mut working_number = num;
    while working_number >0
        {
            let digit = working_number %10;
            if used[digit as usize]
            {
                return_val = false;
            }
            else
            {
                used[digit as usize]=true;
            }
            working_number/=10;
        }
    return_val

}


pub fn digit_count(num:i32) ->usize
{
    let  total ;
    if num >=1000000000 { total = 10;}
    else if num >=100000000 { total = 9;}
    else if num >=10000000 { total = 8;}
    else if num >=1000000 { total = 7;}
    else if num >=100000 { total = 6;}
    else if num >=10000 { total = 5;}
    else if num >=1000 { total = 4;}
    else if num >=100 { total = 3;}
    else if num >=10 { total = 2;}
    else { total = 1}
    return total
}


pub fn prime_check_u64(number:u64) -> bool
{
    let mut  result:bool=true;
    let f_number : f64 = number as f64;
    let target = f_number.sqrt();
    let int_target = target as u64  +1;
    if number< 30
    {
        result = PRELIM_PRIME_FILTER[number as usize];
    }
    else
    {
        if !PRIME_FILTER[(number % 30) as usize]
        {
            result = false;
        } else {
            for f in 2..int_target
                {
                    if number % f == 0
                    {
                        result = false;
                        break;
                    }
                }
        }
    }
    result
}

pub fn gcd(a:u64, b:u64) -> u64
{
    let mut t:u64;
    let mut a2:u64 =a;
    let mut b2:u64 =b;
    while b2 != 0
        {
            t = a2;
            a2 = b2;
            b2 = t%b2;
        }
    a2
}
//generate all subsets of a given vector
pub fn subsets(set:Vec<u32>) -> Vec<Vec<u32>>
{
    let mut return_vec:Vec<Vec<u32>> = Vec::new();
    let element_count = set.len();
    let power_base:u32 = 2;
    let test_number = power_base.pow(element_count as u32);
    //println!("test number = {}",test_number);
    for f in 0 .. test_number
        {
            let mut scratch_number=f;
            let mut subset:Vec<u32> = Vec::new();
            for g in 0 .. element_count
                {
                    if scratch_number % 2 == 1
                    {
                        subset.push(set[g]);
                    }
                    scratch_number = scratch_number >> 1;
                }
            return_vec.push(subset);
        }
    return_vec
}


pub fn subsets_64(set:Vec<u64>) -> Vec<Vec<u64>>
{
    let mut return_vec:Vec<Vec<u64>> = Vec::new();
    let element_count = set.len();
    let power_base:u64 = 2;
    let test_number = power_base.pow(element_count as u32);
    //println!("test number = {}",test_number);
    for f in 0 .. test_number
        {
            let mut scratch_number=f;
            let mut subset:Vec<u64> = Vec::new();
            for g in 0 .. element_count
                {
                    if scratch_number % 2 == 1
                        {
                            subset.push(set[g]);
                        }
                    scratch_number = scratch_number >> 1;
                }
            return_vec.push(subset);
        }
    return_vec
}

pub fn find_pattern(field:Vec<u32>) -> Option<Vec<u32>>
{
    for f in 1 .. field.len()/2
        {
            let pattern = field[0..f].to_vec();
            //println!("pattern={:?}",pattern);
            let mut new_field  =  field.clone();
            while new_field.len() > pattern.len()
                {
                    let matching_field = new_field[0..pattern.len()].to_vec();
                   // println!("matching_field {:?} pattern {:?}",matching_field,pattern);
                    if matching_field == pattern
                    {

                        new_field= new_field[pattern.len()..new_field.len()].to_vec();

                    }
                    else { break;}
                }
            if new_field.len() < pattern.len() { return Some(pattern.to_vec())}
        }
    None
}


pub fn cross_product(X1:Point<i64>,X2:Point<i64>) -> i64
{
    return X1.x * X2.y - X1.y *X2.x;
}