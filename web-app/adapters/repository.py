# from abc import ABC, abstractmethod

from domain import model

__repo = None


# drafts
# class AbstractRepository(ABC):
#     @abstractmethod
#     def add(self, thing):
#         raise NotImplementedError
#
#     @abstractmethod
#     def get(self, ref):
#         raise NotImplementedError


# repository pattern to structure code
# SqlAlchemy repository to access db
# TODO: comments
# TODO: command for put request
class SqlAlchemyRepository:
    def __init__(self, session):
        self.session = session

    def add_user(self, user: model.Quiz):
        self.session.add(user)

    def get_user(self, user_id):
        return self.session.query(model.User).filter_by(id=user_id).one()

    # Working with quizzes
    def add_quiz(self, quiz: model.Quiz):
        self.session.add(quiz)

    def get_quiz(self, quiz_id):
        return self.session.query(model.Quiz).filter_by(id=quiz_id).one()

    def list_quizzes(self, user_id):
        return self.session.query(model.User).filter_by(user_id=user_id).all()

    # Working with questions
    def add_question(self, question: model.Question):
        self.session.add(question)

    def get_question(self, question_id):
        return self.session.query(model.Question).filter_by(id=question_id).one()

    def list_questions(self, quiz_id):
        return self.session.query(model.Question).filter_by(quiz_id=quiz_id).all()

    # Working with answers
    def add_answer(self, answer: model.Answer):
        self.session.add(answer)

    def get_answer(self, answer_id):
        return self.session.query(model.Answer).filter_by(id=answer_id)

    def list_answers(self, question_id):
        return self.session.query(model.Answer).filter_by(question_id=question_id)


# Debug
class FakeRepository:
    def add_user(self, user: model.Quiz):
        print(f"Adding user {user}")

    def get_user(self, user_id):
        print(f"Getting user {user_id}")

    # Working with quizzes
    def add_quiz(self, quiz: model.Quiz):
        print(f"Adding quiz {quiz}")

    def get_quiz(self, quiz_id):
        print(f"Getting quiz {quiz_id}")

    def list_quizzes(self, user_id):
        print(f"Getting list of quizzes of user {user_id}")

    # Working with questions
    def add_question(self, question: model.Question):
        print(f"Adding question {question}")

    def get_question(self, question_id):
        print(f"Getting question {question_id}")

    def list_questions(self, quiz_id):
        print(f"Getting list of question of quiz {quiz_id}")

    # Working with answers
    def add_answer(self, answer: model.Answer):
        print(f"Adding answer {answer}")

    def get_answer(self, answer_id):
        print(f"Getting answer {answer_id}")

    def list_answers(self, question_id):
        print(f"Getting list of answers of question {question_id}")


# Debug
__repo = FakeRepository()


# To access repo from different modules
def initialize_repo(session):
    global __repo

    if not __repo or __repo.__class__ == FakeRepository:
        __repo = SqlAlchemyRepository(session)


def get_repo():
    global __repo
    return __repo
