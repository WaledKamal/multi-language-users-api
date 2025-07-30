<?php
header("Content-Type: application/json");
header("Access-Control-Allow-Origin: *");
header("Access-Control-Allow-Methods: GET, POST");
header("Access-Control-Allow-Headers: Content-Type");

 $users = [
    ["id" => 1, "name" => "Waleed", "email" => "waleed@example.com"],
    ["id" => 2, "name" => "Yasmin", "email" => "yasmin@example.com"],
];

$uri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$method = $_SERVER['REQUEST_METHOD'];

// GET /users
if ($uri === "/users" && $method === "GET") {
    echo json_encode($users);
    exit;
}

// POST /add-user
if ($uri === "/add-user" && $method === "POST") {
    $data = json_decode(file_get_contents("php://input"), true);

    if (!isset($data["name"]) || !isset($data["email"])) {
        http_response_code(400);
        echo json_encode(["error" => "name and email required"]);
        exit;
    }

    $newUser = [
        "id" => count($users) + 1,
        "name" => $data["name"],
        "email" => $data["email"]
    ];

    $users[] = $newUser;

    echo json_encode($newUser);
    exit;
}

http_response_code(404);
echo json_encode(["error" => "Not Found"]);
