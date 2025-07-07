struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: i64,
}

fn main() {
    let user1 = User {
        active: true,
        user_name: String::from("Someusername"),
        email: String::from("someuseremail@.com"),
        sign_in_count: 1,
    };

    print_user_data(&user1);

    let mut user2 = build_new_user(String::from("Donald"), String::from("donald_email@.com"));
    print_user_data(&user2);
    user2.email = String::from("erik_email@.com");
    user2.user_name = String::from("Erik");
    print_user_data(&user2);

    //create a copy of User with some different fields
    let user3 = User {
        user_name: String::from("Thomas"),
        email: String::from("thomas@d.com"),
        ..user2
    };
    print_user_data(&user3);

    let user4 = User {
        ..user1 // we can not use user1 because all String fields were moved to user4
    };
    print_user_data(&user4);

    // print_user_data(&user1); this will generate an error
}

fn build_new_user(user_name: String, email: String) -> User {
    User {
        active: true,
        user_name,
        email,
        sign_in_count: 1,
    }
}

fn print_user_data(user: &User) {
    println!(
        "User active: {}\nUser name: {}\nemail: {}\nsign in count: {}",
        user.active, user.user_name, user.email, user.sign_in_count
    );
}
