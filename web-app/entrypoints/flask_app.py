import json

from flask import Flask, request, render_template
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

import config
from domain import model
from adapters import orm, repository
from service_layer import services
from entrypoints.api import api_initialization
from tests import test_orm

engine = create_engine(config.get_postgres_uri())
get_session = sessionmaker(bind=engine)
repository.initialize_repo(get_session())
orm.start_mappers()
orm.create_all(engine)
app = Flask(
    __name__,
    template_folder='./../static/templates',
    static_folder='./../static'
)
api = api_initialization.api_initialization(app)


@app.route('/')
def main_page():
    return render_template("main_page_bootstrap_free.html")


@app.route('/login')
def login_page():
    return render_template('authorization_page.html')


@app.route('/register')
def registration_page():
    return render_template("registration_page.html")


@app.route('/user')
def user_page():
    return render_template("page_of_user.html")


# [will be deleted]
@app.route('/lobby')
def lobby_page():
    return render_template("waiting_hall.html")


# For tests through docker
@app.route('/tests')
def test():
    repo = repository.get_repo()

    new_items = list()
    for item in repo.get_users():
        new_items.append(item.to_dict())

    return json.dumps({"items": new_items})


@app.route('/tests/user_creation')
def test_user_creation():
    repo = repository.get_repo()

    test_orm.test_add_user(repo)

    return "<p>OK</p>"
