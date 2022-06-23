from flask import jsonify
from flask_restful import reqparse, abort, Resource
from adapters.repository import SqlAlchemyRepository, get_repo
from domain.model import Quiz

quiz_creation_parser = reqparse.RequestParser()
quiz_creation_parser.add_argument('name', required=True)
quiz_creation_parser.add_argument('questions', required=True)
quiz_creation_parser.add_argument('email', required=True)
quiz_creation_parser.add_argument('password', required=True)

quiz_get_parser = reqparse.RequestParser()
quiz_get_parser.add_argument('email', required=True)
quiz_get_parser.add_argument('password', required=True)


# TODO: checking credentials


class QuizResource(Resource):
    def get(self, quiz_id):
        args = quiz_get_parser.parse_args()
        repo = get_repo()
        quiz = repo.get_quiz(quiz_id)
        return quiz

    def put(self, quiz_id):
        args = quiz_creation_parser.parse_args()
        repo = get_repo()
        repo.put_quiz(Quiz(quiz_id=quiz_id, name=args['name'], user_id=args['user_id']))

        # TODO: Putting questions + answers

        return jsonify({"success": "OK"})

    def delete(self, quiz_id):
        args = quiz_get_parser.parse_args()
        repo = get_repo()
        quiz = repo.delete_quiz(quiz_id)

        # TODO: Deleting questions + answers

        return jsonify({"success": "OK"})


class QuizListResource(Resource):
    def get(self, user_id):
        args = quiz_get_parser.parse_args()
        repo = get_repo()
        quizzes = repo.list_quizzes(user_id)
        return quizzes

    def post(self, user_id):
        args = quiz_creation_parser.parse_args()
        repo = get_repo()

        repo.add_quiz(Quiz(quiz_name=args['name'], user_id=user_id))

        #       TODO: parse questions and answers to add them to SQL
        #

        return jsonify({"success": "OK"})
