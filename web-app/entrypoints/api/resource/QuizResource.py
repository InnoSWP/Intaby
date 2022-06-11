from flask import jsonify
from flask_restful import reqparse, abort, Resource
from adapters.repository import SqlAlchemyRepository, get_repo
from domain.model import Quiz

parser = reqparse.RequestParser()
parser.add_argument('id', required=True, type=int)
parser.add_argument('name', required=True)
parser.add_argument('questions', required=True, type=dict)


class QuizResource(Resource):
    def get(self, quiz_id):
        repo = get_repo()
        quiz = repo.get_quiz(quiz_id)
        return quiz


class QuizListResource(Resource):
    def get(self, user_id):
        repo = get_repo()
        quizzes = repo.list_quizzes(user_id)
        return quizzes

    def post(self, user_id):
        args = parser.parse_args()
        repo = get_repo()

        repo.add_quiz(Quiz(quiz_id=args['id'], quiz_name=args['name'], user_id=user_id))

        #       TODO: parse questions and answers to add them to SQL
        #

        return jsonify({"success": "OK"})

    def put(self):
        pass
