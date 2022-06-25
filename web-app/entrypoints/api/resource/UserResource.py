import flask
from flask import jsonify, make_response
from flask_restful import reqparse, abort, Resource
from adapters.repository import SqlAlchemyRepository, get_repo
from domain.model import User
from service_layer import services

registration_parser = reqparse.RequestParser()
registration_parser.add_argument('name', required=True)
registration_parser.add_argument('surname', required=True)
registration_parser.add_argument('email', required=True)
registration_parser.add_argument('password', required=True)

login_parser = reqparse.RequestParser()
login_parser.add_argument('email', required=True)
login_parser.add_argument('password', required=True)


def check_for_credentials(args: dict):
    email, password = services.email_and_pass_from(args)

    repo = get_repo()
    user = repo.get_user_by_email(email)

    if not user:
        abort(404, message="Unauthorized")

    if not user.password == password:
        abort(404, message="Unauthorized")


def abort_if_user_not_found(user_id: int, args: dict):
    email, password = services.email_and_pass_from(args)

    repo = get_repo()
    user = repo.get_user_by_id(user_id)
    if not user or user.email != email or user.password != password:
        abort(404, message="Unauthorized")


class UserResource(Resource):
    def get(self, user_id):
        args = login_parser.parse_args()
        abort_if_user_not_found(user_id, args)
        user = get_repo().get_user_by_id(user_id)
        return make_response("User exists", 200)


class UserRegistrationResource(Resource):
    def get(self):
        args = login_parser.parse_args()

        check_for_credentials(args)

        return make_response("User exists", 200)

    def post(self):
        args = registration_parser.parse_args()
        repo = get_repo()

        user = services.user_from(args)

        repo.add_user(user)
        return make_response("User created", 201)
