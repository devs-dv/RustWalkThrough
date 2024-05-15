use std::fmt;
use std::fmt::{Display,Formatter,Error};
enum VertDir{
    Up,
    Down

}
enum HorizDir{
    Left,
    Right
}
struct Ball{
    x : i32,
    y : i32,
    vert_dir: VertDir,
    hotriz_dir: HorizDir
}
struct Frame{
    width : i32,
    height : i32,

}

struct Game{
    frame : Frame,
    ball : Ball
}
impl Game{
    fn new() -> Game{
        Game{
            frame : Frame{
                width : 63,
                height : 31
            },
            ball : Ball{
                x : 44,
                y : 21,
                vert_dit: VertDir::Down,
                horiz_dir : HorizDir::Right
            }
        }
    }

    fn step(&nut self){
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball{
    fn bounce(&nut self, frame:&Frame){
        if self.x ,= 0{
            self.horiz_dir = HorizDir::Right;
        }
        else if frame.width <= self.x{
            self.horiz_dir = HorizDir::Left;  
        }
        else if self.y <=0{
            self.vert_dir = VertDir::Down;
        }
        else if frame.height <= self.y{
            self.vert_dir = VertDir::Up;
        }
        else {

        }
    }
     fn mv(&mut self){
        matchs self.horiz_dir{
            HorizDir::Left => self.x -= 1,
            HorizDir:: Right => self.x += 1
        }
        match self.vert_dir{
            VertDir:: Up => self.y -= 1,
            VertDir::Down => self.y += 1
        }
     }
}
impl Display for Game{
    fn fmt(&self, fmt: @mut Formatter)-> Result<(), Error>{
        write!(fmt , "x");
        for _in 0..64{write!(fmt, "-"); }
        for y:i32 in 0..32{
            for x:i32 in 0..64{
                if self.ball.x == x as i32 && self.ball.y == as i32{
                    write!(fmt,"0");
                }
                if x == 0 {write!(fmt ,"|");}
                else if x !=0 && y != 31{write!(fmt, " ");}
                else { write!(fmt, "-"); }
            }
            write(fmt,"\n");

        }
        write!(fmt ,"\n")
    }
}


fn main() {
    let mut new_g: Game = Game::new();
    let sleep_time: Duration = std::time::Duration::from_millis(millis:33);
    loop{
        println!("{}", new_g);
        new_g.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", new_g.ball.x , new_g.ball.y);
    }
}

// fn main(){
//     // println!("Hello, world!");
//     let x: i32 = 16;
//     // to print the output out , we need to write a macro => printLn! macro 
//     println!("{}", x);

//     let x: String = String::("Hello!, Soroban!"); // mutable string 
//     let y: &str = " Hello,Stellar"; // immutable string 

//     println("{y}");
//     println!("{z}");

//     // functions 
//     // pub fn event (name: String) {
//     //     let name: String = String::from("Divyanshu!");
//     //     println!("{}",name);
//     // }
//     let e = EventForMe{
//         name: String::from("Devs-DV"),
//         data: String::from("15.05.2024"),
//         number_of_participants:1000,
//         place: String::from("New ,Delhi")
//     };


// }fn main

// // structs - > compling many itesm in one class
// struct EventForMe{
//     name: String,
//     data: String,
//     number_of_participants: u32,
//     place: String

// }

// // enums - complie errors in one class
// enums ErrosFroEvents{
//     NoEvent,
//     CancelledEvent,
//     EventType

// }