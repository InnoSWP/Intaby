import enum
from typing import List, NewType

# Types declaration

# For user
NickName = NewType('NickName', str)
Email = NewType('Email', str)
Password = NewType('Password', str)

# For quiz
QuizId = NewType('QuizId', int)
QuizName = NewType('QuizName', str)

# For question
QuestionAnswer = NewType('QuestionAnswers', str)
QuestionDescription = NewType('QuestionDescription', str)


class Answer:
    def __init__(self, answer_id: int, text: QuestionAnswer, is_correct: bool):
        self.answer_id = answer_id
        self.text = text
        self.is_correct = is_correct


# Possible types of question in a quiz
class QuestionTypes(enum.Enum):
    poll = "polls"
    quiz = "quiz"


# Question class, to store information about question:
#   question type, description, possible answers, and correct answers
class Question:
    def __init__(self, question_id: int, question_type: QuestionTypes, text: QuestionDescription, quiz_id: int):
        self.question_id = question_id
        self.question_type = question_type
        self.text = text
        self.quiz_id = quiz_id

    # [maybe deleted]
    # return next question
    def next(self):
        pass


# Quiz class,
class Quiz:
    def __init__(self, quiz_id: QuizId, quiz_name: QuizName, user_id: int = 0):
        self.quiz_id = quiz_id
        self.quiz_name = quiz_name
        self.user_id = user_id


# User class, to manage stored in db information through nickname/email
class User:
    def __init__(self, user_id: int, nickname: NickName, email: Email, password: Password):
        self.nickname = nickname
        self.email = email
        self.password = password
        self.user_id = user_id
