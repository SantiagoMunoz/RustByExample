//Definition of a struct
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    //Creation of an instance 
    let mut user_1 = User{
        email : String::from("someone@localhost"),
        username: String::from("sum1"),
        active : true,
        sign_in_count: 0,
    };
    //Changing a value of a struct
    user_1.email = String::from("legitmail@legithost.com");
    
    let mut user_2 = build_user(String::from("local@localhost"), String::from("bot"));

    let mut user_3 = User{
        email : String::from("localmail@provider.com"),
        username : String::from("r"),
        ..user_1
    };
    
    let m_color : Color(0, 0, 0);
    let m_point: Point(5, 5, 5);


}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count : 0,
    }
}
