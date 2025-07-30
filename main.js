const express = require("express");
const app = express();
const PORT = 8080;

app.use(express.json()); // Middleware عشان نقرأ الـ JSON

let users = [
  { id: 1, name: "Waleed", email: "waleed@example.com" },
  { id: 2, name: "Yasmin", email: "yasmin@example.com" },
];

// GET /users
app.get("/users", (req, res) => {
  res.json(users);
});

// POST /add-user
app.post("/add-user", (req, res) => {
  const { name, email } = req.body;
  if (!name || !email) {
    return res.status(400).json({ error: "name and email required" });
  }

  const newUser = {
    id: users.length + 1,
    name,
    email,
  };

  users.push(newUser);
  res.json(newUser);
});

// Root
app.get("/", (req, res) => {
  res.json({ message: "API is working 🚀" });
});

// Start server
app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
