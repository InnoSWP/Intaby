from flask import jsonify
from flask_restful import reqparse, abort, Resource
from adapters.repository import SqlAlchemyRepository, get_repo

parser = reqparse.RequestParser()
parser.add_argument()


def abort_if_user_not_found(user_id):
    repo = get_repo()
    user = repo.get_user(user_id)
    if not user:
        abort(404, message=f"User {user_id} not found")


class UserResource(Resource):
    def get(self, user_id):
        abort_if_user_not_found(user_id)
        user = get_repo().get_user(user_id)
        return user
