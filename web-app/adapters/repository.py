# from abc import ABC, abstractmethod


import model

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
class SqlAlchemyRepository:
    def __init__(self, session):
        self.session = session

    def add_user(self, user: model.Quiz):
        self.session.add(user)

    def add_quiz(self, quiz: model.Quiz):
        self.session.add(quiz)

    def get_quiz(self, quiz_id):
        return self.session.query(model.Quiz).filter_by(id=quiz_id).one()

    def list_quizzes(self, user_id):
        return self.session.query(model.User).filter_by(user_id=user_id).all()

    def get_question(self, question_id):
        return self.session.query(model.Question).filter_by(id=question_id).one()

    def list_questions(self, quiz_id):
        return self.session.query(model.Question).filter_by(quiz_id=quiz_id).all()
