from flask import Flask, request
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

import config
from domain import model
from adapters import orm, repository
from service_layer import services

orm.start_mappers()
# get_session = sessionmaker(bind=create_engine(config.get_postgres_uri()))
app = Flask(__name__)

# @app.route("/api/quiz", methods=["POST", "GET", "PUT"])
# def quiz_management():
#     session = get_session()
#     repo = repository.SqlAlchemyRepository(session)
#
#     if request.method == "GET":
#         return repo.list_quizzes()
#
#     if request.method == "POST":
#         repo.add_quiz
