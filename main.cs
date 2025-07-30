[ApiController]
[Route("[controller]")]
public class UsersController : ControllerBase {
    static List<User> users = new List<User> {
        new User{ Id=1, Name="Waleed", Email="waleed@example.com"},
        new User{ Id=2, Name="Yasmin", Email="yasmin@example.com"}
    };

    [HttpGet]
    public IEnumerable<User> Get() => users;

    [HttpPost("add-user")]
    public User Post([FromBody] User user) {
        user.Id = users.Count + 1;
        users.Add(user);
        return user;
    }
}
