from flask import Flask

from flask_restx import Api, Resource

app = Flask(__name__)

api = Api(app, version='1.0', title='Simple API', description='sample swagger docs', doc='/api-docs')

default_api = api.namespace('default', description='just information')
users_api = api.namespace('users', description='handle user data')
addition_api = api.namespace('addition', description='just addition')
multliplication_api = api.namespace('multiplication', description='just multiplication')
pythagorean_api = api.namespace('pythagorean', description='Pythagorean theorem')

@default_api.route('/')
class Home(Resource):
    def get(self):
        return 'default requests!'

@users_api.route('/<user_name>/<int:user_id>')
class Home(Resource):
    def get(self, user_name, user_id):
        data = {}
        data['message'] = f'Hello, {user_name}({user_id})!'
        return data
    
    def delete(self, user_name, user_id):
        data = {}
        data['message'] = f'{user_name}({user_id}) deleted successfully.'

@addition_api.route('/<int:a>/<int:b>')
class Addition(Resource):
    def get(self, a, b):
        data = {}
        data['input1'] = a
        data['input2'] = b
        data['result'] = a + b
        return data

@multliplication_api.route('/<int:a>/<int:b>')
class Multiplication(Resource):
    def get(self, a, b):
        data = {}
        data['input1'] = a
        data['input2'] = b
        data['result'] = a * b
        return data

@pythagorean_api.route('/<int:a>/<int:b>')
class Pythagorean(Resource):
    def get(self, a, b):
        data = {}
        data['input1'] = a
        data['input2'] = b
        data['result'] = (a**2 + b**2)**0.5
        return data

if __name__ == '__main__':
    app.run(debug=True)