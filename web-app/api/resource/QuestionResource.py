from flask import jsonify
from flask_restful import reqparse, abort, Resource
from adapters.repository import SqlAlchemyRepository, get_repo
from domain.model import Question, QuestionTypes

parser = reqparse.RequestParser()
parser.add_argument('id', required=True, type=int)
parser.add_argument('text', required=True)
parser.add_argument('question_type', required=True, type=QuestionTypes)


class QuestionResource(Resource):
    def get(self, question_id):
        repo = get_repo()
        question = repo.get_question(question_id)
        return question
