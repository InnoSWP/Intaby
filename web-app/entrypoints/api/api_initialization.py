from entrypoints.api.resource import AnswerResource, QuizResource, QuestionResource, UserResource
from flask_restful import Api


def api_initialization(app) -> Api:
    api = Api(app)
    api.add_resource(UserResource.UserResource, '/api/user/<int:user_id>')
    api.add_resource(QuizResource.QuizResource, '/api/user/<int:user_id>/quiz/<int:quiz_id>')
    api.add_resource(QuizResource.QuizListResource, '/api/user/<int:user_id>/quiz')

    return api
