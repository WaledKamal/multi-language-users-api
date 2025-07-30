@RestController
public class UserController {
    List<User> users = new ArrayList<>(List.of(
        new User(1,"Waleed","waleed@example.com"),
        new User(2,"Yasmin","yasmin@example.com")
    ));

    @GetMapping("/users")
    public List<User> getUsers() {
        return users;
    }

    @PostMapping("/add-user")
    public User addUser(@RequestBody User user) {
        user.setId(users.size()+1);
        users.add(user);
        return user;
    }
}
