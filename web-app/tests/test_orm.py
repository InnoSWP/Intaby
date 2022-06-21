from adapters import repository
from domain import model


def test_add_user(repo):
    new_user = model.User("Fedor", "fedya@gg.ru", "CoolPass")

    repo.add_user(new_user)
    db_user = repo.session.query(model.User).filter_by(nickname=new_user.nickname).one()

    assert db_user == new_user

    repo.session.query(model.User).filter_by(nickname=new_user.nickname).delete()
    repo.session.commit()

    print("OK")
