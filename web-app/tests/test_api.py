import json

from requests import get, post


def test_quiz_post_request():
    quiz_post_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }

    url = "http://127.0.0.1:8888/api/user/1/quiz"
    return post(url, data=json.dumps(quiz_post_request_body), headers={"Content-Type": "application/json"}).text


def test_quiz_get_request():
    url = "http://127.0.0.1:8080/api/user/1/quiz"
    return get(url).text


def test_user_get_request():
    url = "http://127.0.0.1:8080/api/user/1"
    return get(url).text


def test_user_post_request():
    url = "http://127.0.0.1:8888/api/user"

    user_post_request_body = {
        'nickname': 'Il',
        'email': 'il@k.r',
        'password': 'strong_pass'
    }
    return post(url, data=json.dumps(user_post_request_body), headers={"Content-Type": "application/json"}).text


def test_question_get_request():
    url = "http://127.0.0.1:8888/api/quiz/1/question/2"
    return get(url).text


# print(test_quiz_post_request())
# print(test_quiz_get_request())
print(test_user_post_request())
# print(test_question_get_request())
