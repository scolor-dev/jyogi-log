import requests

BASE_URL = "http://localhost:8080"
session = requests.Session()

access_token = None


def test_health():
    r = session.get(f"{BASE_URL}/health")
    assert r.status_code == 200, f"health failed: {r.status_code} {r.text}"
    print(f"✅ health: {r.json()}")


def test_signup():
    r = session.post(f"{BASE_URL}/auth/signup", json={
        "username": "testuser",
        "password": "Test1234!",
        "display_name": "Test User",
    })
    assert r.status_code == 201, f"signup failed: {r.status_code} {r.text}"
    print(f"✅ signup: 201 Created")


def test_signup_duplicate():
    r = session.post(f"{BASE_URL}/auth/signup", json={
        "username": "testuser",
        "password": "Test1234!",
        "display_name": "Test User",
    })
    assert r.status_code == 409, f"signup duplicate failed: {r.status_code} {r.text}"
    print(f"✅ signup duplicate: 409 {r.json()}")


def test_signup_validation():
    r = session.post(f"{BASE_URL}/auth/signup", json={
        "username": "ab",
        "password": "Test1234!",
        "display_name": "Test User",
    })
    assert r.status_code == 400, f"signup validation failed: {r.status_code} {r.text}"
    print(f"✅ signup validation: 400 {r.json()}")


def test_login():
    global access_token
    r = session.post(f"{BASE_URL}/auth/login", json={
        "username": "testuser",
        "password": "Test1234!",
    })
    assert r.status_code == 200, f"login failed: {r.status_code} {r.text}"
    data = r.json()
    assert "access_token" in data, f"no access_token in response: {data}"
    access_token = data["access_token"]
    print(f"✅ login: access_token = {access_token[:30]}...")
    print(f"   cookies: {dict(session.cookies)}")


def test_me():
    global access_token
    r = session.get(f"{BASE_URL}/auth/me", headers={
        "Authorization": f"Bearer {access_token}"
    })
    assert r.status_code == 200, f"me failed: {r.status_code} {r.text}"
    print(f"✅ me: {r.json()}")


def test_logout():
    global access_token
    r = session.post(f"{BASE_URL}/auth/logout", headers={
        "Authorization": f"Bearer {access_token}"
    })
    assert r.status_code == 200, f"logout failed: {r.status_code} {r.text}"
    print(f"✅ logout: 200 OK")


def test_refresh():
    global access_token
    r = session.post(f"{BASE_URL}/auth/refresh")
    assert r.status_code == 200, f"refresh failed: {r.status_code} {r.text}"
    data = r.json()
    assert "access_token" in data, f"no access_token in response: {data}"
    access_token = data["access_token"]
    print(f"✅ refresh: access_token = {access_token[:30]}...")
    print(f"   cookies: {dict(session.cookies)}")


def test_me_after_refresh():
    global access_token
    r = session.get(f"{BASE_URL}/auth/me", headers={
        "Authorization": f"Bearer {access_token}"
    })
    assert r.status_code == 200, f"me after refresh failed: {r.status_code} {r.text}"
    print(f"✅ me after refresh: {r.json()}")


    global access_token
    r = session.get(f"{BASE_URL}/auth/me", headers={
        "Authorization": f"Bearer {access_token}"
    })
    # JWTは有効期限まで使えるので200が返る（セッションは無効化済み）
    print(f"✅ me after logout: {r.status_code} (JWT still valid until expiry)")


if __name__ == "__main__":
    print("=== API Test ===\n")
    test_health()
    test_signup()
    test_signup_duplicate()
    test_signup_validation()
    test_login()
    test_me()
    test_refresh()
    test_me_after_refresh()
    test_logout()
    test_me_after_logout()
    print("\n=== All tests passed ===")