fn main() {

//VARIABLES


    /*
    let x=5;
    println!("Number is {x}");
    // works prints 5
    
    x=6;
    // cannot assign to immutable error cuz didnt specify at make
    
    let x=6;
    println!("Number is {x}");
    // works prints both num is 5 and num is 6
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
	//works prints both
    
    const 3hrINsec:u32=60*60*3;
    // y easier to calcualte than store??
    
    //shadowing
    let x=5;
    let x=x+1;
    {
    	let x=x*2;
    	println!("{x}");		//12
    }
    println!("{x}");		//6
    
    let spaces="    ";			//str type
    let spaces=spaces.len();		//num type
    //counts number of spaces
    
    let mut spaces="    ";			
    spaces=spaces.len();
    //logically same but due to type differnec mutability not possible
    
 
 
//DATA TYPES
	
	//let guess: u32 = "42".parse().expect("Not a number!");
	//let guess= "42".parse().expect("Not a number!");-ERROR type annotation needed
	
	// scalar type
	//integer(signed/unsigned) 8(i8/u8) 16 32 64 128 arch(isize/usize)   
	//storage = -2^(n-1) to 2^(n-1)-1 where n is bits
	// i8 -2^7 to 2^7-1 = -128 to 127
	//unsigned 0 to 2n-1 u8 0 to 255
	//arch is dependent on computer architecture -> 32bit/64bit 
	//integer literals - Decimal 98_222 / Hex 0xff / Octal 0o77 / Binary 0b1111_0000 / Byte(u8 only) b'A'
	//deafault i32
	//integer overflow saving 256 on u8 can give 2 behaviours  (1) code panic at runtime - exits with error (2) compile w --release mode compile does not check for overflow, when happen performs two's complemnt wrapping 256=0 257=1...
	/* PREVENTION
    Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    Return the None value if there is overflow with the checked_* methods.
    Return the value and a boolean indicating whether there was overflow with the 		 
    overflowing_* methods.
    Saturate at the value’s minimum or maximum values with the saturating_* methods.
	*/
	//floating point f32(single precision) f64(double precision) default64 all signed
	/*	IEEE-754 standard
	let x=2.0; 		//f64
	let y:f32=3.0		//f32
	
	//numeric operation
	/*
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; 		// Results in -1
    let remainder = 43 % 5;
    //boolean
    //let t=true; or let f:bool=false;
    //character SINGLE QUOTES	4 bytes unicode scalar so accepts more than normal
    /*
    let c ="z";
    let z:char='Z';
    let heart_eyed_cat = '😻';
    */
	//compound types
	//tuple fixed length comma sep() can be hetero
	// let tup:(i32,f64,u8)=(500,6.4,1);
	//binds w full tuple so to get ele unpack -> let (x,y,z)=tup;
	//tuple without any value has name -> unit
	/*
	let x: (i32,f64,u8)=(500,6.4,1);
	let fivehun=x.0;
	let spf=x.1;........
	*/
	//ARRAY
	//must have same type
	//fixed length
	//let a=[1,2,3,4,5];
	//vector similiar allowed to grow/shrink 
	//mostly use vetcir than array
	//array fro when u know not gonnna change
	//array type ; size => let a:[i32;5]=[1,2,3,4,5];
	//let a: [3;5]; => [3,3,3,3,3];
	//first = a[0] , second = a[1],......
	//code panicks if try to access outside size => runtime array
	
	
//FUNCTIONS

	
	
	
}
