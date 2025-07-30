require 'sinatra'
require 'json'

users = [
  {id:1, name:"Waleed", email:"waleed@example.com"},
  {id:2, name:"Yasmin", email:"yasmin@example.com"}
]

get '/users' do
  content_type :json
  users.to_json
end

post '/add-user' do
  content_type :json
  data = JSON.parse(request.body.read)
  new_user = {id: users.length+1, name: data["name"], email: data["email"]}
  users << new_user
  new_user.to_json
end
