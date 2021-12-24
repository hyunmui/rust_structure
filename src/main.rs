fn main() {
    // println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someonename123"),
        active: true,
        sign_in_count: 1,
    };

    // let email = user1.email;

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("email2@example.com"), String::from("hyunmui"));

    let user3 = User {
        email: String::from("email2@example.com"), 
        username: String::from("hyunmui"),
        ..user1     // 다른 동일객체의 필드에서 값 가져오기
    };

    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{} / {}", user3.username, user3.active);


    // Tuple
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // 변수명 + 필드명 축약형
        username,   // 변수명 + 필드명 축약형
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);