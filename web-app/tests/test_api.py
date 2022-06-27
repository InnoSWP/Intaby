import json

from requests import get, post, delete


def test_quizzes_post_request():
    answers1 = [
        {"text": "Hello", "correct_answer": True},
        {"text": "World", "correct_answer": False}
    ]
    question1 = {"question_type": "poll",
                 "text": "questions1",
                 "time": 60,
                 "answers": answers1}

    answers2 = [
        {"text": "Hello", "correct_answer": True},
        {"text": "World", "correct_answer": False}
    ]

    question2 = {
        "question_type": "quiz",
        "text": "questions2",
        "time": 15,
        "answers": answers2
    }

    quiz_post_request_body = {
        "questions": [question1, question2],
        "name": "Quiz name"
    }

    # print(type(quiz_post_request_body["questions"]))

    url = "http://127.0.0.1:8888/api/user/1/quiz"
    return post(url, json=quiz_post_request_body,
                headers={"Content-Type": "application/json"}).text


def test_quiz_get_request():
    url = "http://127.0.0.1:8888/api/user/1/quiz/1"
    assert get(url) == 200


def test_quiz_delete_request():
    quiz_post_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    post(url, data=json.dumps(quiz_post_request_body), headers={"Content-Type": "application/json"})

    assert delete(url + "/1").status_code == 200


def test_quizzes_get_request():
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    return get(url).text


def test_quiz_put_request():
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    quiz_put_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }

    return post(url, json=json.dumps(quiz_put_request_body), headers={"Content-Type": "application/json"}).text


def test_user_login_request():
    url = "http://127.0.0.1:8888/api/user/login"

    body = {"email": "vikochka_kruk@mail.ru",
            "password": "123"}

    return post(url, json=body).text


def test_user_post_request():
    url = "http://127.0.0.1:8888/api/user"

    user_post_request_body = {
        'nickname': 'Il',
        'email': 'il@k.r',
        'password': 'strong_pass'
    }
    return post(url, data=user_post_request_body, headers={"Content-Type": "application/json"}).text


def test_question_get_request():
    url = "http://127.0.0.1:8888/api/quiz/1/question/2"
    return get(url).text


def test_question_post_request():
    url = "http://127.0.0.1:8888/api/quiz/1/question/2"

    question_post_request_body = {"answers": ["Incorrect answer", "Correct answer"]}
    return post(url, data=json.dumps(question_post_request_body), headers={"Content-Type": "application/json"}).text


print(test_quizzes_post_request())
# print(test_quiz_get_request())
# print(test_user_login_request())
# print(test_question_get_request())
# print(test_user_get_request())

# print(str([{"Hello": True, "World": False}]))
