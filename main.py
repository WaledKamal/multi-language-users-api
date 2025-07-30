from flask import Flask, jsonify, request

app = Flask(__name__)

users = [
    {"id": 1, "name": "Waleed", "email": "waleed@example.com"},
    {"id": 2, "name": "Yasmin", "email": "yasmin@example.com"}
]

# GET /users
@app.route("/users", methods=["GET"])
def get_users():
    return jsonify(users)

# POST /add-user
@app.route("/add-user", methods=["POST"])
def add_user():
    data = request.get_json()

    if not data or "name" not in data or "email" not in data:
        return jsonify({"error": "name and email required"}), 400

    new_user = {
        "id": len(users) + 1,
        "name": data["name"],
        "email": data["email"]
    }
    users.append(new_user)
    return jsonify(new_user)

# Root
@app.route("/", methods=["GET"])
def home():
    return jsonify({"message": "API is working 🚀"})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080, debug=True)
